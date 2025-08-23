// src/api/empty.rs

use super::DeviceBuffer;
use crate::api::util::MapToken;
use crate::buffer::state::{Empty, Mapped, Written};
use crate::error::{Error, Result};

use crate::api::Queue;

impl<'brand, T> DeviceBuffer<'brand, T, Empty> {
    pub fn map_for_write_block(
        self,
        queue: &'brand Queue<'brand>, // <-- 'brand explizit hinzufügen
    ) -> Result<(DeviceBuffer<'brand, T, Mapped>, MapToken<'brand>)>
    where
        T: bytemuck::Pod,
    {
        if self.len * size_of::<T>() != self.inner.len_bytes() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * size_of::<T>(),
                actual: self.inner.len_bytes(),
            });
        }

        // Delegation - gibt (GpuBuffer<Mapped>, MapGuard) zurück
        let (inner_mapped, map_guard) = self.inner.map_for_write_block(queue.raw())?;

        // MapGuard in MapToken wrappen
        let map_token = MapToken::new(map_guard); // Einfach MapGuard übergeben

        Ok((DeviceBuffer::from_inner(inner_mapped, self.len), map_token))
    }

    pub fn write_block(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<DeviceBuffer<'brand, T, Written>>
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

        let inner_ready /*_evt*/ = self.inner.write_block(queue.raw(), bytes)?;
        Ok(DeviceBuffer::from_inner(inner_ready, self.len))
    }
}
