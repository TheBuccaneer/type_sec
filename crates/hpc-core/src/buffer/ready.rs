use super::GpuBuffer;
use crate::buffer::state::{InFlight, Ready};
use crate::error::{Error, Result};
use opencl3::command_queue::CommandQueue;
//use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};
use opencl3::event::Event;
use opencl3::types::cl_bool;
use std::ffi::c_void;
use std::marker::PhantomData;

#[cfg(feature = "memtracer")]
use crate::memtracer::{Dir, start};

impl GpuBuffer<Ready> {
    /*
    #######################################
    #######################################
    #######################################
    #################READ##################
     */
    pub fn enqueue_read(
        &self,
        queue: &CommandQueue,
        host: &mut [u8],
        blocking: cl_bool,
    ) -> Result<Event> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let evt = queue.enqueue_read_buffer(&self.buf, blocking, 0, host, &[])?;

        Ok(evt)
    }

    pub fn enqueue_read_consuming(
        self,
        queue: &CommandQueue,
        host: &mut [u8],
        blocking: cl_bool,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let evt = queue.enqueue_read_buffer(&self.buf, blocking, 0, host, &[])?;

        Ok((
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }

    /*
    #######################################
    #######################################
    #######################################
    #################WRITE#################
     */

    pub fn overwrite(
        &mut self,
        queue: &CommandQueue,
        host: &[u8],
        blocking: cl_bool,
    ) -> Result<Event> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }
        #[cfg(feature = "memtracer")]
        let token_box = if crate::memtracer::is_auto_trace_enabled() {
            Some(Box::new(crate::memtracer::start(
                crate::memtracer::Dir::H2D,
                host.len(),
            )))
        } else {
            None
        };

        let evt = queue.enqueue_write_buffer(&mut self.buf, blocking, 0, host, &[])?;

        #[cfg(feature = "memtracer")]
        if let Some(token_box) = token_box {
            use opencl3::event::CL_COMPLETE;
            let ptr = Box::into_raw(token_box) as *mut std::ffi::c_void;
            if let Err(e) = evt.set_callback(CL_COMPLETE, crate::memtrace_callback, ptr) {
                eprintln!("callback failed: {e}");
                unsafe { Box::from_raw(ptr.cast::<crate::memtracer::CopyToken>()) }.finish();
            }
        }

        Ok(evt)
    }

    pub fn overwrite_consuming(
        mut self, // Konsumiert self
        queue: &CommandQueue,
        host: &[u8],
        blocking: cl_bool,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        let evt = queue.enqueue_write_buffer(&mut self.buf, blocking, 0, host, &[])?;
        Ok((
            GpuBuffer {
                buf: self.buf, // Move ist OK, da self konsumiert wird
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }

    pub fn enqueue_kernel(
        self,
        queue: &CommandQueue,
        kernel: &opencl3::kernel::Kernel,
        global_work_size: usize,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        let kernel_ptr = kernel.get() as *mut c_void;

        let evt = queue.enqueue_nd_range_kernel(
            kernel_ptr,
            1, // 1D NDRange (später anpassen)
            std::ptr::null(),
            &global_work_size as *const usize,
            std::ptr::null(),
            &[],
        )?;

        Ok((
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }
}
