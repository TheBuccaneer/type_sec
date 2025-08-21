// src/api/queue.rs

use opencl3::command_queue::CommandQueue as CLQueue;
use std::marker::PhantomData;
//=============================================================================
// QUEUE
//=============================================================================

#[must_use]
#[derive(Debug)]
pub struct Queue<'brand> {
    pub(crate) inner: CLQueue,
    pub(crate) _brand: PhantomData<&'brand ()>, // Branding Lifetime
}

impl<'brand> Queue<'brand> {
    /// Low-Level-Zugriff auf die OpenCL CommandQueue
    ///
    /// Normalerweise nicht nötig, da die High-Level API alles abdeckt.
    pub fn raw(&self) -> &CLQueue {
        &self.inner
    }
}
