// src/api/device_buffer/ready/mod.rs

use crate::EventToken;
use crate::api::{DeviceBuffer, Kernel, Queue};
use crate::buffer::state::{InFlight, Written};
use crate::error::Result;

// Import I/O implementations
mod io;

//=============================================================================
// COMPUTE OPERATIONS
//=============================================================================

impl<'brand, T> DeviceBuffer<'brand, T, Written> {
    pub fn enqueue_kernel(
        self,
        queue: &'brand Queue,
        kernel: &Kernel<'brand>,
        global_work_size: usize,
    ) -> Result<(DeviceBuffer<'brand, T, InFlight>, EventToken<'brand>)> {
        let (inner_inflight, evt) =
            self.inner
                .enqueue_kernel(queue.raw(), kernel.raw(), global_work_size)?;

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }
}
