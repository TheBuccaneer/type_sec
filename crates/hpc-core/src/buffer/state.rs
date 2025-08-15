//! Type-state pattern for compile-time state checking of device buffers.

mod sealed {
    /// Sealed trait: verhindert Implementierungen außerhalb dieses Moduls.
    pub trait Sealed {}
}

/// Gemeinsames Marker-Trait für alle Buffer-Zustände.
pub trait State: sealed::Sealed + std::fmt::Debug + Send + Sync {}

/// Puffer ist frisch angelegt / uninitialisiert auf dem Device.
#[derive(Debug, Clone, Copy, Default)]
pub struct Empty;
impl sealed::Sealed for Empty {}
impl State for Empty {}

/// Puffer enthält gültige Daten und ist nicht in-flight.
#[derive(Debug, Clone, Copy)]
pub struct Ready;
impl sealed::Sealed for Ready {}
impl State for Ready {}

/// Auf dem Puffer läuft gerade ein Command (Write/Kernel/Read).
/// Kein Host-Zugriff, bis gewartet wurde.
#[derive(Debug, Clone, Copy)]
pub struct InFlight;
impl sealed::Sealed for InFlight {}
impl State for InFlight {}

#[deprecated(since = "0.1.0", note = "Queued integrated in Ready")]
pub type Queued = Ready;
