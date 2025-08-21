// src/api/device_buffer/ready/mod.rs

use crate::buffer::state::{Ready, InFlight};
use crate::error::Result;
use crate::EventToken;
use crate::api::{DeviceBuffer, Queue, Kernel};
use std::marker::PhantomData;

// Import I/O implementations
mod io;

//=============================================================================
// COMPUTE OPERATIONS
//=============================================================================

impl<'ctx, T> DeviceBuffer<'ctx, T, Ready> {
    pub fn enqueue_kernel<'q>(
        self,
        queue: &'q Queue,
        kernel: &Kernel<'q>,
        global_work_size: usize,
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)> {
        let (inner_inflight, evt) =
            self.inner
                .enqueue_kernel(queue.raw(), kernel.raw(), global_work_size)?;

        Ok((
            DeviceBuffer {
                inner: inner_inflight,
                len: self.len,
                _marker: PhantomData,
            },
            EventToken::from_event(evt),
        ))
    }
}