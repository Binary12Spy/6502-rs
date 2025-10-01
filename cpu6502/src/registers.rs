///! Registers for the 6502 CPU

/// The 6502 CPU has the following main registers:
/// - Accumulator (A): 8-bit register used for arithmetic and logic operations.
/// - X Register (X): 8-bit register used for indexing and loop counters.
/// - Y Register (Y): 8-bit register used for indexing and loop counters.
/// - Program Counter (PC): 16-bit register that holds the address of the next instruction to be executed.
/// - Stack Pointer (SP): 8-bit register that points to the current position in the stack (stack is located in page 1, i.e., addresses $0100 to $01FF).
#[derive(Clone, Copy, Debug)]
pub struct Registers {
    /// Accumulator
    pub accumulator: u8,
    /// X Register
    pub x: u8,
    /// Y Register
    pub y: u8,
    /// Program Counter
    pub program_counter: u16,
    /// Stack Pointer
    pub stack_pointer: u8,
}

impl Default for Registers {
    /// Create a new Registers instance with default values.
    ///
    /// Defaults:
    /// - Accumulator: 0x00
    /// - X Register: 0x00
    /// - Y Register: 0x00
    /// - Program Counter: 0xFFFC (reset vector)
    /// - Stack Pointer: 0xFD (initial stack pointer)
    fn default() -> Registers {
        Registers {
            accumulator: 0x00,
            x: 0x00,
            y: 0x00,
            program_counter: 0xFFFC,
            stack_pointer: 0xFD,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    // Test Default implementation
    #[test]
    fn test_registers_default() {
        let registers = Registers::default();

        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.x, 0x00);
        assert_eq!(registers.y, 0x00);
        assert_eq!(registers.program_counter, 0xFFFC); // Reset vector
        assert_eq!(registers.stack_pointer, 0xFD); // Initial stack pointer
    }

    // Test individual register initialization
    #[test]
    fn test_registers_individual_initialization() {
        let registers = Registers {
            accumulator: 0x42,
            x: 0x33,
            y: 0x77,
            program_counter: 0x8000,
            stack_pointer: 0xFF,
        };

        assert_eq!(registers.accumulator, 0x42);
        assert_eq!(registers.x, 0x33);
        assert_eq!(registers.y, 0x77);
        assert_eq!(registers.program_counter, 0x8000);
        assert_eq!(registers.stack_pointer, 0xFF);
    }

    // Test 8-bit register boundary values
    #[test]
    fn test_8bit_register_boundaries() {
        let mut registers = Registers::default();

        // Test minimum values (0x00)
        registers.accumulator = 0x00;
        registers.x = 0x00;
        registers.y = 0x00;
        registers.stack_pointer = 0x00;

        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.x, 0x00);
        assert_eq!(registers.y, 0x00);
        assert_eq!(registers.stack_pointer, 0x00);

        // Test maximum values (0xFF)
        registers.accumulator = 0xFF;
        registers.x = 0xFF;
        registers.y = 0xFF;
        registers.stack_pointer = 0xFF;

        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.x, 0xFF);
        assert_eq!(registers.y, 0xFF);
        assert_eq!(registers.stack_pointer, 0xFF);
    }

    // Test 16-bit program counter boundary values
    #[test]
    fn test_program_counter_boundaries() {
        let mut registers = Registers::default();

        // Test minimum value
        registers.program_counter = 0x0000;
        assert_eq!(registers.program_counter, 0x0000);

        // Test maximum value
        registers.program_counter = 0xFFFF;
        assert_eq!(registers.program_counter, 0xFFFF);

        // Test typical 6502 memory ranges
        registers.program_counter = 0x0200; // Typical program start
        assert_eq!(registers.program_counter, 0x0200);

        registers.program_counter = 0x8000; // ROM area
        assert_eq!(registers.program_counter, 0x8000);
    }

    // Test 6502-specific register behavior
    #[test]
    fn test_6502_specific_values() {
        let registers = Registers::default();

        // Test reset vector (where PC starts after reset)
        assert_eq!(registers.program_counter, 0xFFFC);

        // Test initial stack pointer (6502 starts with SP = 0xFD)
        assert_eq!(registers.stack_pointer, 0xFD);

        // Verify stack is in page 1 (0x0100-0x01FF)
        // Stack pointer + 0x0100 should give actual stack address
        let actual_stack_address = 0x0100 + (registers.stack_pointer as u16);
        assert_eq!(actual_stack_address, 0x01FD);
        assert!(actual_stack_address >= 0x0100 && actual_stack_address <= 0x01FF);
    }

    // Test stack pointer behavior
    #[test]
    fn test_stack_pointer_behavior() {
        let mut registers = Registers::default();

        // Test stack operations (simulated)
        // Push operation would decrement SP
        registers.stack_pointer = registers.stack_pointer.wrapping_sub(1);
        assert_eq!(registers.stack_pointer, 0xFC);

        // Another push
        registers.stack_pointer = registers.stack_pointer.wrapping_sub(1);
        assert_eq!(registers.stack_pointer, 0xFB);

        // Pop operation would increment SP
        registers.stack_pointer = registers.stack_pointer.wrapping_add(1);
        assert_eq!(registers.stack_pointer, 0xFC);

        // Test stack underflow (wrapping)
        registers.stack_pointer = 0x00;
        registers.stack_pointer = registers.stack_pointer.wrapping_sub(1);
        assert_eq!(registers.stack_pointer, 0xFF);

        // Test stack overflow (wrapping)
        registers.stack_pointer = 0xFF;
        registers.stack_pointer = registers.stack_pointer.wrapping_add(1);
        assert_eq!(registers.stack_pointer, 0x00);
    }

    // Test program counter behavior
    #[test]
    fn test_program_counter_behavior() {
        let mut registers = Registers::default();

        // Test typical instruction fetch (PC increment)
        registers.program_counter = registers.program_counter.wrapping_add(1);
        assert_eq!(registers.program_counter, 0xFFFD);

        // Test jump to specific address
        registers.program_counter = 0x8000;
        assert_eq!(registers.program_counter, 0x8000);

        // Test program counter overflow (wrapping)
        registers.program_counter = 0xFFFF;
        registers.program_counter = registers.program_counter.wrapping_add(1);
        assert_eq!(registers.program_counter, 0x0000);

        // Test relative branch calculation (simulated)
        registers.program_counter = 0x1000;
        let branch_offset: i8 = -10; // Negative branch
        registers.program_counter =
            (registers.program_counter as i32 + branch_offset as i32) as u16;
        assert_eq!(registers.program_counter, 0x0FF6);
    }

    // Test register independence
    #[test]
    fn test_register_independence() {
        let mut registers = Registers::default();

        // Modify each register and ensure others are unchanged
        let original_x = registers.x;
        let original_y = registers.y;
        let original_pc = registers.program_counter;
        let original_sp = registers.stack_pointer;

        registers.accumulator = 0xAA;
        assert_eq!(registers.x, original_x);
        assert_eq!(registers.y, original_y);
        assert_eq!(registers.program_counter, original_pc);
        assert_eq!(registers.stack_pointer, original_sp);

        registers.x = 0xBB;
        assert_eq!(registers.accumulator, 0xAA); // Should remain unchanged
        assert_eq!(registers.y, original_y);
        assert_eq!(registers.program_counter, original_pc);
        assert_eq!(registers.stack_pointer, original_sp);
    }

    // Test typical 6502 register usage patterns
    #[test]
    fn test_typical_6502_usage_patterns() {
        let mut registers = Registers::default();

        // Simulate LDA #$42 (Load Accumulator with immediate value)
        registers.accumulator = 0x42;
        assert_eq!(registers.accumulator, 0x42);

        // Simulate TAX (Transfer Accumulator to X)
        registers.x = registers.accumulator;
        assert_eq!(registers.x, 0x42);
        assert_eq!(registers.accumulator, 0x42); // A should remain unchanged

        // Simulate INX (Increment X)
        registers.x = registers.x.wrapping_add(1);
        assert_eq!(registers.x, 0x43);

        // Simulate LDY #$10 (Load Y with immediate value)
        registers.y = 0x10;
        assert_eq!(registers.y, 0x10);

        // Simulate indexed addressing preparation
        let base_address = 0x2000u16;
        let indexed_address = base_address.wrapping_add(registers.x as u16);
        assert_eq!(indexed_address, 0x2043);
    }

    // Test Clone and Copy traits
    #[test]
    fn test_registers_clone_copy() {
        let original_registers = Registers {
            accumulator: 0x11,
            x: 0x22,
            y: 0x33,
            program_counter: 0x1234,
            stack_pointer: 0x44,
        };

        // Test copy
        let copied_registers = original_registers;
        assert_eq!(copied_registers.accumulator, 0x11);
        assert_eq!(copied_registers.x, 0x22);
        assert_eq!(copied_registers.y, 0x33);
        assert_eq!(copied_registers.program_counter, 0x1234);
        assert_eq!(copied_registers.stack_pointer, 0x44);

        // Test clone
        let cloned_registers = original_registers.clone();
        assert_eq!(cloned_registers.accumulator, 0x11);
        assert_eq!(cloned_registers.x, 0x22);
        assert_eq!(cloned_registers.y, 0x33);
        assert_eq!(cloned_registers.program_counter, 0x1234);
        assert_eq!(cloned_registers.stack_pointer, 0x44);

        // Test independence after copy
        let mut modified_registers = original_registers;
        modified_registers.accumulator = 0x99;
        assert_eq!(original_registers.accumulator, 0x11); // Original unchanged
        assert_eq!(modified_registers.accumulator, 0x99); // Copy changed
    }

    // Test Debug trait
    #[test]
    fn test_registers_debug() {
        let registers = Registers {
            accumulator: 0xAA,
            x: 0xBB,
            y: 0xCC,
            program_counter: 0x1234,
            stack_pointer: 0xDD,
        };

        let debug_string = format!("{:?}", registers);
        assert!(debug_string.contains("accumulator"));
        assert!(debug_string.contains("170")); // 0xAA in decimal
        assert!(debug_string.contains("x"));
        assert!(debug_string.contains("187")); // 0xBB in decimal
        assert!(debug_string.contains("program_counter"));
        assert!(debug_string.contains("4660")); // 0x1234 in decimal
        assert!(debug_string.contains("Registers"));
    }

    // Test realistic 6502 program scenarios
    #[test]
    fn test_realistic_6502_scenarios() {
        let mut registers = Registers::default();

        // Scenario 1: Simple counter program
        registers.program_counter = 0x0800; // Program starts at $0800
        registers.x = 0x00; // Counter starts at 0

        // Simulate a loop that increments X 5 times
        for i in 0..5 {
            registers.x = registers.x.wrapping_add(1);
            registers.program_counter = registers.program_counter.wrapping_add(2); // Assume 2-byte instructions
            assert_eq!(registers.x, i + 1);
        }
        assert_eq!(registers.x, 5);
        assert_eq!(registers.program_counter, 0x080A); // 0x0800 + 5*2

        // Scenario 2: Stack operations
        registers.stack_pointer = 0xFD; // Reset to initial value

        // Simulate PHA (Push Accumulator) - SP decrements
        registers.accumulator = 0x42;
        registers.stack_pointer = registers.stack_pointer.wrapping_sub(1);
        let stack_addr = 0x0100 + registers.stack_pointer as u16;
        assert_eq!(stack_addr, 0x01FC); // Stack address where A would be stored

        // Simulate PLA (Pull Accumulator) - SP increments
        registers.stack_pointer = registers.stack_pointer.wrapping_add(1);
        assert_eq!(registers.stack_pointer, 0xFD); // Back to original
    }

    // Test edge cases and error conditions
    #[test]
    fn test_edge_cases() {
        let mut registers = Registers::default();

        // Test all registers at maximum values
        registers.accumulator = 0xFF;
        registers.x = 0xFF;
        registers.y = 0xFF;
        registers.program_counter = 0xFFFF;
        registers.stack_pointer = 0xFF;

        // Test operations that would overflow
        let new_a = registers.accumulator.wrapping_add(1);
        assert_eq!(new_a, 0x00); // Wrapped around

        let new_pc = registers.program_counter.wrapping_add(1);
        assert_eq!(new_pc, 0x0000); // Wrapped around

        // Test stack at boundaries
        registers.stack_pointer = 0x00;
        let stack_addr = 0x0100 + registers.stack_pointer as u16;
        assert_eq!(stack_addr, 0x0100); // Bottom of stack page

        registers.stack_pointer = 0xFF;
        let stack_addr = 0x0100 + registers.stack_pointer as u16;
        assert_eq!(stack_addr, 0x01FF); // Top of stack page
    }
}
