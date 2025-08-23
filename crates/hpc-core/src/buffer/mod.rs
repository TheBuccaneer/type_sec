//! GPU Buffer management with type-state pattern

mod gpu_guard;
mod map_guard;

pub use gpu_guard::GpuEventGuard;
pub use map_guard::MapGuard;
pub mod state;
pub use state::{InFlight, Mapped, State};

mod empty;
mod inflight;
mod written;

use opencl3::memory::Buffer;

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

    #[inline]
    pub fn dev_len_bytes(&self) -> usize {
        self.len_bytes()
    }
}
