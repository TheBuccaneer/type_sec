use crate::buffer::state::{Empty, Ready};
use crate::buffer::{GpuBuffer, GpuEventGuard};
use crate::error::{Error, Result};
use core::mem::size_of;
use opencl3::command_queue::CommandQueue;
use opencl3::memory::{CL_MEM_READ_WRITE};
use opencl3::types::CL_BLOCKING;
use opencl3::{context::Context, memory::Buffer, types::cl_mem_flags};
use std::marker::PhantomData;

impl GpuBuffer<Empty> {
    pub fn create_uninit_elems<T>(ctx: &Context, n_elems: usize) -> Result<Self> {
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
}

impl GpuBuffer<Empty> {
    pub fn enqueue_write(
        mut self,
        queue: &CommandQueue,
        host: &[u8],
    ) -> Result<(GpuBuffer<Ready> /* , GpuEventGuard*/)> {
        // Größenprüfung
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        // Write enqueuen
        let evt = queue.enqueue_write_buffer(&mut self.buf, CL_BLOCKING, 0, host, &[])?;

        Ok(
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<Ready>,
            }//,
            //GpuEventGuard::new(evt),
        )
    }
}
