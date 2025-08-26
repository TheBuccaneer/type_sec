//! Guard type for mapped device memory.
//!
//! A MapGuard represents a region of device memory that has been
//! mapped into host address space via clEnqueueMapBuffer.
use opencl3::command_queue::CommandQueue;
use opencl3::types::cl_mem;

pub struct MapGuard<'a> {
    queue: &'a CommandQueue,
    mem_obj: cl_mem, // Raw OpenCL memory object
    pub ptr: *mut u8,
}

impl<'a> MapGuard<'a> {
    pub fn new(queue: &'a CommandQueue, mem_obj: cl_mem, ptr: *mut u8) -> Self {
        Self {
            queue,
            mem_obj,
            ptr,
        }
    }
}

impl<'a> Drop for MapGuard<'a> {
    fn drop(&mut self) {
        let _ = self
            .queue
            .enqueue_unmap_mem_object(
                self.mem_obj, // Direkt cl_mem verwenden
                self.ptr as *mut std::ffi::c_void,
                &[],
            )
            .and_then(|event| event.wait());
    }
}
