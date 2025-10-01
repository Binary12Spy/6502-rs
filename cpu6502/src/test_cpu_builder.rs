//! CPU Builder pattern for testing and setup

#![allow(dead_code)]
use crate::{cpu::Cpu, errors::CpuError, flags::Flags, registers::Registers};
use bus::{BusController, errors::BusError, trait_bus_device::BusDevice};
use ram::{Ram, ram_size::RamSize};
use rom::{Rom, rom_size::RomSize};

/// A builder for creating and configuring CPU instances for testing
#[cfg(test)]
pub struct CpuBuilder {
    bus: BusController,
    registers: Option<Registers>,
    flags: Option<Flags>,
    reset_vector: Option<u16>,
}

#[cfg(test)]
impl CpuBuilder {
    /// Create a new CPU builder
    pub fn new() -> Self {
        Self {
            bus: BusController::new(),
            registers: None,
            flags: None,
            reset_vector: None,
        }
    }

    /// Add a generic bus device
    pub fn with_bus_device<T: BusDevice + 'static>(
        mut self,
        device: T,
        start_address: u16,
        end_address: u16,
    ) -> Result<Self, BusError> {
        self.bus
            .register_device(start_address, end_address, Box::new(device))?;
        Ok(self)
    }

    /// Set the CPU registers
    pub fn with_registers(mut self, registers: Registers) -> Self {
        self.registers = Some(registers);
        self
    }

    /// Set individual register values
    pub fn with_accumulator(mut self, value: u8) -> Self {
        let mut registers = self.registers.unwrap_or_default();
        registers.accumulator = value;
        self.registers = Some(registers);
        self
    }

    /// Set X register
    pub fn with_x_register(mut self, value: u8) -> Self {
        let mut registers = self.registers.unwrap_or_default();
        registers.x = value;
        self.registers = Some(registers);
        self
    }

    /// Set Y register
    pub fn with_y_register(mut self, value: u8) -> Self {
        let mut registers = self.registers.unwrap_or_default();
        registers.y = value;
        self.registers = Some(registers);
        self
    }

    /// Set program counter
    pub fn with_program_counter(mut self, value: u16) -> Self {
        let mut registers = self.registers.unwrap_or_default();
        registers.program_counter = value;
        self.registers = Some(registers);
        self
    }

    /// Set stack pointer
    pub fn with_stack_pointer(mut self, value: u8) -> Self {
        let mut registers = self.registers.unwrap_or_default();
        registers.stack_pointer = value;
        self.registers = Some(registers);
        self
    }

    /// Set the CPU flags
    pub fn with_flags(mut self, flags: Flags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Set individual flag values
    pub fn with_carry_flag(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.carry = value;
        self.flags = Some(flags);
        self
    }

    /// Set zero flag
    pub fn with_zero_flag(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.zero = value;
        self.flags = Some(flags);
        self
    }

    /// Set interrupt disable flag
    pub fn with_interrupt_disable(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.interrupt_disable = value;
        self.flags = Some(flags);
        self
    }

    /// Set decimal mode flag
    pub fn with_decimal_mode(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.decimal_mode = value;
        self.flags = Some(flags);
        self
    }

    /// Set break command flag
    pub fn with_break_flag(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.break_command = value;
        self.flags = Some(flags);
        self
    }

    /// Set overflow flag
    pub fn with_overflow_flag(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.overflow = value;
        self.flags = Some(flags);
        self
    }

    /// Set negative flag
    pub fn with_negative_flag(mut self, value: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.negative = value;
        self.flags = Some(flags);
        self
    }

    /// Set the reset vector (where PC should point after reset)
    pub fn with_reset_vector(mut self, address: u16) -> Self {
        self.reset_vector = Some(address);
        self
    }

    /// Build the CPU with the configured settings
    pub fn build(self) -> Result<Cpu, CpuError> {
        // Create the CPU
        let mut cpu = Cpu::new(self.bus);

        // Set up reset vector if specified
        if let Some(reset_addr) = self.reset_vector {
            let low_byte = (reset_addr & 0xFF) as u8;
            let high_byte = ((reset_addr >> 8) & 0xFF) as u8;
            cpu.bus
                .write(0xFFFC, low_byte)
                .map_err(CpuError::BusError)?;
            cpu.bus
                .write(0xFFFD, high_byte)
                .map_err(CpuError::BusError)?;
        }

        // Apply register settings
        if let Some(registers) = self.registers {
            cpu.registers = registers;
        }

        // Apply flag settings
        if let Some(flags) = self.flags {
            cpu.flags = flags;
        }

        Ok(cpu)
    }

    /// Build and reset the CPU
    pub fn build_and_reset(self) -> Result<Cpu, CpuError> {
        let mut cpu = self.build()?;
        cpu.reset()?;
        Ok(cpu)
    }
}

