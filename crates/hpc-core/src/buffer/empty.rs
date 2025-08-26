//! Operations for `DeviceBuffer<T, Empty>`.
//!
//! Represents a freshly allocated but uninitialized device buffer.

use crate::buffer::GpuBuffer;
use crate::buffer::MapGuard;
use crate::buffer::state::{Empty, Mapped, Written};
use crate::error::{Error, Result};
use core::mem::size_of;
use opencl3::command_queue::CommandQueue;
use opencl3::memory::CL_MEM_READ_WRITE;
use opencl3::memory::ClMem; // <-- Das fehlt!
use opencl3::types::CL_BLOCKING;
use opencl3::{context::Context, memory::Buffer, types::cl_mem_flags};
use std::marker::PhantomData;

impl GpuBuffer<Empty> {
    pub fn create_empty_buffer<T>(ctx: &Context, n_elems: usize) -> Result<Self> {
        let n_bytes = n_elems
            .checked_mul(size_of::<T>())
            .ok_or_else(|| Error::AllocationFailed("size overflow".into()))?;

        let cl_buf = Buffer::<u8>::create(
            ctx,
            CL_MEM_READ_WRITE as cl_mem_flags,
            n_bytes,
            core::ptr::null_mut(),
        )
        .map_err(Error::from)?;

        Ok(Self {
            buf: cl_buf,
            len_bytes: n_bytes,
            _state: core::marker::PhantomData,
        })
    }

    /// works also with other datatypes bc of enqueue_write_buffer, but needs to be
    /// TODO: transfered into generic version
    pub fn write_block(mut self, queue: &CommandQueue, host: &[u8]) -> Result<GpuBuffer<Written>> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let _evt = queue.enqueue_write_buffer(&mut self.buf, CL_BLOCKING, 0, host, &[])?;

        Ok(GpuBuffer {
            buf: self.buf,
            len_bytes: self.len_bytes,
            _state: PhantomData::<Written>,
        })
    }

    /// Maps the buffer on the host side → Mapped.
    /// Returns a guard that gives you `&mut [T]`.

    pub fn map_for_write_block(
        mut self,
        queue: &CommandQueue,
    ) -> Result<(GpuBuffer<Mapped>, MapGuard<'_>)> {
        let mut mapped_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        let _event = queue.enqueue_map_buffer(
            &mut self.buf,
            opencl3::types::CL_TRUE, // blocking
            CL_MEM_READ_WRITE,
            0,
            self.len_bytes,
            &mut mapped_ptr,
            &[],
        )?;

        let guard = MapGuard::new(queue, self.buf.get(), mapped_ptr as *mut u8);

        Ok((
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<Mapped>,
            },
            guard, // Für späteren unmap
        ))
    }
}
