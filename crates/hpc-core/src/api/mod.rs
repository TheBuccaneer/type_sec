// src/api/mod.rs

// Re-export von Error types
pub use crate::error::{Error, Result};

// Submodule
mod device_buffer;
mod opencl;

// Re-exports der Submodule
pub use device_buffer::DeviceBuffer;
pub use opencl::{Context, Kernel, Queue};

mod util;
//pub use util::{EventToken, ReadGuard};
pub use util::*;
