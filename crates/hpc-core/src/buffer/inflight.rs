use super::GpuBuffer;
use crate::buffer::GpuEventGuard;
use crate::buffer::state::{InFlight, Ready};
use opencl3::event::Event;
use std::marker::PhantomData;

// InFlight state implementation
impl GpuBuffer<InFlight> {
    /// S3: The only legal transition out of `InFlight`.
    /// Consumes `self` and the completion event, waits for it, and returns `Ready`.
    /// Double-wait is prevented by taking `self` by value. Host I/O remains unavailable until this returns.
    pub fn wait(self, evt: Event) -> GpuBuffer<Ready> {
        let _g = GpuEventGuard::new(evt);

        #[cfg(feature = "metrics")]
        crate::metrics::record("complete", Instant::now());
        #[cfg(feature = "metrics")]
        mlog("pipeline.complete", self.len);

        GpuBuffer {
            buf: self.buf,
            len_bytes: self.len_bytes,
            _state: PhantomData::<Ready>,
        }
    }
}
