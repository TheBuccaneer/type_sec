//! Type-state pattern for compile-time state checking of device buffers.

mod sealed {
    /// Sealed trait: prevents implementations outside of this module.
    pub trait Sealed {}
}

/// Common marker trait for all buffer states.
pub trait State: sealed::Sealed + std::fmt::Debug + Send + Sync {}

/// Buffer is freshly created and uninitialized on the device.
/// No valid data available, cannot be used as kernel argument.
#[derive(Debug, Clone, Copy, Default)]
pub struct Empty;
impl sealed::Sealed for Empty {}
impl State for Empty {}


#[derive(Debug, Clone, Copy)]
pub struct Written;
impl sealed::Sealed for Written {}
impl State for Written {}

/// Buffer is mapped for host access (memory-mapped).
#[derive(Debug, Clone, Copy)]
pub struct Mapped;
impl sealed::Sealed for Mapped {}
impl State for Mapped {}

/// Buffer is part of a running asynchronous operation (kernel, read, write).
#[derive(Debug, Clone, Copy)]
pub struct InFlight;
impl sealed::Sealed for InFlight {}
impl State for InFlight {}

/// Buffer operation is completed and synchronized with host.
#[derive(Debug, Clone, Copy)]
pub struct Synchronized;
impl sealed::Sealed for Synchronized {}
impl State for Synchronized {}