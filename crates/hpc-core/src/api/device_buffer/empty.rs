// src/api/empty.rs

use super::DeviceBuffer;
use crate::buffer::state::{Empty, Ready};
use crate::error::{Error, Result};

use crate::api::opencl::Queue;

impl<'brand, T> DeviceBuffer<'brand, T, Empty> {
    pub fn enqueue_write(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<DeviceBuffer<'brand, T, Ready>>
    where
        T: bytemuck::Pod,
    {
        if data.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: data.len(),
            });
        }

        // Cast &[T] → &[u8]
        let bytes: &[u8] = bytemuck::cast_slice(data);

        let (inner_ready, _evt) = self.inner.enqueue_write(queue.raw(), bytes)?;
        Ok(DeviceBuffer::from_inner(inner_ready, self.len))
    }
}
