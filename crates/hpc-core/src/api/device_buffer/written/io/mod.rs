//! I/O operations for `DeviceBuffer<T, Ready>`
//!
//! This module groups read and write implementations, which are attached
//! directly to `DeviceBuffer<T, Ready>`.
//! There are no public re-exports here.

mod read;
mod write;
