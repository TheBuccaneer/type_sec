// src/api/device_buffer/ready/io/read.rs

use crate::api::util::{EventToken, ReadGuard};
use crate::api::{DeviceBuffer, Queue};
use crate::buffer::state::{InFlight, Ready};
use crate::error::{Error, Result};

impl<'brand, T> DeviceBuffer<'brand, T, Ready> {
    //############################READING FUNCTIONS

    pub fn enqueue_read_blocking(&self, queue: &Queue<'brand>, out: &mut [T]) -> Result<()>
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

    pub fn enqueue_read_non_blocking<'a>(
        self,
        queue: &Queue<'brand>,
        out: &'a mut [T],
    ) -> Result<(DeviceBuffer<'brand, T, InFlight>, ReadGuard<'a, 'brand, T>)>
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
            opencl3::types::CL_NON_BLOCKING,
        )?;

        let token = EventToken::from_event(evt);
        let guard = ReadGuard::new(out, token);

        Ok((DeviceBuffer::from_inner(inner_inflight, self.len), guard))
    }
}
