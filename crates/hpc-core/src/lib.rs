#![doc = include_str!("../README.md")]

/// Vollständige Spezifikation (S1–S3) direkt in der Doku.
/// Quelle: `crates/hpc-core/SPEC.md`.
#[doc = include_str!("../SPEC.md")]
pub mod spec {}


// HPC-Core: High-Performance Computing utilities for OpenCL
// 
// This crate provides safe wrappers and utilities for GPU computing.

// Core modules (always available)
mod error;
mod buffer;
pub mod api;

// Re-export core types
pub use error::{ClError, Result};
pub use buffer::{GpuBuffer, GpuEventGuard};
pub use buffer::state::{State, Queued, InFlight, Ready};

// Feature-gated modules
#[cfg(feature = "metrics")]
pub mod metrics;

#[cfg(feature = "memtrace")]
pub mod memtracer;
#[cfg(feature = "memtrace")]
pub use memtracer::{
    start, flush_csv, reset,
    Dir, Operation, CopyToken, TracingScope,
    is_auto_trace_enabled, enable_auto_trace, disable_auto_trace,
    AbortEvent, AbortTokenGuard, set_abort_token, clear_abort_token,
    log_abort, log_transfer, now_us,
};

// FFI callback for memtrace
#[cfg(feature = "memtrace")]
use std::ffi::c_void;

#[cfg(feature = "memtrace")]
pub extern "C" fn memtrace_callback(
    _evt: opencl3::types::cl_event,
    _status: opencl3::types::cl_int,
    user_data: *mut c_void,
) {
    // SAFETY: Pointer was obtained via Box::into_raw, so it is non-null and uniquely owned.
    let tok: Box<CopyToken> = unsafe { Box::from_raw(user_data.cast()) };
    tok.finish();
}

// Re-exports / Fallbacks für Beispiele:
#[cfg(feature = "metrics")]
pub use metrics::summary;

#[cfg(not(feature = "metrics"))]
#[inline]
pub fn summary() {} // No-Op, damit Beispiele ohne Feature bauen