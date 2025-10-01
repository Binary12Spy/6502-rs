//! Library for handling RAM for 6502-based systems.

#[allow(dead_code)]

/// RAM size definitions and utilities.
pub mod ram_size;

use bus::errors::BusError;
use bus::trait_bus_device::BusDevice;

use crate::ram_size::RamSize;

/// Represents a Random Access Memory (RAM) module.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Ram {
    /// Actual RAM memory
    memory: Vec<u8>,
    /// Size of RAM
    size: RamSize,
    /// Start address of RAM
    start_address: u16,
}

impl Ram {
    /// Create a new RAM instance with the specified size and start address.
    ///
    /// # Arguments
    /// * `size` - Size of the RAM (default is 32KB)
    /// * `start_address` - Start address of the RAM in memory (default is 0x8000)
    ///
    /// # Returns
    /// * A new Ram instance
    ///
    /// # Examples
    /// ``` ignore
    /// let ram = Ram::new(RamSize::_32K, 0x8000);
    /// ```
    pub fn new(size: RamSize, start_address: u16) -> Self {
        Self {
            memory: vec![0; size as usize],
            size,
            start_address,
        }
    }

    /// Import data into the RAM at the specified offset.
    ///
    /// # Arguments
    /// * `data` - Data to import into the RAM
    /// * `offset` - Offset within the RAM to start importing data (default is 0)
    ///
    /// # Returns
    /// * `Ok(())` if data was imported successfully
    /// * `Err(String)` if the data exceeds RAM size
    ///
    /// # Errors
    /// * If the data length plus offset exceeds the RAM size
    ///
    /// # Examples
    /// ``` ignore
    /// let mut ram = Ram::new(RamSize::_32K, 0x8000);
    /// let data = vec![0x00, 0x01, 0x02, 0x03];
    /// ram.load(&data, 0).unwrap();
    /// ```
    pub fn import(&mut self, data: &[u8], start_address: u16) -> Result<(), String> {
        let offset = start_address as usize;
        if offset + data.len() > self.memory.len() {
            return Err("Data exceeds RAM size".to_string());
        }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Export data from the RAM at the specified offset and length.
    ///
    /// # Arguments
    /// * `offset` - Offset within the RAM to start exporting data (default is 0)
    /// * `length` - Length of data to export (default is entire RAM size)
    ///
    /// # Returns
    /// * A vector containing the exported data
    ///
    /// # Examples
    /// ``` ignore
    /// let ram = Ram::new(RamSize::_32K, 0x8000);
    /// let data = ram.export(0, 16);
    /// ```
    pub fn export(&self, start_address: u16, length: usize) -> Vec<u8> {
        let offset = start_address as usize;
        let end = (offset + length).min(self.memory.len());
        self.memory[offset..end].to_vec()
    }
}

impl BusDevice for Ram {
    fn read(&self, address: u16) -> Result<u8, BusError> {
        let offset = address.wrapping_sub(self.start_address) as usize;
        if offset < self.memory.len() {
            Ok(self.memory[offset])
        } else {
            Err(BusError::AddressOutOfRange(address))
        }
    }

    fn write(&mut self, _address: u16, _data: u8) -> Result<(), BusError> {
        let offset = _address.wrapping_sub(self.start_address) as usize;
        if offset < self.memory.len() {
            self.memory[offset] = _data;
            Ok(())
        } else {
            Err(BusError::AddressOutOfRange(_address))
        }
    }

    fn tick(&mut self) {
        // RAM does not need to do anything on tick
    }

    fn check_irq(&self) -> bool {
        // RAM does not generate IRQs
        false
    }

    fn check_nmi(&self) -> bool {
        // RAM does not generate NMIs
        false
    }
}
