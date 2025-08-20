// src/api/event_token.rs
use opencl3::event::Event;
use crate::buffer::{GpuBuffer, state::{Ready, InFlight}};
use crate::api::DeviceBuffer;
use std::marker::PhantomData;



#[must_use]
pub struct EventToken<'q> {
    evt: Event,
    _marker: PhantomData<&'q ()>,
}

impl<'q> EventToken<'q> {
    pub(crate) fn new(evt: Event) -> Self {
        Self { evt, _marker: PhantomData }
    }

    pub fn wait<T>(
        self,
        buf: DeviceBuffer<'q, T, InFlight>,
    ) -> crate::Result<DeviceBuffer<'q, T, Ready>> {
        let inner_ready: GpuBuffer<Ready> = buf.inner.wait(self.evt);
       Ok(DeviceBuffer::from_inner(inner_ready, buf.len))  // sauberer Übergang
    }
}