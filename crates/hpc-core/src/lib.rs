//! HPC Core Library
//! 
//! Provides both low-level and high-level APIs for GPU computing with OpenCL.

//=============================================================================
// LOW-LEVEL MODULES
//=============================================================================

pub mod buffer;
pub mod error;
pub mod memtracer;
pub mod metrics;

//=============================================================================
// HIGH-LEVEL API
//=============================================================================

pub mod api;

// Re-export the main high-level API for easy access
pub use api::{
    // Core types
    DeviceBuffer, Context, Queue, Kernel,
    // Utilities  
    EventToken, ReadGuard,
    // Functions
    create_buffer,
    // Error handling
    Error, Result,
};

//=============================================================================
// LOW-LEVEL RE-EXPORTS (for advanced users)
//=============================================================================

// Buffer states for advanced usage
pub use buffer::state::{Empty, InFlight, Ready, State};

// Low-level buffer for tests/benches
pub use buffer::GpuBuffer;