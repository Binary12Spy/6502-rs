//! Library for handling ROM files and sizes for 6502-based systems.

/// ROM size definitions and utilities.
pub mod rom_size;

use bus::errors::BusError;
use bus::trait_bus_device::BusDevice;

use crate::rom_size::RomSize;

/// Represents a Read-Only Memory (ROM) module.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Rom {
    /// Actual ROM memory
    memory: Vec<u8>,
    /// Size of ROM
    size: RomSize,
    /// Start address of ROM
    start_address: u16,
}

impl Rom {
    /// Create a new ROM instance with the specified size and start address.
    ///
    /// # Arguments
    /// * `size` - Size of the ROM (default is 32KB)
    /// * `start_address` - Start address of the ROM in memory (default is 0x8000)
    ///
    /// # Returns
    /// * A new Rom instance
    ///
    /// # Examples
    /// ``` ignore
    /// let rom = Rom::new(RomSize::_32K, 0x8000);
    /// ```
    pub fn new(size: RomSize, start_address: u16) -> Self {
        Self {
            memory: vec![0; size as usize],
            size,
            start_address,
        }
    }

    /// Import data into the ROM at the specified offset.
    ///
    /// # Arguments
    /// * `data` - Data to import into the ROM
    /// * `offset` - Offset within the ROM to start importing data (default is 0)
    ///
    /// # Returns
    /// * `Ok(())` if data was imported successfully
    /// * `Err(String)` if the data exceeds ROM size
    ///
    /// # Errors
    /// * If the data length plus offset exceeds the ROM size
    ///
    /// # Examples
    /// ``` ignore
    /// let mut rom = Rom::new(RomSize::_32K, 0x8000);
    /// let data = vec![0x00, 0x01, 0x02, 0x03];
    /// rom.load(&data, 0).unwrap();
    /// ```
    pub fn import(&mut self, data: &[u8], offset: usize) -> Result<(), String> {
        if offset + data.len() > self.memory.len() {
            return Err("Data exceeds ROM size".to_string());
        }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Export data from the ROM at the specified offset and length.
    ///
    /// # Arguments
    /// * `offset` - Offset within the ROM to start exporting data (default is 0)
    /// * `length` - Length of data to export (default is entire ROM size)
    ///
    /// # Returns
    /// * A vector containing the exported data
    ///
    /// # Examples
    /// ``` ignore
    /// let rom = Rom::new(RomSize::_32K, 0x8000);
    /// let data = rom.export(0, 16);
    /// ```
    pub fn export(&self, offset: usize, length: usize) -> Vec<u8> {
        let end = (offset + length).min(self.memory.len());
        self.memory[offset..end].to_vec()
    }
}

impl BusDevice for Rom {
    fn read(&self, address: u16) -> Result<u8, BusError> {
        let offset = address.wrapping_sub(self.start_address) as usize;
        if offset < self.memory.len() {
            Ok(self.memory[offset])
        } else {
            Err(BusError::AddressOutOfRange(address))
        }
    }

    fn write(&mut self, address: u16, _data: u8) -> Result<(), BusError> {
        Err(BusError::ReadOnly(address))
    }

    fn tick(&mut self) {
        // ROM does not need to do anything on tick
    }

    fn check_irq(&self) -> bool {
        // ROM does not generate IRQs
        false
    }

    fn check_nmi(&self) -> bool {
        // ROM does not generate NMIs
        false
    }
}
