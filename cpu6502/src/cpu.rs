use crate::errors::CpuError;
use crate::flags::Flags;
use crate::opcodes::{
    instruction_variants::{DEFAULT_INSTRUCTION_VARIANT, InstructionVariant},
    microcode::{MicrocodeStep, OperationResult},
    variant_by_opcode,
};
use crate::registers::Registers;
use bus::{BusController, trait_bus_device::BusDevice};
use std::slice::Iter;

const PROGRAM_COUNTER_RESET_VECTOR: u16 = 0xFFFC;

/// 6502 CPU
pub struct Cpu {
    /// CPU Flags
    pub(crate) flags: Flags,
    /// CPU Registers
    pub(crate) registers: Registers,
    /// Bus Controller
    pub(crate) bus: BusController,
    /// Current Instruction Variant
    pub(crate) current_instruction: &'static InstructionVariant,
    /// Current microcode iter for the instruction being executed
    pub(crate) current_microcode_iter: Iter<'static, MicrocodeStep>,
    /// Temporary address storage for operations
    pub(crate) temp_address: u16,
    /// Temporary data storage for operations
    pub(crate) temp_data: u8,
    /// Page boundary cross penalty cycles
    pub(crate) page_boundary_cross_penalty: u8,
    /// Total CPU cycles executed
    pub(crate) cycles: u64,
}

impl Cpu {
    /// Create a new CPU instance with the provided BusController
    ///
    /// # Arguments
    /// * `bus` - The BusController to connect the CPU to
    ///
    /// # Returns
    /// * A new Cpu instance
    ///
    /// # Examples
    /// ``` ignore
    /// let bus = BusController::new();
    /// let cpu = Cpu::new(bus);
    /// ```
    pub fn new(bus: BusController) -> Cpu {
        Cpu {
            flags: Flags::default(),
            registers: Registers::default(),
            bus,
            current_instruction: DEFAULT_INSTRUCTION_VARIANT,
            current_microcode_iter: [].iter(),
            temp_address: 0,
            temp_data: 0,
            page_boundary_cross_penalty: 0,
            cycles: 0,
        }
    }

    /// Reset the CPU to its initial state
    ///
    /// This sets the registers to their default values and initializes the program counter
    /// from the reset vector located at memory addresses 0xFFFC and 0xFFFD.
    ///
    /// # Returns
    /// * `Ok(())` if the reset was successful
    /// * `Err(CpuError)` if there was an error reading from the bus
    ///
    /// # Errors
    /// * `CpuError::BusError` if there is an error reading from the bus
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.reset()?;
    /// ```
    pub fn reset(&mut self) -> Result<(), CpuError> {
        self.registers = Registers::default();
        self.flags = Flags::default();

        // Program counter is read from the reset vector
        let low_byte = self
            .bus
            .read(PROGRAM_COUNTER_RESET_VECTOR)
            .map_err(CpuError::BusError)?;
        let high_byte = self
            .bus
            .read(PROGRAM_COUNTER_RESET_VECTOR + 1)
            .map_err(CpuError::BusError)?;
        self.registers.program_counter = ((high_byte as u16) << 8) | low_byte as u16;

        // Reset current instruction
        self.current_instruction = DEFAULT_INSTRUCTION_VARIANT;

        // Reset the cycles
        self.cycles = 0;

        Ok(())
    }

    /// Increment the program counter by 1, wrapping around on overflow
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.increment_pc();
    /// ```
    pub(crate) fn increment_pc(&mut self) {
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }

    /// Fetch the next operand byte from memory and increment the program counter
    ///
    /// # Returns
    /// * `Ok(u8)` containing the fetched operand
    /// * `Err(CpuError)` if there was an error reading from the bus
    ///
    /// # Errors
    /// * `CpuError::BusError` if there is an error reading from the bus
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// let operand = cpu.fetch_operand()?;
    /// ```
    pub(crate) fn fetch_operand(&mut self) -> Result<u8, CpuError> {
        let operand = self
            .bus
            .read(self.registers.program_counter)
            .map_err(CpuError::BusError)?;
        self.increment_pc();
        Ok(operand)
    }

    /// Push a byte onto the stack
    ///
    /// # Arguments
    /// * `value` - The byte value to push onto the stack
    ///
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(CpuError)` if there was an error writing to the bus
    ///
    /// # Errors
    /// * `CpuError::BusError` if there is an error writing to the bus
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.push_stack_data(0x42)?;
    /// ```
    pub(crate) fn push_stack_data(&mut self, value: u8) -> Result<(), CpuError> {
        self.bus
            .write(0x0100 + self.registers.stack_pointer as u16, value)
            .map_err(CpuError::BusError)?;
        Ok(())
    }

