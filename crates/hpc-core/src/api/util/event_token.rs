//! Event tokens for synchronizing GPU operations.
//! An EventToken represents the completion of an asynchronous
//! operation (such as buffer writes, reads, or kernel launches).
//! - `#[must_use]`: prevents silent dropping of event tokens.

use crate::api::DeviceBuffer;
use crate::buffer::GpuEventGuard;
use crate::buffer::state::InFlight;
use crate::buffer::state::Written;
use core::marker::PhantomData;
use opencl3::event::Event;

#[must_use = "GPU work is in-flight: call wait(event, buf) to complete it"]
pub struct EventToken<'brand> {
    inner: GpuEventGuard,
    _brand: PhantomData<&'brand ()>,
}

impl<'brand> EventToken<'brand> {
    /// Creates a token directly from a guard (low-level).

    pub(crate) fn from_guard(guard: GpuEventGuard) -> Self {
        Self {
            inner: guard,
            _brand: PhantomData,
        }
    }

    /// Creates a token directly from a raw OpenCL event.
    pub(crate) fn from_event(evt: Event) -> Self {
        Self::from_guard(GpuEventGuard::new(evt))
    }

    /// Consuming transition: only allowed path from InFlight → Ready.
    pub fn wait<T>(
        self,
        buf: DeviceBuffer<'brand, T, InFlight>,
    ) -> DeviceBuffer<'brand, T, Written> {
        self.inner.wait();

        DeviceBuffer::from_inner(
            crate::buffer::GpuBuffer {
                buf: buf.inner.buf,
                len_bytes: buf.inner.len_bytes,
                _state: PhantomData::<Written>,
            },
            buf.len,
        )
    }
}
