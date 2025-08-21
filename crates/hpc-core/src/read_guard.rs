use crate::api::DeviceBuffer;
use crate::buffer::state::{InFlight, Ready};
use super::EventToken;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Guard that holds a mutable slice until GPU read operation completes
#[must_use]
pub struct ReadGuard<'a, 'q, T> {
    slice: &'a mut [T],
    token: EventToken<'q>,
}

impl<'a, 'q, T> ReadGuard<'a, 'q, T> {
    /// Create new ReadGuard (internal use only)
    pub(crate) fn new(slice: &'a mut [T], token: EventToken<'q>) -> Self {
        Self { slice, token }
    }
    
    /// Wait until GPU is finished, then return Ready buffer and release the slice
    #[must_use]
    pub fn wait(self, buf: DeviceBuffer<T, InFlight>) -> DeviceBuffer<T, Ready> {
        let buf_ready = self.token.wait(buf);
        // Nach dem wait() ist self konsumiert und die Slice-Referenz wird freigegeben
        // Der Aufrufer kann seine ursprüngliche Slice wieder normal nutzen
        buf_ready
    }
}

// ReadGuard ist NICHT Deref/DerefMut - das wäre unsicher!
// Die Daten sind erst nach wait() gültig

// Optional: Debug implementation
impl<'a, 'q, T> std::fmt::Debug for ReadGuard<'a, 'q, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadGuard")
            .field("slice_len", &self.slice.len())
            .finish()
    }
}