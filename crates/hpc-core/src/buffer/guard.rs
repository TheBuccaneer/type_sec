//! Event guard for automatic synchronization

use opencl3::event::Event;

/// Guard that waits for event completion on drop
pub struct GpuEventGuard {
    evt: Option<Event>,
    #[cfg(feature = "metrics")]
    start_time: std::time::Instant,
}

impl GpuEventGuard {
    #[inline]
    pub fn new(evt: Event) -> Self {
        Self { evt: Some(evt) }
    }

    /// Consume the guard and yield the underlying Event.
    /// After this, Drop will NOT wait on the event anymore.
    pub fn into_event(mut self) -> Event {
        self.evt.take().expect("event already taken")
    }

    /// Get reference to underlying event (borrow, non-consuming)
    pub fn event(&self) -> &Event {
        self.evt.as_ref().expect("no event")
    }

    /// Explicitly wait for event completion (consuming).
    /// After this returns, Drop won't wait again.
    #[inline]
    pub fn wait(mut self) {
        if let Some(e) = self.evt.take() {
            let _ = e.wait(); // oder e.wait()? wenn du Result propagieren willst
        }
    }
}

impl Drop for GpuEventGuard {
    fn drop(&mut self) {
        if let Some(evt) = self.evt.take() {
            let _ = evt.wait();
        }
    }
}

impl From<Event> for GpuEventGuard {
    #[inline]
    fn from(evt: Event) -> Self {
        GpuEventGuard::new(evt)
    }
}
