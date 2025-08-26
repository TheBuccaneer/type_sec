//! Write operations for DeviceBuffer<T, Written>
//!
//! Provides blocking and non-blocking host write methods.
//! - Blocking: copies data into the buffer and waits for completion.
//! - Non-blocking: enqueues write and returns an `EventToken` for sync.

use crate::EventToken;
use crate::api::util::MapToken;
use crate::api::{DeviceBuffer, Queue};
use crate::buffer::state::{InFlight, Mapped, Written};
use crate::error::{Error, Result};
use opencl3::types::CL_BLOCKING;

//#####
// WRITE OPERATIONS
//#####

impl<'brand, T> DeviceBuffer<'brand, T, Written> {
    ///Performs blocking write for api DeviceBuffer
    pub fn write_blocking(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<DeviceBuffer<'brand, T, Written>>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let inner_written = self.inner.write_block(queue.raw(), bytes)?;
        Ok(DeviceBuffer::from_inner(inner_written, self.len))
    }

    /// This method exists only for Criterion benchmarks:
    /// - Buffers are often allocated once outside of the `b.iter(...)` closure
    ///   to avoid measuring allocation costs.
    /// - Before each iteration, the buffer must be re-initialized with fresh
    ///   host data.
    /// - The operation is blocking to ensure deterministic timing without
    ///   overlapping asynchronous writes.
    ///
    /// Not intended for normal API usage; prefer the standard write methods

    pub fn overwrite_blocking_for_bench(&mut self, queue: &Queue<'brand>, data: &[T]) -> Result<()>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let _evt = self.inner.overwrite(queue.raw(), bytes, CL_BLOCKING)?;
        Ok(())
    }

    /// like write. None blocke alternative.
    pub fn write_non_block(
        self,
        queue: &Queue<'brand>,
        data: &[T],
    ) -> Result<(DeviceBuffer<'brand, T, InFlight>, EventToken<'brand>)>
    where
        T: bytemuck::Pod,
    {
        let bytes = bytemuck::cast_slice(data);
        let (inner_inflight, evt) = self.inner.write_non_block(queue.raw(), bytes)?;
        //

        Ok((
            DeviceBuffer::from_inner(inner_inflight, self.len),
            EventToken::from_event(evt),
        ))
    }

    ///we use this function for a mapped write. Only used in the Mapped state
    pub fn map_for_write_block(
        self,
        queue: &'brand Queue<'brand>,
    ) -> Result<(DeviceBuffer<'brand, T, Mapped>, MapToken<'brand>)>
    where
        T: bytemuck::Pod,
    {
        if self.len * size_of::<T>() != self.inner.len_bytes() {
            return Err(Error::BufferSizeMismatch {
                expected: self.len * size_of::<T>(),
                actual: self.inner.len_bytes(),
            });
        }

        let (inner_mapped, map_guard) = self.inner.map_for_write_block(queue.raw())?;

        let map_token = MapToken::new(map_guard);

        Ok((DeviceBuffer::from_inner(inner_mapped, self.len), map_token))
    }
}
