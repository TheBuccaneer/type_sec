use crate::api::device_buffer::DeviceBuffer;
use crate::buffer::MapGuard;
use crate::buffer::state::{Mapped, Written};
#[must_use = "call .unmap(...) with this token to release the mapped buffer"]
pub struct MapToken<'a> {
    map_guard: MapGuard<'a>,
}

impl<'a> MapToken<'a> {
    pub(crate) fn new(map_guard: MapGuard<'a>) -> Self {
        MapToken { map_guard }
    }

    // Neue Methoden hinzufügen:
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.map_guard.ptr
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.map_guard.ptr as *const u8
    }

    pub fn unmap<T>(
        self,
        mapped_buffer: DeviceBuffer<'_, T, Mapped>,
    ) -> crate::error::Result<DeviceBuffer<'_, T, Written>> {
        // Clean up MapGuard (this does the actual OpenCL unmap)
        drop(self.map_guard);

        // Buffer State-Transition
        let inner_written = unsafe { mapped_buffer.inner.assume_state::<Written>() };

        Ok(DeviceBuffer::from_inner(inner_written, mapped_buffer.len))
    }
}
