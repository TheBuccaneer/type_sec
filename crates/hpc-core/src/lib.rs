//Low-Level-Module
pub mod buffer;
pub mod error;
pub mod event_token;
pub mod memtracer;
pub mod metrics;

pub mod read_guard;

pub use read_guard::*;


pub use event_token::EventToken;

// High-Level-API (noch im Aufbau)
pub mod api;

// Für interne Nutzung: States und Error sichtbar machen
pub use buffer::state::{Empty, InFlight, Ready, State};

// Falls du das Low-Level für Tests/Benches brauchst, kannst du
// die zentralen Typen auch re-exportieren:
pub use buffer::GpuBuffer;

pub use crate::error::{Error, Result};
// Platzhalter für einheitliches Fehler-Handling
// (später evtl. eigenes Error-Enum in error.rs)
