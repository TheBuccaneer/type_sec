//! HPC Core Library
//!
//! Provides both low-level and high-level APIs for GPU computing with OpenCL.

//=============================================================================
// LOW-LEVEL MODULES
//=============================================================================

pub mod buffer;
pub mod error;

//=============================================================================
// HIGH-LEVEL API
//=============================================================================

pub mod api;

// Re-export the main high-level API for easy access
pub use api::{
    Context,
    // Core types
    DeviceBuffer,
    // Error handling
    Error,
    // Utilities
    EventToken,
    Kernel,
    Queue,
    ReadGuard,
    Result,
};

//=============================================================================
// LOW-LEVEL RE-EXPORTS (for advanced users)
//=============================================================================

// Buffer states for advanced usage
pub use buffer::state::{Empty, InFlight, Mapped, State, Written};

// Low-level buffer for tests/benches
pub use buffer::GpuBuffer;
