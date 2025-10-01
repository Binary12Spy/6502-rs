use std::fmt;

use bus::errors::BusError;

#[derive(Debug)]
/// Errors related to CPU operations
pub enum CpuError {
    /// A Bus error occurred
    BusError(BusError),
    /// ALU operation error
    AluError(String),
    /// Unknown instruction error
    UnknownInstruction,
    /// Unsupported operation error
    UnsupportedOperation(String),
    /// Other unspecified CPU error
    Other(String),
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuError::BusError(err) => write!(f, "Bus error: {}", err),
            CpuError::AluError(msg) => write!(f, "ALU error: {}", msg),
            CpuError::UnknownInstruction => write!(f, "Unknown instruction error"),
            CpuError::UnsupportedOperation(msg) => {
                write!(f, "Unsupported operation error: {}", msg)
            }
            CpuError::Other(msg) => write!(f, "Other CPU error: {}", msg),
        }
    }
}

impl std::error::Error for CpuError {}
