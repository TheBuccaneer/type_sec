// src/api/inflight.rs

use super::DeviceBuffer;
use crate::buffer::state::{InFlight, Ready};
use std::marker::PhantomData;

//=============================================================================
// INFLIGHT STATE IMPLEMENTATIONS
//=============================================================================

impl<'brand, T> DeviceBuffer<'brand, T, InFlight> {
    pub fn into_ready(self) -> DeviceBuffer<'brand, T, Ready> {
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
