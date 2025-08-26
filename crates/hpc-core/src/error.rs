//! Error handling for the high-level API.
//!
//! Defines a unified Error type wrapping OpenCL error codes

use opencl3::error_codes::ClError;

#[derive(Debug)]
pub enum Error {
    OpenCl(ClError),
    Msg(String),
    AllocationFailed(String),
    BufferSizeMismatch { expected: usize, actual: usize },
    // evtl. mehr Varianten ...
}

pub type Result<T> = std::result::Result<T, Error>;

// --- Implementierungen für automatische Konvertierung --- //
impl From<ClError> for Error {
    fn from(e: ClError) -> Self {
        Error::OpenCl(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Msg(s.to_string())
    }
}
