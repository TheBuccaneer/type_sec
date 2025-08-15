//! GPU Buffer management with type-state pattern


mod guard;

pub use guard::GpuEventGuard;
pub mod state;
#[allow(unused_imports)]
pub use state::{State, Empty, Ready, InFlight, Queued};

use opencl3::{
    context::Context,
    memory::{Buffer, CL_MEM_READ_WRITE},
    command_queue::CommandQueue,
    event::Event,
    types::CL_NON_BLOCKING,
};
use std::{marker::PhantomData, ptr};
use crate::error::{ClError, Result};

#[cfg(feature = "metrics")]
use std::time::Instant;

#[cfg(feature = "metrics")]
use std::sync::atomic::Ordering;

/// GPU Buffer wrapper with compile-time state checking
pub struct GpuBuffer<S: State> {
    buf: Buffer<u8>,
    len: usize,
    _state: PhantomData<S>,
}

#[cfg(feature = "metrics")]
use crate::metrics::{RunLog, log_run};

#[cfg(feature = "metrics")]
#[inline]
fn mlog(example: &'static str, n: usize) {
    // eine JSONL-Zeile, Fehler bewusst ignorieren (keine Panik im Fast-Path)
    let _ = log_run(&RunLog { example, n });
}

// Queued state implementation
impl GpuBuffer<Queued> {
    /// Create a new GPU buffer
    pub fn new(ctx: &Context, len: usize) -> Result<Self> {
        #[cfg(feature = "metrics")]
        {
            crate::metrics::ALLOCS.fetch_add(1, Ordering::Relaxed);
            crate::metrics::ALLOC_BYTES.fetch_add(len as u64, Ordering::Relaxed);
        }

        #[cfg(feature = "metrics")]
        let t = Instant::now();

        let buf = Buffer::<u8>::create(ctx, CL_MEM_READ_WRITE, len, ptr::null_mut())?;

        #[cfg(feature = "metrics")]
            mlog("buffer.new", len);

        #[cfg(feature = "metrics")]
        crate::metrics::record("GpuBuffer::new", t);

        Ok(Self { 
            buf, 
            len,
            _state: PhantomData 
        })
    }

    /// Enqueue write operation from host to device
    pub fn enqueue_write(
        mut self,
        queue: &CommandQueue,
        host: &[u8],
    ) -> Result<(GpuBuffer<InFlight>, GpuEventGuard)> {
        // Validate buffer size
        if host.len() != self.len {
            return Err(ClError::BufferSizeMismatch {
                expected: self.len,
                actual: host.len(),
            });
        }

        #[cfg(feature = "metrics")]
        let t = Instant::now();

        #[cfg(feature = "memtrace")]
        let token_box = if crate::memtracer::is_auto_trace_enabled() {
            Some(Box::new(crate::memtracer::start(crate::memtracer::Dir::H2D, host.len())))
        } else {
            None
        };

        let evt = queue.enqueue_write_buffer(
            &mut self.buf,
            CL_NON_BLOCKING,
            0,
            host,
            &[],
        )?;

        #[cfg(feature = "memtrace")]
        if let Some(token_box) = token_box {
            use opencl3::event::CL_COMPLETE;
            let ptr = Box::into_raw(token_box) as *mut std::ffi::c_void;
            if let Err(e) = evt.set_callback(CL_COMPLETE, crate::memtrace_callback, ptr) {
                eprintln!("callback failed: {e}");
                unsafe { Box::from_raw(ptr.cast::<crate::memtracer::CopyToken>()) }.finish();
            }
        }

        #[cfg(feature = "metrics")]
        crate::metrics::record("enqueue_write", t);

        #[cfg(feature = "metrics")]
        mlog("pipeline.enqueue_write", self.len);

        Ok((
            GpuBuffer {
                buf: self.buf,
                len: self.len,
                _state: PhantomData::<InFlight>,
            },
            GpuEventGuard::new(evt),
        ))
    }

