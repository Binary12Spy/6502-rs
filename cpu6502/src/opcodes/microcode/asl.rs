//! Arithmetic Shift Left

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn accumulator_asl(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.accumulator = alu::asl(cpu.registers.accumulator, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

fn temp_data_asl(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = alu::asl(cpu.temp_data, &mut cpu.flags);
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    Ok(OperationResult::Continue)
}

pub(crate) static ACCUMULATOR: MicrocodeSequence<1> = [accumulator_asl];
pub(crate) static ZEROPAGE: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    temp_data_asl,
    common::temp_data_into_temp_address,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    common::temp_address_data_into_temp_data,
    temp_data_asl,
    common::temp_data_into_temp_address,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_data_into_temp_data,
    temp_data_asl,
    common::temp_data_into_temp_address,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<6> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_register,
    common::temp_address_data_into_temp_data,
    temp_data_asl,
    common::temp_data_into_temp_address,
];

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::test_cpu_builder::CpuBuilder;
    use ram::{Ram, ram_size::RamSize};

    /// Create a CPU with basic RAM setup for testing
    fn create_test_cpu() -> Cpu {
        let ram = Ram::new(RamSize::_32K, 0x0000);
        CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x7FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU")
    }

    /// Create a CPU with memory pre-populated with test data
    fn create_test_cpu_with_data(data: &[u8], start_address: u16) -> Cpu {
        let mut ram = Ram::new(RamSize::_32K, 0x0000);
        ram.import(data, start_address)
            .expect("Failed to import data");
        CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x7FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU")
    }

    // Test core ASL microcode functions
    #[test]
    fn test_accumulator_asl_basic() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b01010101; // 0x55

        let result = accumulator_asl(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0b10101010); // 0x55 << 1 = 0xAA
        assert!(!cpu.flags.carry); // No carry out from bit 7
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative); // Bit 7 is set
    }

    #[test]
    fn test_accumulator_asl_with_carry() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b10101010; // 0xAA

        let result = accumulator_asl(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0b01010100); // 0xAA << 1 = 0x54 (with carry)
        assert!(cpu.flags.carry); // Carry out from bit 7
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_accumulator_asl_zero_result() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b10000000; // 0x80

        let result = accumulator_asl(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0x00); // 0x80 << 1 = 0x00 (with carry)
        assert!(cpu.flags.carry); // Carry out from bit 7
        assert!(cpu.flags.zero); // Result is zero
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_temp_data_asl() {
        let mut cpu = create_test_cpu();
        cpu.temp_data = 0b00110011; // 0x33
        cpu.temp_address = 0x1000;

        let result = temp_data_asl(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.temp_data, 0b01100110); // 0x33 << 1 = 0x66

        // Verify data was written back to memory
        let memory_value = cpu.bus.read(0x1000).expect("Failed to read memory");
        assert_eq!(memory_value, 0x66);

        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_temp_data_asl_with_carry_and_write() {
        let mut cpu = create_test_cpu();
        cpu.temp_data = 0xFF;
        cpu.temp_address = 0x2000;

        let result = temp_data_asl(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.temp_data, 0xFE); // 0xFF << 1 = 0xFE (with carry)

        // Verify data was written back to memory
        let memory_value = cpu.bus.read(0x2000).expect("Failed to read memory");
        assert_eq!(memory_value, 0xFE);

        assert!(cpu.flags.carry); // Carry out from bit 7
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative); // Bit 7 is set
    }

    #[test]
    fn test_temp_data_asl_bus_error() {
        let ram = Ram::new(RamSize::_16K, 0x0000); // Only 16K (0x0000-0x3FFF)
        let mut cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x3FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        cpu.temp_data = 0x42;
        cpu.temp_address = 0x8000; // Outside RAM range

        let result = temp_data_asl(&mut cpu);

        assert!(result.is_err());
        match result.unwrap_err() {
            CpuError::BusError(_) => (),
            _ => panic!("Expected BusError"),
        }
    }

    // Test all addressing modes
    #[test]
    fn test_accumulator_addressing_mode() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0x40;

        // Execute ACCUMULATOR sequence
        for operation in ACCUMULATOR.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0x80); // 0x40 << 1
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x33], 0x0050);
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x50) // zero page address
            .expect("Failed to write operand");

        // Execute ZEROPAGE sequence
        for operation in ZEROPAGE.iter() {
            operation(&mut cpu).unwrap();
        }

        // Check memory was modified
        let result = cpu.bus.read(0x0050).expect("Failed to read result");
        assert_eq!(result, 0x66); // 0x33 << 1
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_x_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x0F], 0x0060); // data at 0x50 + 0x10
        cpu.registers.x = 0x10;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x50) // zero page base address
            .expect("Failed to write operand");

        // Execute ZEROPAGE_X sequence
        for operation in ZEROPAGE_X.iter() {
            operation(&mut cpu).unwrap();
        }

        // Check memory was modified
        let result = cpu.bus.read(0x0060).expect("Failed to read result");
        assert_eq!(result, 0x1E); // 0x0F << 1
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_x_addressing_mode_wraparound() {
        let mut cpu = create_test_cpu_with_data(&[0x21], 0x000F); // wraparound to 0x000F
        cpu.registers.x = 0x10;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0xFF) // 0xFF + 0x10 = 0x10F -> 0x0F in zero page
            .expect("Failed to write operand");

        // Execute ZEROPAGE_X sequence
        for operation in ZEROPAGE_X.iter() {
            operation(&mut cpu).unwrap();
        }

        // Check memory was modified
        let result = cpu.bus.read(0x000F).expect("Failed to read result");
        assert_eq!(result, 0x42); // 0x21 << 1
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_absolute_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x7F], 0x1234);
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x34) // little-endian address 0x1234
            .expect("Failed to write operand low");
        cpu.bus
            .write(0x1001, 0x12)
            .expect("Failed to write operand high");

        // Execute ABSOLUTE sequence
        for operation in ABSOLUTE.iter() {
            operation(&mut cpu).unwrap();
        }

        // Check memory was modified
        let result = cpu.bus.read(0x1234).expect("Failed to read result");
        assert_eq!(result, 0xFE); // 0x7F << 1
        assert!(!cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_absolute_x_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x81], 0x1239); // 0x1234 + 0x05
        cpu.registers.x = 0x05;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x34) // base address 0x1234
            .expect("Failed to write operand low");
        cpu.bus
            .write(0x1001, 0x12)
            .expect("Failed to write operand high");

        // Execute ABSOLUTE_X sequence
        for operation in ABSOLUTE_X.iter() {
            operation(&mut cpu).unwrap();
        }

        // Check memory was modified
        let result = cpu.bus.read(0x1239).expect("Failed to read result");
        assert_eq!(result, 0x02); // 0x81 << 1 = 0x02 (with carry)
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    // Test flag behavior extensively
    #[test]
    fn test_asl_flag_combinations() {
        let test_cases = [
            // (input, expected_output, expected_carry, expected_zero, expected_negative, desc)
            (0x00, 0x00, false, true, false, "zero input"),
            (0x01, 0x02, false, false, false, "simple shift no carry"),
            (0x40, 0x80, false, false, true, "shift to negative"),
            (0x80, 0x00, true, true, false, "carry out, zero result"),
            (0x81, 0x02, true, false, false, "carry out, positive result"),
            (0xFF, 0xFE, true, false, true, "all bits set"),
            (0x7F, 0xFE, false, false, true, "max positive to negative"),
            (0x55, 0xAA, false, false, true, "alternating bits"),
        ];

        for (input, expected_out, exp_carry, exp_zero, exp_neg, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu();
            cpu.registers.accumulator = *input;

            // Clear flags to known state
            cpu.flags.carry = false;
            cpu.flags.zero = false;
            cpu.flags.negative = false;
            cpu.flags.overflow = true; // Should be preserved

            accumulator_asl(&mut cpu).unwrap();

            assert_eq!(
                cpu.registers.accumulator, *expected_out,
                "Output failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.carry, *exp_carry,
                "Carry flag failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.zero, *exp_zero,
                "Zero flag failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.negative, *exp_neg,
                "Negative flag failed for case: {}",
                desc
            );

            // ASL should not affect overflow flag
            assert!(
                cpu.flags.overflow,
                "Overflow flag should be preserved for case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_asl_memory_operations() {
        // Test various memory locations and values
        let test_cases = [
            (0x0001, 0x42, 0x84, false, false, true, "zero page"),
            (0x1000, 0x55, 0xAA, false, false, true, "absolute"),
            (
                0x7FFE,
                0x80,
                0x00,
                true,
                true,
                false,
                "high memory with carry",
            ),
        ];

        for (addr, input, expected_out, exp_carry, exp_zero, exp_neg, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu_with_data(&[*input], *addr);
            cpu.temp_data = *input;
            cpu.temp_address = *addr;

            temp_data_asl(&mut cpu).unwrap();

            assert_eq!(
                cpu.temp_data, *expected_out,
                "temp_data failed for case: {}",
                desc
            );

            let memory_value = cpu.bus.read(*addr).expect("Failed to read memory");
            assert_eq!(
                memory_value, *expected_out,
                "Memory write failed for case: {}",
                desc
            );

            assert_eq!(
                cpu.flags.carry, *exp_carry,
                "Carry flag failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.zero, *exp_zero,
                "Zero flag failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.negative, *exp_neg,
                "Negative flag failed for case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_complete_asl_instruction_simulation() {
        // Test accumulator mode
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0x33;

        for operation in ACCUMULATOR.iter() {
            operation(&mut cpu).unwrap();
        }
        assert_eq!(cpu.registers.accumulator, 0x66);
        assert!(!cpu.flags.carry);

        // Test zero page mode
        let mut cpu = create_test_cpu_with_data(&[0x22], 0x0080);
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x80)
            .expect("Failed to write operand");

        for operation in ZEROPAGE.iter() {
            operation(&mut cpu).unwrap();
        }
        let result = cpu.bus.read(0x0080).expect("Failed to read result");
        assert_eq!(result, 0x44);
        assert!(!cpu.flags.carry);

        // Test absolute mode
        let mut cpu = create_test_cpu_with_data(&[0x88], 0x2000);
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x00)
            .expect("Failed to write operand low");
        cpu.bus
            .write(0x1001, 0x20)
            .expect("Failed to write operand high");

        for operation in ABSOLUTE.iter() {
            operation(&mut cpu).unwrap();
        }
        let result = cpu.bus.read(0x2000).expect("Failed to read result");
        assert_eq!(result, 0x10); // 0x88 << 1 = 0x10 (with carry)
        assert!(cpu.flags.carry);
    }

    #[test]
    fn test_asl_edge_cases() {
        // Test edge cases specific to ASL operation
        let edge_cases = [
            (0x00, "zero value"),
            (0xFF, "all bits set"),
            (0x80, "only sign bit"),
            (0x01, "only LSB"),
            (0x7F, "max positive"),
        ];

        for (value, desc) in edge_cases.iter() {
            // Test accumulator mode
            let mut cpu = create_test_cpu();
            cpu.registers.accumulator = *value;
            accumulator_asl(&mut cpu).unwrap();

            let expected = (*value as u16) << 1;
            let expected_acc = (expected & 0xFF) as u8;
            let expected_carry = expected > 0xFF;

            assert_eq!(
                cpu.registers.accumulator, expected_acc,
                "Accumulator failed for edge case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.carry, expected_carry,
                "Carry failed for edge case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.zero,
                expected_acc == 0,
                "Zero flag failed for edge case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.negative,
                (expected_acc & 0x80) != 0,
                "Negative flag failed for edge case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_bus_error_propagation() {
        // Test that bus errors are properly propagated through the microcode functions
        let ram = Ram::new(RamSize::_16K, 0x0000); // Only goes to 0x3FFF
        let mut cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x3FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        cpu.temp_data = 0x42;
        cpu.temp_address = 0x8000; // Outside RAM range

        let result = temp_data_asl(&mut cpu);
        assert!(result.is_err());

        match result.unwrap_err() {
            CpuError::BusError(_) => (),
            _ => panic!("Expected bus error"),
        }
    }

    #[test]
    fn test_asl_preserves_unaffected_flags() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0x42;

        // Set flags that should be preserved
        cpu.flags.interrupt_disable = true;
        cpu.flags.decimal_mode = true;
        cpu.flags.break_command = true;
        cpu.flags.overflow = true;

        accumulator_asl(&mut cpu).unwrap();

        // These flags should not be affected by ASL
        assert!(
            cpu.flags.interrupt_disable,
            "Interrupt disable should be preserved"
        );
        assert!(cpu.flags.decimal_mode, "Decimal mode should be preserved");
        assert!(cpu.flags.break_command, "Break command should be preserved");
        assert!(cpu.flags.overflow, "Overflow should be preserved");
    }
}
