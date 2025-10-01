use std::fmt;

#[derive(Debug)]
/// Errors related to bus operations
pub enum BusError {
    /// An address is out of the valid range for a device
    AddressOutOfRange(u16),
    /// Attempted write to a read-only address
    ReadOnly(u16),
    /// Attempted read from a write-only address
    WriteOnly(u16),
    /// No device found at the specified address
    DeviceNotFound(u16),
    /// Invalid data encountered
    InvalidData,
    /// Other unspecified bus error
    Other(String),
}

impl fmt::Display for BusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusError::AddressOutOfRange(addr) => write!(f, "Address out of range: 0x{:04X}", addr),
            BusError::ReadOnly(addr) => write!(f, "Attempted write to read-only address: 0x{:04X}", addr),
            BusError::WriteOnly(addr) => write!(f, "Attempted read from write-only address: 0x{:04X}", addr),
            BusError::DeviceNotFound(addr) => write!(f, "No device found at address: 0x{:04X}", addr),
            BusError::InvalidData => write!(f, "Invalid data encountered"),
            BusError::Other(msg) => write!(f, "Other bus error: {}", msg),
        }
    }
}

impl std::error::Error for BusError {}