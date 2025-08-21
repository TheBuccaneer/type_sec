// src/api/device_buffer/ready/io/write.rs

use crate::buffer::state::{Ready, InFlight};
use crate::error::{Error, Result};
use crate::EventToken;
use crate::api::{DeviceBuffer, Queue};
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};

//=============================================================================
// WRITE OPERATIONS
//=============================================================================

impl<'ctx, T> DeviceBuffer<'ctx, T, Ready> {
    pub fn overwrite_blocking(&mut self, queue: &Queue, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_BLOCKING)?;
        Ok(())
    }

        pub fn overwrite_non_blocking<'q>(
        self,
        queue: &'q Queue,
        data: &[T],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)>
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

    pub fn overwrite_byte_non_blocking<'q>(
        self,
        queue: &'q Queue,
        data: &[u8],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)> {
        if data.len() != self.len * std::mem::size_of::<T>() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * std::mem::size_of::<T>(),
                actual: data.len(),
            });
        }
        let (inner_inflight, evt) = self.inner.overwrite_byte_consuming(
            queue.raw(),
            data, // ← direkt data, ohne cast
            CL_NON_BLOCKING,
        )?;

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

    pub fn benchmark_overwrite_non_blocking(&mut self, queue: &Queue, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_NON_BLOCKING)?;

        Ok(())
    }

    

    pub fn overwrite_byte_blocking(&mut self, queue: &Queue, data: &[u8]) -> Result<()> {
        if data.len() != self.len * std::mem::size_of::<T>() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * std::mem::size_of::<T>(),
                actual: data.len(),
            });
        }
        self.inner.overwrite_byte(queue.raw(), data, CL_BLOCKING)?;
        Ok(())
    }
}