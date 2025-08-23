//! Type-state pattern for compile-time state checking of device buffers.

mod sealed {
    /// Sealed trait: verhindert Implementierungen außerhalb dieses Moduls.
    pub trait Sealed {}
}

/// Gemeinsames Marker-Trait für alle Buffer-Zustände.
pub trait State: sealed::Sealed + std::fmt::Debug + Send + Sync {}

// Buffer ist frisch angelegt und uninitialisiert auf dem Device.
/// Keine gültigen Daten vorhanden, kann nicht als Kernel-Argument verwendet werden.
#[derive(Debug, Clone, Copy, Default)]
pub struct Empty;
impl sealed::Sealed for Empty {}
impl State for Empty {}

/// Buffer enthält gültige Daten vom Host oder durch Kernel-Operation.
/// Daten sind synchronisiert und bereit für weitere Operationen.
/// Host-I/O ist verfügbar, Buffer kann als Kernel-Argument verwendet werden.
#[derive(Debug, Clone, Copy)]
pub struct Written;
impl sealed::Sealed for Written {}
impl State for Written {}

/// Buffer ist für Host-Zugriff gemappt (Memory-Mapped).
/// Direkter Speicherzugriff zwischen Host und Device möglich.
/// Kein gleichzeitiger Kernel-Zugriff erlaubt während Mapping aktiv ist.
#[derive(Debug, Clone, Copy)]
pub struct Mapped;
impl sealed::Sealed for Mapped {}
impl State for Mapped {}

/// Buffer ist Teil einer laufenden asynchronen Operation (Kernel, Read, Write).
/// Keine Host-I/O Operationen verfügbar bis Operation abgeschlossen ist.
/// Event-basierte Synchronisation erforderlich für Zustandsübergang.
#[derive(Debug, Clone, Copy)]
pub struct InFlight;
impl sealed::Sealed for InFlight {}
impl State for InFlight {}

/// Buffer-Operation ist abgeschlossen und mit Host synchronisiert.
/// Resultate sind verfügbar, Buffer kann für neue Operationen verwendet werden.
/// Äquivalent zu OpenCL's CL_COMPLETE Event-Status.
#[derive(Debug, Clone, Copy)]
pub struct Synchronized;
impl sealed::Sealed for Synchronized {}
impl State for Synchronized {}
