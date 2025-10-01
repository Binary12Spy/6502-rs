//! Bus module for managing memory-mapped devices and handling read/write operations.
//! This module provides a `BusController` struct that implements the `BusDevice` trait,
//! allowing it to manage multiple devices and route memory accesses appropriately.

/// Errors related to bus operations
pub mod errors;
/// Trait defining the interface for bus devices
pub mod trait_bus_device;

use crate::{errors::BusError, trait_bus_device::BusDevice};

struct DeviceEntry {
    start: u16,
    end: u16,
    device: Box<dyn BusDevice>,
}

/// BusController manages multiple memory-mapped devices and routes read/write operations
/// to the appropriate device based on the address.
pub struct BusController {
    devices: Vec<DeviceEntry>,
}

impl BusController {
    /// Create a new BusController instance
    ///
    /// # Returns
    /// * A new BusController
    ///
    /// # Examples
    /// ``` ignore
    /// let bus = BusController::new();
    /// ```
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Register a device in the memory map
    ///
    /// # Arguments
    /// * `start` - Start address of the device
    /// * `end` - End address of the device
    /// * `device` - The device to register
    ///
    /// # Returns
    /// * `Ok(())` if the device was registered successfully
    /// * `Err(BusError)` if the device overlaps with an existing device
    ///
    /// # Errors
    /// * If the device address range overlaps with an existing device
    ///
    /// # Examples
    /// ``` ignore
    /// let mut bus = BusController::new();
    /// let device = Box::new(MyDevice::new());
    /// bus.register_device(0x2000, 0x2FFF, device).unwrap();
    /// ```
    pub fn register_device(
        &mut self,
        start: u16,
        end: u16,
        device: Box<dyn BusDevice>,
    ) -> Result<(), BusError> {
        // Check if the device overlaps with any existing devices
        for device_entry in &self.devices {
            if (start >= device_entry.start && start <= device_entry.end)
                || (end >= device_entry.start && end <= device_entry.end)
            {
                return Err(BusError::Other(format!(
                    "Device address range 0x{:04X}-0x{:04X} overlaps with existing device range 0x{:04X}-0x{:04X}",
                    start, end, device_entry.start, device_entry.end
                )));
            }
        }

        self.devices.push(DeviceEntry { start, end, device });
        Ok(())
    }
}

impl BusDevice for BusController {
    /// Handle memory reads by forwarding to the correct device
    ///
    /// # Arguments
    /// * `address` - Memory address to read from
    ///
    /// # Returns
    /// * The byte read from memory
    ///
    /// # Errors
    /// * If the memory access is out of range
    /// * If the device read fails
    fn read(&self, address: u16) -> Result<u8, BusError> {
        for device_entry in &self.devices {
            if address >= device_entry.start && address <= device_entry.end {
                return device_entry.device.read(address);
            }
        }
        Err(BusError::AddressOutOfRange(address))
    }

    /// Handle memory writes by forwarding to the correct device
    ///
    /// # Arguments
    /// * `address` - Memory address to write to
    /// * `data` - Byte value to write
    ///
    /// # Errors
    /// * If the memory access is out of range
    /// * If the device write fails
    fn write(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        for device_entry in &mut self.devices {
            if address >= device_entry.start && address <= device_entry.end {
                return device_entry.device.write(address, data);
            }
        }
        Err(BusError::AddressOutOfRange(address))
    }

    /// Perform a clock tick for all devices
    ///
    /// # Examples
    /// ``` ignore
    /// bus.tick();
    /// ```
    fn tick(&mut self) {
        for device_entry in &mut self.devices {
            device_entry.device.tick();
        }
    }

    /// Check the state of the IRQ line
    ///
    /// # Returns
    /// * `true` if any device has its IRQ line asserted
    /// * `false` otherwise
    ///
    /// # Examples
    /// ``` ignore
    /// if bus.check_irq() {
    ///    // Handle IRQ
    /// }
    /// ```
    fn check_irq(&self) -> bool {
        for device_entry in &self.devices {
            if device_entry.device.check_irq() {
                return true;
            }
        }
        false
    }

    /// Check the state of the NMI line
    ///
    /// # Returns
    /// * `true` if any device has its NMI line asserted
    /// * `false` otherwise
    ///
    /// # Examples
    /// ``` ignore
    /// if bus.check_nmi() {
    ///   // Handle NMI
    /// }
    /// ```
    fn check_nmi(&self) -> bool {
        for device_entry in &self.devices {
            if device_entry.device.check_nmi() {
                return true;
            }
        }
        false
    }
}
