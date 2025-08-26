//! IMPLEMENATION OPSOLETE


use super::GpuBuffer;
use crate::buffer::state::{InFlight, Ready};
use crate::error::{Error, Result};
use opencl3::command_queue::CommandQueue;
//use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING};
use opencl3::event::Event;
use opencl3::types::cl_bool;
use std::ffi::c_void;
use std::marker::PhantomData;

impl GpuBuffer<Ready> {
    /*
    #######################################
    #######################################
    #######################################
    #################READ##################
     */
    pub fn enqueue_read(
        &self,
        queue: &CommandQueue,
        host: &mut [u8],
        blocking: cl_bool,
    ) -> Result<Event> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let evt = queue.enqueue_read_buffer(&self.buf, blocking, 0, host, &[])?;

        Ok(evt)
    }

    pub fn enqueue_read_consuming(
        self,
        queue: &CommandQueue,
        host: &mut [u8],
        blocking: cl_bool,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let evt = queue.enqueue_read_buffer(&self.buf, blocking, 0, host, &[])?;

        Ok((
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }

    /*
    #######################################
    #######################################
    #######################################
    #################WRITE#################
     */

    pub fn overwrite(
        &mut self,
        queue: &CommandQueue,
        host: &[u8],
        blocking: cl_bool,
    ) -> Result<Event> {
        if host.len() != self.len_bytes {
            return Err(Error::BufferSizeMismatch {
                expected: self.len_bytes,
                actual: host.len(),
            });
        }

        let evt = queue.enqueue_write_buffer(&mut self.buf, blocking, 0, host, &[])?;

        Ok(evt)
    }

    pub fn overwrite_consuming(
        mut self, // Konsumiert self
        queue: &CommandQueue,
        host: &[u8],
        blocking: cl_bool,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        let evt = queue.enqueue_write_buffer(&mut self.buf, blocking, 0, host, &[])?;
        Ok((
            GpuBuffer {
                buf: self.buf, // Move ist OK, da self konsumiert wird
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }

    pub fn enqueue_kernel(
        self,
        queue: &CommandQueue,
        kernel: &opencl3::kernel::Kernel,
        global_work_size: usize,
    ) -> Result<(GpuBuffer<InFlight>, Event)> {
        let kernel_ptr = kernel.get() as *mut c_void;

        let evt = queue.enqueue_nd_range_kernel(
            kernel_ptr,
            1, // 1D NDRange (später anpassen)
            std::ptr::null(),
            &global_work_size as *const usize,
            std::ptr::null(),
            &[],
        )?;

        Ok((
            GpuBuffer {
                buf: self.buf,
                len_bytes: self.len_bytes,
                _state: PhantomData::<InFlight>,
            },
            evt,
        ))
    }
}
