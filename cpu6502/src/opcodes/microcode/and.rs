//! Logical AND

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn operand_and_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    cpu.registers.accumulator = alu::and(cpu.registers.accumulator, cpu.temp_data, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

fn accumulator_and_temp_address_data(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.registers.accumulator = alu::and(cpu.registers.accumulator, cpu.temp_data, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

pub(crate) static IMMEDIATE: MicrocodeSequence<1> = [operand_and_accumulator];
pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    accumulator_and_temp_address_data,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    accumulator_and_temp_address_data,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    accumulator_and_temp_address_data,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_page_boundary_check,
    accumulator_and_temp_address_data,
];
pub(crate) static ABSOLUTE_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_y_page_boundary_check,
    accumulator_and_temp_address_data,
];
pub(crate) static INDIRECT_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register,
    common::temp_address_data_into_temp_data,
    common::temp_data_low_and_temp_address_inc_high_into_temp_address,
    accumulator_and_temp_address_data,
];
pub(crate) static INDIRECT_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    common::temp_address_inc_data_as_temp_address_high_add_y_page_boundary_check,
    accumulator_and_temp_address_data,
];

#[cfg(test)]
mod tests {
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

    // Test core AND microcode functions
    #[test]
    fn test_operand_and_accumulator_basic() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b11110000;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0b10101010) // operand
            .expect("Failed to write operand");

        let result = operand_and_accumulator(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0b10100000); // 0b11110000 & 0b10101010
        assert_eq!(cpu.temp_data, 0b10101010);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative); // bit 7 is set
    }

    #[test]
    fn test_operand_and_accumulator_zero_result() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b00001111;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0b11110000) // operand
            .expect("Failed to write operand");

        let result = operand_and_accumulator(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0b00000000); // 0b00001111 & 0b11110000
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_operand_and_accumulator_all_bits_set() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0xFF;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x42) // operand
            .expect("Failed to write operand");

        let result = operand_and_accumulator(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0x42); // 0xFF & 0x42
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_accumulator_and_temp_address_data() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b11001100;
        cpu.temp_address = 0x1000;
        cpu.bus
            .write(0x1000, 0b10011001)
            .expect("Failed to write test data");

        let result = accumulator_and_temp_address_data(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.accumulator, 0b10001000); // 0b11001100 & 0b10011001
        assert_eq!(cpu.temp_data, 0b10011001);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative); // bit 7 is set
    }

    #[test]
    fn test_accumulator_and_temp_address_data_bus_error() {
        let ram = Ram::new(RamSize::_16K, 0x0000); // Only 16K (0x0000-0x3FFF)
        let mut cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x3FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        cpu.registers.accumulator = 0xFF;
        cpu.temp_address = 0x8000; // Outside of RAM range

        let result = accumulator_and_temp_address_data(&mut cpu);

        assert!(result.is_err());
        match result.unwrap_err() {
            CpuError::BusError(_) => (),
            _ => panic!("Expected BusError"),
        }
    }

    // Test all addressing modes
    #[test]
    fn test_immediate_addressing_mode() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0b11110000;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0b01010101) // immediate operand
            .expect("Failed to write operand");

        // Execute IMMEDIATE sequence
        for operation in IMMEDIATE.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0b01010000); // 0b11110000 & 0b01010101
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0b10011001], 0x0050); // data at zero page
        cpu.registers.accumulator = 0b11001100;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x50) // zero page address
            .expect("Failed to write operand");

        // Execute ZEROPAGE sequence
        for operation in ZEROPAGE.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0b10001000); // 0b11001100 & 0b10011001
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_x_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0b00000001], 0x0060); // data at 0x50 + 0x10
        cpu.registers.accumulator = 0b11111111;
        cpu.registers.x = 0x10;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x50) // zero page base address
            .expect("Failed to write operand");

        // Execute ZEROPAGE_X sequence
        for operation in ZEROPAGE_X.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0b00000001); // 0b11111111 & 0b00000001
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_zeropage_x_addressing_mode_wraparound() {
        let mut cpu = create_test_cpu_with_data(&[0x42], 0x000F); // wraparound to 0x000F
        cpu.registers.accumulator = 0xFF;
        cpu.registers.x = 0x10;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0xFF) // 0xFF + 0x10 = 0x10F -> 0x0F in zero page
            .expect("Failed to write operand");

        // Execute ZEROPAGE_X sequence
        for operation in ZEROPAGE_X.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0x42);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_absolute_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0b01010101], 0x1234);
        cpu.registers.accumulator = 0b10101010;
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

        assert_eq!(cpu.registers.accumulator, 0b00000000); // 0b10101010 & 0b01010101
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_absolute_x_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x80], 0x1239); // 0x1234 + 0x05
        cpu.registers.accumulator = 0xFF;
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

        assert_eq!(cpu.registers.accumulator, 0x80);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn test_absolute_y_addressing_mode() {
        let mut cpu = create_test_cpu_with_data(&[0x3F], 0x123E); // 0x1234 + 0x0A
        cpu.registers.accumulator = 0x7F;
        cpu.registers.y = 0x0A;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x34) // base address 0x1234
            .expect("Failed to write operand low");
        cpu.bus
            .write(0x1001, 0x12)
            .expect("Failed to write operand high");

        // Execute ABSOLUTE_Y sequence
        for operation in ABSOLUTE_Y.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0x3F); // 0x7F & 0x3F
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_indirect_x_addressing_mode() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0xAA;
        cpu.registers.x = 0x04;
        cpu.registers.program_counter = 0x1000;

        // Set up zero page indirect addressing
        cpu.bus
            .write(0x0024, 0x00)
            .expect("Failed to write indirect low"); // low byte of indirect address at 0x20 + 0x04
        cpu.bus
            .write(0x0025, 0x30)
            .expect("Failed to write indirect high"); // high byte of indirect address
        cpu.bus.write(0x3000, 0x55).expect("Failed to write data"); // actual data at indirect address
        cpu.bus
            .write(0x1000, 0x20)
            .expect("Failed to write operand"); // zero page base address

        // Execute INDIRECT_X sequence
        for operation in INDIRECT_X.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0x00); // 0xAA & 0x55
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    #[test]
    fn test_indirect_y_addressing_mode() {
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0xFF;
        cpu.registers.y = 0x10;
        cpu.registers.program_counter = 0x1000;

        // Set up zero page indirect addressing
        cpu.bus
            .write(0x0020, 0x00)
            .expect("Failed to write indirect low"); // low byte of base address
        cpu.bus
            .write(0x0021, 0x30)
            .expect("Failed to write indirect high"); // high byte of base address
        cpu.bus.write(0x3010, 0x0F).expect("Failed to write data"); // actual data at 0x3000 + 0x10
        cpu.bus
            .write(0x1000, 0x20)
            .expect("Failed to write operand"); // zero page pointer

        // Execute INDIRECT_Y sequence
        for operation in INDIRECT_Y.iter() {
            operation(&mut cpu).unwrap();
        }

        assert_eq!(cpu.registers.accumulator, 0x0F); // 0xFF & 0x0F
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
    }

    // Test flag behavior
    #[test]
    fn test_and_flag_combinations() {
        // Test all flag combinations that should be preserved
        let test_cases = [
            // (accumulator, operand, expected_result, expected_zero, expected_negative, desc)
            (0xFF, 0xFF, 0xFF, false, true, "all bits set"),
            (
                0x00,
                0xFF,
                0x00,
                true,
                false,
                "zero result from zero accumulator",
            ),
            (
                0xFF,
                0x00,
                0x00,
                true,
                false,
                "zero result from zero operand",
            ),
            (0x80, 0x80, 0x80, false, true, "negative result"),
            (0x7F, 0x7F, 0x7F, false, false, "positive result"),
            (0x55, 0xAA, 0x00, true, false, "complementary patterns"),
            (0x0F, 0xF0, 0x00, true, false, "nibble separation"),
            (0x01, 0x01, 0x01, false, false, "single bit preservation"),
        ];

        for (acc, operand, expected, exp_zero, exp_neg, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu();
            cpu.registers.accumulator = *acc;
            cpu.registers.program_counter = 0x1000;
            cpu.bus
                .write(0x1000, *operand)
                .expect("Failed to write operand");

            // Clear flags to known state
            cpu.flags.zero = false;
            cpu.flags.negative = false;
            cpu.flags.carry = true; // Should be preserved
            cpu.flags.overflow = true; // Should be preserved

            operand_and_accumulator(&mut cpu).unwrap();

            assert_eq!(
                cpu.registers.accumulator, *expected,
                "Failed for case: {}",
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

            // AND should not affect carry and overflow flags
            assert!(
                cpu.flags.carry,
                "Carry flag should be preserved for case: {}",
                desc
            );
            assert!(
                cpu.flags.overflow,
                "Overflow flag should be preserved for case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_page_boundary_penalty_simulation() {
        // Test absolute,X with page boundary crossing
        let mut cpu = create_test_cpu_with_data(&[0x42], 0x20FE); // 0x1FFF + 0xFF crosses page
        cpu.registers.accumulator = 0xFF;
        cpu.registers.x = 0xFF;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0xFF) // address 0x1FFF
            .expect("Failed to write operand low");
        cpu.bus
            .write(0x1001, 0x1F)
            .expect("Failed to write operand high");

        // Execute ABSOLUTE_X sequence
        for operation in ABSOLUTE_X.iter() {
            let result = operation(&mut cpu).unwrap();
            // Page boundary check should add penalty cycle
            if let OperationResult::PageBoundaryPenalty(_) = result {
                // Expected for page crossing
            }
        }

        assert_eq!(cpu.registers.accumulator, 0x42);
    }

    #[test]
    fn test_and_edge_cases() {
        // Test edge cases specific to AND operation
        let test_cases = [
            (0x00, 0x00, "zero and zero"),
            (0xFF, 0x00, "all bits and zero"),
            (0x00, 0xFF, "zero and all bits"),
            (0x80, 0x7F, "sign bit isolation"),
            (0x01, 0xFE, "LSB isolation"),
        ];

        for (acc, operand, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu();
            cpu.registers.accumulator = *acc;
            cpu.temp_address = 0x1000;
            cpu.bus
                .write(0x1000, *operand)
                .expect("Failed to write test data");

            let expected = *acc & *operand;
            accumulator_and_temp_address_data(&mut cpu).unwrap();

            assert_eq!(
                cpu.registers.accumulator, expected,
                "Failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.zero,
                expected == 0,
                "Zero flag failed for case: {}",
                desc
            );
            assert_eq!(
                cpu.flags.negative,
                (expected & 0x80) != 0,
                "Negative flag failed for case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_complete_and_instruction_simulation() {
        // Test immediate mode
        let mut cpu = create_test_cpu();
        cpu.registers.accumulator = 0xFF;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0xAA)
            .expect("Failed to write operand");

        for operation in IMMEDIATE.iter() {
            operation(&mut cpu).unwrap();
        }
        assert_eq!(cpu.registers.accumulator, 0xAA); // 0xFF & 0xAA
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);

        // Test zero page mode
        let mut cpu = create_test_cpu_with_data(&[0x55], 0x0050);
        cpu.registers.accumulator = 0xFF;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x50)
            .expect("Failed to write operand"); // Zero page address

        for operation in ZEROPAGE.iter() {
            operation(&mut cpu).unwrap();
        }
        assert_eq!(cpu.registers.accumulator, 0x55); // 0xFF & 0x55
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);

        // Test absolute mode
        let mut cpu = create_test_cpu_with_data(&[0x33], 0x2000);
        cpu.registers.accumulator = 0xFF;
        cpu.registers.program_counter = 0x1000;
        cpu.bus
            .write(0x1000, 0x00)
            .expect("Failed to write operand low"); // $2000
        cpu.bus
            .write(0x1001, 0x20)
            .expect("Failed to write operand high");

        for operation in ABSOLUTE.iter() {
            operation(&mut cpu).unwrap();
        }
        assert_eq!(cpu.registers.accumulator, 0x33); // 0xFF & 0x33
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.negative);
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

        cpu.registers.accumulator = 0xFF;
        cpu.temp_address = 0x8000; // Outside RAM range

        let result = accumulator_and_temp_address_data(&mut cpu);
        assert!(result.is_err());

        match result.unwrap_err() {
            CpuError::BusError(_) => (),
            _ => panic!("Expected bus error"),
        }
    }
}
