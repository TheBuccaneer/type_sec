// src/api/queue.rs

use opencl3::command_queue::CommandQueue as CLQueue;

//=============================================================================
// QUEUE
//=============================================================================

#[must_use]
#[derive(Debug)]
pub struct Queue {
    pub(crate) inner: CLQueue,
}

impl Queue {
    pub fn raw(&self) -> &CLQueue {
        &self.inner
    }
}