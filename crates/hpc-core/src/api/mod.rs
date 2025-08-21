// src/api/mod.rs

// Re-export von Error types
pub use crate::error::{Error, Result};

// Submodule
mod device_buffer;
mod opencl;
mod functions;

// Re-exports der Submodule
pub use device_buffer::DeviceBuffer;
pub use opencl::{Context, Queue, Kernel};
pub use functions::create_buffer;

mod util;
pub use util::{EventToken, ReadGuard};