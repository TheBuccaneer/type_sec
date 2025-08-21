// src/api/inflight.rs

use crate::buffer::state::{InFlight, Ready};
use super::DeviceBuffer;
use std::marker::PhantomData;

//=============================================================================
// INFLIGHT STATE IMPLEMENTATIONS  
//=============================================================================

impl<'ctx, T> DeviceBuffer<'ctx, T, InFlight> {
    pub fn into_ready(self) -> DeviceBuffer<'ctx, T, Ready> {
        DeviceBuffer::from_inner(
            crate::buffer::GpuBuffer {
                buf: self.inner.buf,
                len_bytes: self.inner.len_bytes,
                _state: PhantomData::<Ready>,
            },
            self.len,
        )
    }
}
