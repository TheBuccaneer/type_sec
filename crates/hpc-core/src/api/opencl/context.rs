// src/api/context.rs

use opencl3::{
    command_queue::CommandQueue as CLQueue,
    context::Context as CLContext,
    device::{get_all_devices, CL_DEVICE_TYPE_GPU},
    platform::get_platforms,
    types::{cl_context_properties, cl_device_id},
};

use crate::api::opencl::Queue; 
use std::ptr;

use crate::error::Result;

//=============================================================================
// CONTEXT
//=============================================================================

#[must_use]
#[derive(Debug)]
pub struct Context {
    inner: CLContext,
    device: cl_device_id,
}


impl Context {
    /// Baue einen OpenCL-Context für das erste gefundene GPU-Device + passende Queue.
    /*
    possible error: Assumption: 1 platform with 1 device -> NVIDIA
    */
    pub fn new_first_gpu() -> Result<(Self, Queue)> {
        let _platforms = get_platforms()?; // aktuell ungenutzt, später ggf. filtern
        let devices: Vec<cl_device_id> = get_all_devices(CL_DEVICE_TYPE_GPU)?;
        let props: &[cl_context_properties] = &[];
        let ctx = CLContext::from_devices(&devices, props, None, ptr::null_mut())?;
        let q = CLQueue::create(&ctx, devices[0], 0)?;
        Ok((
            Self {
                inner: ctx,
                device: devices[0],
            },
            Queue { inner: q },
        ))
    }

    /// Low‑Level‑Zugriff (nur wenn unbedingt nötig)
    pub fn raw(&self) -> &CLContext {
        &self.inner
    }

    pub fn device_id(&self) -> cl_device_id {
        self.device
    }
}
