//! Trait defining the interface for bus devices

use crate::errors::BusError;

/// This module provides the `BusDevice` trait which must be implemented by any device
/// that wants to be connected to the `BusController`.
pub trait BusDevice {
    /// Read a byte from the device at the specified address
    /// # Arguments
    /// * `address` - The address to read from
    /// # Returns
    /// * `Ok(u8)` containing the data read
    /// * `Err(String)` if the read fails
    /// # Errors
    /// * If the address is out of range for the device
    fn read(&self, address: u16) -> Result<u8, BusError>;
    /// Write a byte to the device at the specified address
    fn write(&mut self, address: u16, data: u8) -> Result<(), BusError>;

    /// Perform a clock tick for the device
    fn tick(&mut self);

    /// Check the state of the IRQ line
    fn check_irq(&self) -> bool;
    /// Check the state of the NMI line
    fn check_nmi(&self) -> bool;
}
