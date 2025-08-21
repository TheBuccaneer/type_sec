use super::EventToken;
use crate::buffer::{
    GpuBuffer,
    state::{Empty, InFlight, Ready, State},
};
use super::ReadGuard;
use opencl3::command_queue::CommandQueue as CLQueue;
use opencl3::device::get_all_devices;
use opencl3::kernel::Kernel as CLKernel;
use opencl3::platform::get_platforms;
use opencl3::program::Program as CLProgram;
use opencl3::{context::Context as CLContext, device::CL_DEVICE_TYPE_GPU, types::cl_device_id};
use opencl3::{types::CL_BLOCKING, types::CL_NON_BLOCKING, types::cl_context_properties};
use std::marker::PhantomData;
use std::ptr;

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
    _program: CLProgram,
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
            inner,          // der u8-Buffer
            len: len_elems, // Element-Länge für Nutzer
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

impl<'q> Kernel<'q> {
    pub fn from_source(ctx: &Context, src: &str, name: &str) -> Result<Self> {
        let program = CLProgram::create_and_build_from_source(ctx.raw(), src, "")?;
        let inner = CLKernel::create(&program, name)?;
        Ok(Self {
            inner,
            program,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn raw(&self) -> &CLKernel {
        &self.inner
    }
}

//####################################INTERN#############################################

impl<'ctx, T> DeviceBuffer<'ctx, T, Empty> {
    pub fn enqueue_write(self, queue: &Queue, data: &[T]) -> Result<DeviceBuffer<'ctx, T, Ready>>
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
    //############################READING FUNCTIONS

    pub fn enqueue_read_blocking(&self, queue: &Queue, out: &mut [T]) -> Result<()>
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

    pub fn enqueue_read_non_blocking<'q, 'a>(
        self,
        queue: &'q Queue,
        out: &'a mut [T],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, ReadGuard<'a, 'q, T>)>
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
            opencl3::types::CL_NON_BLOCKING
        )?;
        
        let token = EventToken::from_event(evt);
        let guard = ReadGuard::new(out, token);

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            guard,
        ))
    }

    pub fn overwrite_blocking(&mut self, queue: &Queue, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_BLOCKING)?;
        Ok(())
    }

    pub fn overwrite_non_blocking<'q>(
        self,
        queue: &'q Queue,
        data: &[T],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let (inner_inflight, evt) =
            self.inner
                .overwrite_consuming(queue.raw(), bytes, CL_NON_BLOCKING)?;
        //

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

    pub fn overwrite_byte_non_blocking<'q>(
        self,
        queue: &'q Queue,
        data: &[u8],
    ) -> Result<(DeviceBuffer<'ctx, T, InFlight>, EventToken<'q>)> {
        if data.len() != self.len * std::mem::size_of::<T>() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * std::mem::size_of::<T>(),
                actual: data.len(),
            });
        }
        let (inner_inflight, evt) = self.inner.overwrite_byte_consuming(
            queue.raw(),
            data, // ← direkt data, ohne cast
            CL_NON_BLOCKING,
        )?;

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

    pub fn benchmark_overwrite_non_blocking(&mut self, queue: &Queue, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_NON_BLOCKING)?;

        Ok(())
    }

    pub fn overwrite_byte_blocking(&mut self, queue: &Queue, data: &[u8]) -> Result<()> {
        if data.len() != self.len * std::mem::size_of::<T>() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * std::mem::size_of::<T>(),
                actual: data.len(),
            });
        }
        self.inner.overwrite_byte(queue.raw(), data, CL_BLOCKING)?;
        Ok(())
    }

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
            //TO_DO - IMPLEMENT ID
            EventToken::from_event(evt), // dein Token-Wrapper
        ))
    }
}

impl Queue {
    pub fn raw(&self) -> &CLQueue {
        &self.inner
    }
}

impl<'ctx, T> DeviceBuffer<'ctx, T, InFlight> {
    pub fn into_ready(self) -> DeviceBuffer<'ctx, T, Ready> {
        DeviceBuffer::from_inner(
            GpuBuffer {
                buf: self.inner.buf,
                len_bytes: self.inner.len_bytes,
                _state: PhantomData::<Ready>,
            },
            self.len,
        )
    }
}
