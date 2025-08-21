use crate::api::DeviceBuffer;
use crate::buffer::GpuEventGuard;
use crate::buffer::state::{InFlight, Ready};
use core::marker::PhantomData;
use opencl3::event::Event;

/// High-Level EventToken.
/// - #[must_use], nicht Copy
/// - einzig erlaubter Pfad: wait() konsumiert Token + InFlight
pub struct EventToken<'brand> {
    inner: GpuEventGuard,
    _brand: PhantomData<&'brand ()>,
}

impl<'brand> EventToken<'brand> {
    /// Erzeugt ein Token direkt aus einem Guard (Low-Level).
    pub(crate) fn from_guard(guard: GpuEventGuard) -> Self {
        Self {
            inner: guard,
            _brand: PhantomData,
        }
    }

    /// Erzeugt ein Token direkt aus einem rohen OpenCL-Event.
    pub(crate) fn from_event(evt: Event) -> Self {
        Self::from_guard(GpuEventGuard::new(evt))
    }

    /// Konsumierender Übergang: einzig erlaubter Pfad von InFlight → Ready.
    #[must_use]
    pub fn wait<T>(self, buf: DeviceBuffer<'brand, T, InFlight>) -> DeviceBuffer<'brand, T, Ready>{
        self.inner.wait();
        buf.into_ready()
    }
}
