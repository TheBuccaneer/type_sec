//! GPU Buffer management with type-state pattern

mod guard;

pub use guard::GpuEventGuard;
pub mod state;
pub use state::{InFlight, Ready, State};

mod empty;
mod ready;
mod inflight;


use opencl3::memory::Buffer;


#[cfg(feature = "metrics")]
use std::time::Instant;

#[cfg(feature = "metrics")]
use std::sync::atomic::Ordering;


#[cfg(feature = "metrics")]
use crate::metrics::{RunLog, log_run};

#[cfg(feature = "metrics")]
#[inline]
fn mlog(example: &'static str, n: usize) {
    // eine JSONL-Zeile, Fehler bewusst ignorieren (keine Panik im Fast-Path)
    let _ = log_run(&RunLog { example, n });
}

#[derive(Debug)]
pub struct GpuBuffer<S: State> {
    pub buf: Buffer<u8>,
    pub len_bytes: usize,
    pub _state: core::marker::PhantomData<S>,
}




// Common methods for all states
impl<S: State> GpuBuffer<S> {
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn assume_state<Target: state::State>(self) -> GpuBuffer<Target> {
        unsafe { core::mem::transmute(self) }
    }

    #[inline]
    pub fn raw(&self) -> &Buffer<u8> {
        &self.buf
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut Buffer<u8> {
        &mut self.buf
    }

    /// Anzahl der Bytes im Gerätepuffer (kanonisch)
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.len_bytes
    }


}



impl<S: State> GpuBuffer<S> {
    #[inline]
    pub fn dev_len_bytes(&self) -> usize {
        self.len_bytes()
    }
}
