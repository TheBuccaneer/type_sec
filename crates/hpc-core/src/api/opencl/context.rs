// src/api/context.rs

use opencl3::{
    command_queue::CommandQueue as CLQueue,
    context::Context as CLContext,
    device::{CL_DEVICE_TYPE_GPU, get_all_devices},
    platform::get_platforms,
    types::{cl_context_properties, cl_device_id},
};

use crate::api::DeviceBuffer;
use crate::api::Queue;
use crate::buffer::state::Empty;
use crate::error::Result;
use std::marker::PhantomData;
use std::ptr;

//#####
// CONTEXT
//#####

#[must_use]
#[derive(Debug)]
pub struct Context<'brand> {
    inner: CLContext,
    device: cl_device_id,
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand> Context<'brand> {
    /// Build an OpenCL context for the first found GPU device + matching queue.
    /*
    possible error: Assumption: 1 platform with 1 device -> NVIDIA
    */
    pub fn create_context() -> Result<Self> {
        let _platforms = get_platforms()?; // aktuell ungenutzt, später ggf. filtern
        let devices: Vec<cl_device_id> = get_all_devices(CL_DEVICE_TYPE_GPU)?;
        let props: &[cl_context_properties] = &[];
        let ctx = CLContext::from_devices(&devices, props, None, ptr::null_mut())?;

        Ok(Self {
            inner: ctx,
            device: devices[0],
            _brand: PhantomData,
        })
    }

    pub fn create_queue(&'brand self) -> Result<Queue<'brand>> {
        let q = CLQueue::create(&self.inner, self.device, 0)?;
        Ok(Queue {
            inner: q,
            _brand: PhantomData,
        })
    }

    pub fn create_empty_buffer<T>(
        &'brand self,
        n_elems: usize,
    ) -> Result<DeviceBuffer<'brand, T, Empty>> {
        // Delegate to the existing create_buffer function, but with branded DeviceBuffer
        let inner: crate::buffer::GpuBuffer<Empty> =
            crate::buffer::GpuBuffer::<Empty>::create_empty_buffer::<T>(&self.inner, n_elems)?;

        Ok(DeviceBuffer::from_inner(inner, n_elems))
    }

    /// Low-level access (only when absolutely necessary)
    pub fn raw(&self) -> &CLContext {
        &self.inner
    }

    pub fn device_id(&self) -> cl_device_id {
        self.device
    }
}
