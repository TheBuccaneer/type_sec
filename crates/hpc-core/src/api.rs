use crate::buffer::{GpuBuffer, state::{State, Empty, Ready, InFlight}};
use std::marker::PhantomData;
use std::ptr;
use opencl3::kernel::Kernel as CLKernel;
use opencl3::program::Program as CLProgram;
use opencl3::{context::Context as CLContext, types::cl_device_id, device::CL_DEVICE_TYPE_GPU};
use opencl3::{types::cl_context_properties};
use opencl3::device::get_all_devices;
use opencl3::platform::get_platforms;
use opencl3::command_queue::CommandQueue as CLQueue;
use super::EventToken;

pub use crate::error::{Error, Result};
// Alias öffentlich ins Root reexportieren

/// High-Level typisierter Buffer. Delegiert an den GPUBuffer
#[derive(Debug)]
pub struct DeviceBuffer<'ctx, T, S: State> {
    pub inner: GpuBuffer<S>,
    pub len: usize, //Anzahl Elemente
    pub _marker: std::marker::PhantomData<&'ctx T>,
}


#[must_use]
#[derive(Debug)]
pub struct Context {
    inner: CLContext,
    device: cl_device_id,
}

#[must_use]
#[derive(Debug)]
pub struct Queue {
    inner: CLQueue,
}

#[must_use]
#[derive(Debug)]
pub struct Kernel<'q> {
    inner: CLKernel,
    program: CLProgram,
    _marker: std::marker::PhantomData<&'q ()>, // Kernel hängt an Context/Queue
}


//#####################OPEN CL PUBLIC################################




/*
Erstellt einen neuen Buffer auf Nutzerebene!
 */
pub fn create_buffer<T>(ctx: &Context, n_elems: usize) -> Result<DeviceBuffer<'static, T, Empty>> {
    let inner: GpuBuffer<Empty> = GpuBuffer::<Empty>::create_uninit_elems::<T>(ctx.raw(), n_elems)?;
    Ok(DeviceBuffer::from_inner(inner, n_elems))
}

/*
Hilfsfunktion, die intern einfach das neue Objekt weitergibt */
impl<'ctx, T, S: State> DeviceBuffer<'ctx, T, S> {
    pub(crate) fn from_inner(inner: GpuBuffer<S>, len_elems: usize) -> Self {
        Self {
            inner,               // der u8-Buffer
            len: len_elems,      // Element-Länge für Nutzer
            _marker: PhantomData,
        }
    }
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
        Ok((Self { inner: ctx, device: devices[0] }, Queue { inner: q }))
    }

    /// Low‑Level‑Zugriff (nur wenn unbedingt nötig)
    pub fn raw(&self) -> &CLContext {
        &self.inner
    }

    pub fn device_id(&self) -> cl_device_id {
        self.device
    }
}


impl<'q> Kernel<'q> {
    pub fn from_source(ctx: &Context, src: &str, name: &str) -> Result<Self> {
        let program = CLProgram::create_and_build_from_source(ctx.raw(), src, "")?;
        let inner = CLKernel::create(&program, name)?;
        Ok(Self { inner, program, _marker: std::marker::PhantomData })
    }

    pub fn raw(&self) -> &CLKernel {
        &self.inner
    }
}



//####################################INTERN#############################################

impl<'ctx, T> DeviceBuffer<'ctx, T, Empty> {
    pub fn enqueue_write(
        self,
        queue: &Queue,
        data: &[T],
    ) -> Result<DeviceBuffer<'ctx, T, Ready>>
    where
        T: bytemuck::Pod,
    {
        if data.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: data.len(),
            });
        }

        // Cast &[T] → &[u8]
        let bytes: &[u8] = bytemuck::cast_slice(data);

        let (inner_ready, _evt) = self.inner.enqueue_write(queue.raw(), bytes)?;
        Ok(DeviceBuffer::from_inner(inner_ready, self.len))
    }
}


impl<'ctx, T> DeviceBuffer<'ctx, T, Ready> {
 pub fn enqueue_read(&self, queue: &Queue, out: &mut [T]) -> Result<()>
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

            queue.raw().enqueue_read_buffer(&self.inner.buf, 
                                            opencl3::types::CL_BLOCKING, 
                                            0, 
                                            bytes, 
                                            &[])?;
        

        Ok(())
    }

    pub fn overwrite(
        &mut self,
        queue: &Queue,
        data: &[T],
    ) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        if data.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: data.len(),
            });
        }

        let bytes: &[u8] = bytemuck::cast_slice(data);

            queue.raw().enqueue_write_buffer(
                &mut self.inner.buf,
                opencl3::types::CL_BLOCKING, // Blockierend für Einfachheit
                0,
                bytes,
                &[],
            )?;

        Ok(())
    }


    pub fn overwrite_byte(&mut self, queue: &Queue, data: &[u8]) -> Result<()> {
        if data.len() != self.len * std::mem::size_of::<T>() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * std::mem::size_of::<T>(),
                actual: data.len(),
            });
        }
        self.inner.overwrite_byte(queue.raw(), data)
    }

     pub fn enqueue_kernel<'q>(
        self,
        queue: &'q Queue,
        kernel: &Kernel<'q>,
        global_work_size: usize,
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)> {
        let (inner_inflight, evt) =
            self.inner.enqueue_kernel(queue.raw(), kernel.raw(), global_work_size)?;

        Ok((
            DeviceBuffer {
                inner: inner_inflight,
                len: self.len,
                _marker: PhantomData,
            },
            //TO_DO - IMPLEMENT ID
            EventToken::new(evt), // dein Token-Wrapper
        ))
    }
}

impl Queue {
    pub fn raw(&self) -> &CLQueue {
        &self.inner
    }

    // ✅ Das hinzufügen:
    pub fn wait<T, S: State>(
        &self,
        event_token: EventToken,  // Dein EventToken aus ready.rs
        buf: DeviceBuffer<'_, T, InFlight>,
    ) -> DeviceBuffer<'_, T, Ready> {
        // Event aus Token extrahieren und warten
        let event = event_token.into_event();  // Oder wie auch immer dein Token funktioniert
        
        // Auf Event warten
        event.wait().expect("Failed to wait for event");
        
        // Buffer-State von InFlight -> Ready
        DeviceBuffer {
            inner: GpuBuffer {
                buf: buf.inner.buf,
                len_bytes: buf.inner.len_bytes,
                _state: PhantomData::<Ready>,
            },
            len: buf.len,
            _marker: PhantomData,
        }
    }
}