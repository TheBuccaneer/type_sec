//! Type-state pattern for compile-time state checking of device buffers.

mod sealed {
    /// Sealed trait: verhindert Implementierungen außerhalb dieses Moduls.
    pub trait Sealed {}
}

/// Gemeinsames Marker-Trait für alle Buffer-Zustände.
pub trait State: sealed::Sealed + std::fmt::Debug + Send + Sync {}

/// Puffer ist frisch angelegt / uninitialisiert auf dem Device.
#[derive(Debug, Clone, Copy, Default)]

/// Type-state: buffer is empty/uninitialized and not a valid kernel argument.
/// Transitions: created/reset here; not usable for host I/O (S1).
pub struct Empty;
impl sealed::Sealed for Empty {}
impl State for Empty {}

/// Puffer enthält gültige Daten und ist nicht in-flight.
#[derive(Debug, Clone, Copy)]

/// Type-state: buffer is synchronized with the device.
/// Invariant S1: host I/O is only available in `Ready`.
/// Transition: kernel enqueue consumes `Ready` and yields `InFlight` (S2).
pub struct Ready;
impl sealed::Sealed for Ready {}
impl State for Ready {}

/// Auf dem Puffer läuft gerade ein Command (Write/Kernel/Read).
/// Kein Host-Zugriff, bis gewartet wurde.
#[derive(Debug, Clone, Copy)]

/// Type-state: buffer is owned by GPU work (in flight).
/// Invariant S1: no host I/O is available in `InFlight`.
/// Transition S3: the only legal exit is `wait(self, Event) -> Ready`.
pub struct InFlight;
impl sealed::Sealed for InFlight {}
impl State for InFlight {}

/// API-stability shim: `Queued` is an alias of `Ready`.
/// New code should prefer `Ready`. Kept to avoid deprecation noise.
pub type Queued = Ready;
