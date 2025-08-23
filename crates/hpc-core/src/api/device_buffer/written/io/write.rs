// src/api/device_buffer/ready/io/write.rs

use crate::EventToken;
use crate::api::{DeviceBuffer, Queue};
use crate::buffer::state::{InFlight, Written, Mapped};
use crate::error::Result;
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};
use crate::api::util::MapToken;
use crate::error::{Error};

#[cfg(feature = "memtracer")]
use crate::memtracer::{Dir, start};

//=============================================================================
// WRITE OPERATIONS
//=============================================================================

impl<'brand, T> DeviceBuffer<'brand, T, Written> {
    pub fn write_blocking(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<DeviceBuffer<'brand, T, Written>>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let inner_written = self.inner.write_block(queue.raw(), bytes)?;
        Ok(DeviceBuffer::from_inner(inner_written, self.len))
    }

    pub fn overwrite_blocking_for_bench(&mut self, queue: &Queue<'brand>, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_BLOCKING)?;
        Ok(())
    }

    pub fn write_non_block(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<(DeviceBuffer<'brand, T, InFlight>, EventToken<'brand>)>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let (inner_inflight, evt) = self.inner.write_non_block(queue.raw(), bytes)?;
        //

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

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


    
}
