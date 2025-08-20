// src/api/event_token.rs
use opencl3::event::Event;
use crate::buffer::{GpuBuffer, state::{Ready, InFlight}};
use crate::api::DeviceBuffer;
use std::marker::PhantomData;

#[must_use]
pub struct EventToken<'q> {
    evt: Event,
    _marker: std::marker::PhantomData<&'q ()>,
}

impl<'q> EventToken<'q> {
    pub fn new(evt: Event) -> Self {  // ← Underscore vor queue_id
        Self { 
            evt, 
            _marker: std::marker::PhantomData 
        }
    }
    
    pub fn wait<T>(self, buf: DeviceBuffer<'_, T, InFlight>)
        -> crate::error::Result<DeviceBuffer<'_, T, Ready>>  // ← Vollständiger Pfad
    {
        self.evt.wait()?; // blockiert, bis fertig
        
        Ok(DeviceBuffer {
            inner: GpuBuffer {
                buf: buf.inner.buf,
                len_bytes: buf.inner.len_bytes,
                _state: PhantomData::<Ready>,
            },
            len: buf.len,
            _marker: PhantomData,
        })
    }
}