    /// Launch buffer operation
    pub fn launch(self) -> GpuBuffer<InFlight> {
        #[cfg(feature = "metrics")]
        crate::metrics::record("launch", Instant::now());
        #[cfg(feature = "metrics")]
        mlog("pipeline.launch", self.len);
        
        GpuBuffer { 
            buf: self.buf, 
            len: self.len, 
            _state: PhantomData 
        }
    }
}

// Ready state implementation
impl GpuBuffer<Ready> {
    /// Enqueue read operation from device to host
    pub fn enqueue_read(
        self,
        queue: &CommandQueue,
        host_out: &mut [u8],
    ) -> Result<(GpuBuffer<InFlight>, GpuEventGuard)> {
        if host_out.len() != self.len {
            return Err(ClError::BufferSizeMismatch {
                expected: self.len,
                actual: host_out.len(),
            });
        }

        #[cfg(feature = "metrics")]
        let t = Instant::now();

        #[cfg(feature = "memtrace")]
        let token_box = if crate::memtracer::is_auto_trace_enabled() {
            Some(Box::new(crate::memtracer::start(crate::memtracer::Dir::D2H, host_out.len())))
        } else {
            None
        };

        let evt = queue.enqueue_read_buffer(
            &self.buf,
            CL_NON_BLOCKING,
            0,
            host_out,
            &[],
        )?;

        #[cfg(feature = "metrics")]
mlog("pipeline.enqueue_read", self.len);

        #[cfg(feature = "memtrace")]
        if let Some(token_box) = token_box {
            use opencl3::event::CL_COMPLETE;
            let ptr = Box::into_raw(token_box) as *mut std::ffi::c_void;
            if let Err(e) = evt.set_callback(CL_COMPLETE, crate::memtrace_callback, ptr) {
                eprintln!("callback failed: {e}");
                unsafe { Box::from_raw(ptr.cast::<crate::memtracer::CopyToken>()) }.finish();
            }
        }

        #[cfg(feature = "metrics")]
        crate::metrics::record("enqueue_read", t);

        Ok((
            GpuBuffer {
                buf: self.buf,
                len: self.len,
                _state: PhantomData::<InFlight>,
            },
            GpuEventGuard::new(evt),
        ))
    }
}

// InFlight state implementation
impl GpuBuffer<InFlight> {
    /// Complete operation and transition to Ready
    pub fn complete(self, evt: Event) -> GpuBuffer<Ready> {
        let _g = GpuEventGuard::new(evt);
        
        #[cfg(feature = "metrics")]
        crate::metrics::record("complete", Instant::now());

        #[cfg(feature = "metrics")]
        mlog("pipeline.complete", self.len);
        
        GpuBuffer { 
            buf: self.buf, 
            len: self.len, 
            _state: PhantomData 
        }
    }

    /// Transition to Ready with guard
    pub fn into_ready(self, _g: GpuEventGuard) -> GpuBuffer<Ready> {
        #[cfg(feature = "metrics")]
        crate::metrics::record("into_ready", Instant::now());

        #[cfg(feature = "metrics")]
            mlog("pipeline.ready", self.len);

        GpuBuffer { 
            buf: self.buf, 
            len: self.len, 
            _state: PhantomData 
        }
    }
}

// Common methods for all states
impl<S: State> GpuBuffer<S> {

    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn assume_state<Target: state::State>(self) -> GpuBuffer<Target> {
        unsafe {core::mem::transmute(self)}
    }


    #[inline]
    pub fn raw(&self) -> &Buffer<u8> { &self.buf }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut Buffer<u8> { &mut self.buf }

    /// Anzahl der Bytes im Gerätepuffer (kanonisch)
    #[inline]
    pub fn len_bytes(&self) -> usize { self.len }

    /// Alias für Rückwärtskompatibilität
    #[inline]
    pub fn len(&self) -> usize { self.len_bytes() }

    #[inline]
    pub fn is_empty(&self) -> bool { self.len == 0 }
}


impl GpuBuffer<Empty> {
    pub fn dev_alloc_bytes(_bytes: usize) -> Self {
        unimplemented!("buffer::GpuBuffer<Empty>::dev_alloc_bytes")
    }
}


impl<S: State> GpuBuffer<S> {
    #[inline] pub fn dev_len_bytes(&self) -> usize { self.len_bytes() }
}
