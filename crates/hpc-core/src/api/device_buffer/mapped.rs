//! Operations for `DeviceBuffer<T, Mapped>`
//! Represents a buffer region that has been mapped into host address space
//! via clEnqueueMapBuffer. While in this state, the buffer must not be
//! accessed by the device.

use crate::api::DeviceBuffer;
use crate::api::util::MapToken;
use crate::buffer::state::Mapped;
use crate::error::{Error, Result};
//#####
// MAPPED STATE IMPLEMENTATIONS
//#####

impl<'brand, T> DeviceBuffer<'brand, T, Mapped> {
    /// Write data directly to the mapped memory (blocking)
    pub fn write_blocking(&mut self, data: &[T], token: &mut MapToken<'brand>) -> Result<()>
    where
        T: bytemuck::Pod + Copy,
    {
        if data.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: data.len(),
            });
        }

        let ptr = token.as_mut_ptr();
        if ptr.is_null() {
            return Err(Error::Msg("null pointer in MapToken".into()));
        }

        // Cast to typed pointer and copy data
        unsafe {
            let typed_ptr = ptr as *mut T;
            let mapped_slice = std::slice::from_raw_parts_mut(typed_ptr, self.len);
            mapped_slice.copy_from_slice(data);
        }

        Ok(())
    }

    /// Read data from the mapped memory (blocking)
    pub fn read_blocking(&self, output: &mut [T], token: &MapToken<'brand>) -> Result<()>
    where
        T: bytemuck::Pod + Copy,
    {
        if output.len() != self.len {
            return Err(Error::BufferSizeMismatch {
                expected: self.len,
                actual: output.len(),
            });
        }

        let ptr = token.as_ptr();
        if ptr.is_null() {
            return Err(Error::Msg("null pointer in MapToken".into()));
        }

        unsafe {
            let typed_ptr = ptr as *const T;
            let mapped_slice = std::slice::from_raw_parts(typed_ptr, self.len);
            output.copy_from_slice(mapped_slice);
        }

        Ok(())
    }
}
