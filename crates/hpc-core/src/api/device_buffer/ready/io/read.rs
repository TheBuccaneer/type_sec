// src/api/device_buffer/ready/io/read.rs

use crate::buffer::state::{Ready, InFlight};
use crate::error::{Error, Result};
use crate::api::{DeviceBuffer, Queue};
use crate::api::util::{EventToken, ReadGuard};


impl<'ctx, T> DeviceBuffer<'ctx, T, Ready> {
    //############################READING FUNCTIONS

    pub fn enqueue_read_blocking(&self, queue: &Queue, out: &mut [T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        if out.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: out.len(),
            });
        }

        let bytes: &mut [u8] = bytemuck::cast_slice_mut(out);

        self.inner
            .enqueue_read(queue.raw(), bytes, opencl3::types::CL_BLOCKING)?;

        Ok(())
    }

    pub fn enqueue_read_non_blocking<'q, 'a>(
        self,
        queue: &'q Queue,
        out: &'a mut [T],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, ReadGuard<'a, 'q, T>)>
    where
        T: bytemuck::Pod,
    {
        if out.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: out.len(),
            });
        }

        let bytes: &mut [u8] = bytemuck::cast_slice_mut(out);

        let (inner_inflight, evt) = self.inner.enqueue_read_consuming(
            queue.raw(), 
            bytes, 
            opencl3::types::CL_NON_BLOCKING
        )?;
        
        let token = EventToken::from_event(evt);
        let guard = ReadGuard::new(out, token);

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            guard,
        ))
    }
}