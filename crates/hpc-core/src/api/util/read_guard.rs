use super::EventToken;
use crate::api::DeviceBuffer;
use crate::buffer::state::{InFlight, Written};

/// Guard that holds a mutable slice until GPU read operation completes
#[must_use]
pub struct ReadGuard<'a, 'brand, T> {
    slice: &'a mut [T],
    token: EventToken<'brand>,
}

impl<'a, 'brand, T> ReadGuard<'a, 'brand, T> {
    /// Create new ReadGuard (internal use only)
    pub(crate) fn new(slice: &'a mut [T], token: EventToken<'brand>) -> Self {
        Self { slice, token }
    }

    /// Wait until GPU is finished, then return Ready buffer and release the slice
    #[must_use]
    pub fn wait(self, buf: DeviceBuffer<'brand, T, InFlight>) -> DeviceBuffer<'brand, T, Written> {
        let buf_ready = self.token.wait(buf);
        // Nach dem wait() ist self konsumiert und die Slice-Referenz wird freigegeben
        // Der Aufrufer kann seine ursprüngliche Slice wieder normal nutzen
        buf_ready
    }
}

// ReadGuard ist NICHT Deref/DerefMut - das wäre unsicher!
// Die Daten sind erst nach wait() gültig

// Optional: Debug implementation
impl<'a, 'brand, T> std::fmt::Debug for ReadGuard<'a, 'brand, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadGuard")
            .field("slice_len", &self.slice.len())
            .finish()
    }
}