    /// Decrement the stack pointer (push operation)
    ///
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(CpuError)` if there was an error
    ///
    /// # Errors
    /// * `CpuError::Other` if the stack pointer underflow (should not happen in normal operation)
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.push_stack_ptr()?;
    /// ```
    pub(crate) fn push_stack_ptr(&mut self) -> Result<(), CpuError> {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
        Ok(())
    }

    /// Increment the stack pointer (pop operation)
    ///
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(CpuError)` if there was an error
    ///
    /// # Errors
    /// * `CpuError::Other` if the stack pointer overflows (should not happen in normal operation)
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.pop_stack_ptr()?;
    /// ```
    pub(crate) fn pop_stack_ptr(&mut self) -> Result<(), CpuError> {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        Ok(())
    }

    /// Pop a byte from the stack
    ///
    /// # Returns
    /// * `Ok(u8)` containing the byte popped from the stack
    /// * `Err(CpuError)` if there was an error reading from the bus
    ///
    /// # Errors
    /// * `CpuError::BusError` if there is an error reading from the bus
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// let value = cpu.pop_stack_data()?;
    /// ```
    pub(crate) fn pop_stack_data(&mut self) -> Result<u8, CpuError> {
        let value = self
            .bus
            .read(0x0100 + self.registers.stack_pointer as u16)
            .map_err(CpuError::BusError)?;
        Ok(value)
    }

    /// Execute a single CPU step (cycle)
    ///
    /// This function handles fetching the next instruction, managing cycles,
    /// and processing the current instruction.
    ///
    /// # Returns
    /// * `Ok(())` if the step was successful
    /// * `Err(CpuError)` if an error occurred during execution
    ///
    /// # Errors
    /// * `CpuError::UnknownInstruction` if the fetched opcode does not correspond to an instruction
    /// * `CpuError::BusError` if there is an error reading from or writing to the bus
    pub fn step(&mut self) -> Result<(), CpuError> {
        if self.page_boundary_cross_penalty > 0 {
            self.page_boundary_cross_penalty -= 1;
            self.cycles = self.cycles.wrapping_add(1);
            return Ok(());
        }

        match self.current_microcode_iter.next() {
            Some(microcode_step) => match microcode_step(self)? {
                OperationResult::Continue => {}
                OperationResult::PageBoundaryPenalty(cycles) => {
                    self.page_boundary_cross_penalty =
                        self.page_boundary_cross_penalty.wrapping_add(cycles);
                }
                OperationResult::Break => {
                    self.current_microcode_iter = [].iter();
                }
            },
            None => {
                let opcode = self.fetch_operand()?;
                match variant_by_opcode(opcode) {
                    Some(variant) => {
                        self.current_instruction = variant;
                        self.current_microcode_iter =
                            self.current_instruction.microcode_sequence.iter();
                    }
                    None => return Err(CpuError::UnknownInstruction),
                }
            }
        }

        self.cycles = self.cycles.wrapping_add(1);

        Ok(())
    }

    /// Update Zero and Negative flags based on the provided value
    ///
    /// # Arguments
    /// * `value` - The value to evaluate for flag updates
    ///
    /// # Effects
    /// * Updates the Zero flag if the value is zero
    /// * Updates the Negative flag based on the most significant bit of the value
    ///
    /// # Example
    /// ``` ignore
    /// let mut cpu = Cpu::new(bus);
    /// cpu.update_zero_negative_flags(0x00); // Sets Zero flag
    /// cpu.update_zero_negative_flags(0x80); // Sets Negative flag
    /// ```
    pub(crate) fn update_zero_negative_flags(&mut self, value: u8) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x80) != 0;
    }

    /// Determine if a page boundary was crossed between two addresses
    ///
    /// # Arguments
    /// * `start_address` - The starting address before the operation
    /// * `end_address` - The ending address after the operation
    ///
    /// # Returns
    /// * `true` if a page boundary was crossed, `false` otherwise
    ///
    /// # Example
    /// ``` ignore
    /// let crossed = cpu.determine_page_cross_penalty(0x10FF, 0x1100); // Returns true
    /// let not_crossed = cpu.determine_page_cross_penalty(0x10FE, 0x10FF); // Returns false
    /// ```
    pub(crate) fn determine_page_cross_penalty(
        &self,
        start_address: u16,
        end_address: u16,
    ) -> bool {
        (start_address & 0xFF00) != (end_address & 0xFF00)
    }
}