#[cfg(test)]
impl Default for CpuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cpu_builder() {
        let cpu = CpuBuilder::new()
            .with_accumulator(0x42)
            .with_carry_flag(true)
            .build()
            .expect("Failed to build CPU");

        assert_eq!(cpu.registers.accumulator, 0x42);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn test_cpu_builder_with_ram() {
        let ram = Ram::new(RamSize::_2K, 0x0000);
        let cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x07FF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        // Test that we can read from RAM (should be 0 by default)
        let result = cpu.bus.read(0x0000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x00);
    }

    #[test]
    fn test_cpu_builder_with_memory_data() {
        let mut ram = Ram::new(RamSize::_2K, 0x0000);
        ram.import(&[0x10, 0x20, 0x30, 0x40], 0x0200)
            .expect("Failed to import data");
        let cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x07FF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        // Test that memory contains our data
        assert_eq!(cpu.bus.read(0x0200).unwrap(), 0x10);
        assert_eq!(cpu.bus.read(0x0201).unwrap(), 0x20);
        assert_eq!(cpu.bus.read(0x0202).unwrap(), 0x30);
        assert_eq!(cpu.bus.read(0x0203).unwrap(), 0x40);
    }

    #[test]
    fn test_cpu_builder_complete_setup() {
        let ram = Ram::new(RamSize::_32K, 0x0000);
        let mut rom = Rom::new(RomSize::_32K, 0x8000);

        // Pre-populate ROM with reset vector at 0xFFFC and 0xFFFD
        // 0xFFFC = 0x8000 & 0xFF = 0x00 (low byte)
        // 0xFFFD = (0x8000 >> 8) & 0xFF = 0x80 (high byte)
        let reset_vector_data = vec![0x00, 0x80]; // Little-endian: 0x8000
        let rom_offset = 0xFFFC - 0x8000; // Offset within ROM for reset vector
        rom.import(&reset_vector_data, rom_offset as usize)
            .expect("Failed to set reset vector in ROM");

        let cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x7FFF)
            .expect("Failed to add RAM")
            .with_bus_device(rom, 0x8000, 0xFFFF)
            .expect("Failed to add ROM")
            .with_accumulator(0x55)
            .with_x_register(0xAA)
            .with_y_register(0xFF)
            .with_program_counter(0x8000)
            .with_stack_pointer(0xFD)
            .with_carry_flag(true)
            .with_zero_flag(false)
            .with_negative_flag(true)
            .build()
            .expect("Failed to build CPU");

        // Test register values
        assert_eq!(cpu.registers.accumulator, 0x55);
        assert_eq!(cpu.registers.x, 0xAA);
        assert_eq!(cpu.registers.y, 0xFF);
        assert_eq!(cpu.registers.program_counter, 0x8000);
        assert_eq!(cpu.registers.stack_pointer, 0xFD);

        // Test flag values
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, true);

        // Test reset vector
        assert_eq!(cpu.bus.read(0xFFFC).unwrap(), 0x00); // Low byte of 0x8000
        assert_eq!(cpu.bus.read(0xFFFD).unwrap(), 0x80); // High byte of 0x8000
    }
}
