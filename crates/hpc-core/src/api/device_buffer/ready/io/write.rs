// src/api/device_buffer/ready/io/write.rs

use crate::EventToken;
use crate::api::{DeviceBuffer, Queue};
use crate::buffer::state::{InFlight, Ready};
use crate::error::Result;
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};

#[cfg(feature = "memtracer")]
use crate::memtracer::{Dir, start};

//=============================================================================
// WRITE OPERATIONS
//=============================================================================

impl<'brand, T> DeviceBuffer<'brand, T, Ready> {
    pub fn overwrite_blocking(&mut self, queue: &Queue<'brand>, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_BLOCKING)?;
        Ok(())
    }

    pub fn overwrite_non_blocking(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<(DeviceBuffer<'brand, T, InFlight>, EventToken<'brand>)>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let (inner_inflight, evt) =
            self.inner
                .overwrite_consuming(queue.raw(), bytes, CL_NON_BLOCKING)?;
        //

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

    pub fn benchmark_overwrite_non_blocking(
        &mut self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_NON_BLOCKING)?;

        Ok(())
    }
}
