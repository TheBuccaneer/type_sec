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
pub use buffer::state::{Empty, InFlight, Ready, State};

// Low-level buffer for tests/benches
pub use buffer::GpuBuffer;


// memtracer öffentlich machen (falls noch nicht)
#[cfg(feature = "memtrace")]
pub mod memtracer;

// C-Callback, das der Event aufruft, wenn er fertig ist (CL_COMPLETE)
#[cfg(feature = "memtrace")]
#[no_mangle]
pub extern "C" fn memtrace_callback(_event: *const core::ffi::c_void, user_data: *mut core::ffi::c_void) {
    // Box zurückholen → finish() loggt und droppt das Token
    unsafe { Box::from_raw(user_data.cast::<crate::memtracer::CopyToken>()) }.finish();
}
