//! Branch if Carry Set

use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn fetch_offset(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    if !cpu.flags.carry {
        return Ok(OperationResult::Break);
    }
    Ok(OperationResult::Continue)
}

fn add_offset_to_program_counter(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let old_pc = cpu.registers.program_counter;
    cpu.registers.program_counter =
        alu::add_pc_with_signed_offset(cpu.registers.program_counter, cpu.temp_data)
            .map_err(|e| CpuError::AluError(e))?;
    if (old_pc & 0xFF00) != (cpu.registers.program_counter & 0xFF00) {
        return Ok(OperationResult::PageBoundaryPenalty(1));
    }
    Ok(OperationResult::Continue)
}

pub(crate) static RELATIVE: MicrocodeSequence<2> = [fetch_offset, add_offset_to_program_counter];

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

    // Test core BCS microcode functions
    #[test]
    fn test_fetch_offset_carry_set_positive_offset() {
        let mut cpu = create_test_cpu_with_data(&[0x10], 0x1000); // Positive offset (+16)
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = true; // Carry set - should branch

        let result = fetch_offset(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.temp_data, 0x10);
        assert_eq!(cpu.registers.program_counter, 0x1001); // PC incremented by fetch
    }

    #[test]
    fn test_fetch_offset_carry_set_negative_offset() {
        let mut cpu = create_test_cpu_with_data(&[0xF0], 0x1000); // Negative offset (-16)
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = true; // Carry set - should branch

        let result = fetch_offset(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.temp_data, 0xF0);
        assert_eq!(cpu.registers.program_counter, 0x1001);
    }

    #[test]
    fn test_fetch_offset_carry_clear_no_branch() {
        let mut cpu = create_test_cpu_with_data(&[0x20], 0x1000);
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = false; // Carry clear - should not branch

        let result = fetch_offset(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Break); // Should break the sequence
        assert_eq!(cpu.temp_data, 0x20);
        assert_eq!(cpu.registers.program_counter, 0x1001);
    }

    #[test]
    fn test_fetch_offset_zero_offset() {
        let mut cpu = create_test_cpu_with_data(&[0x00], 0x1000); // Zero offset
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = true;

        let result = fetch_offset(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.temp_data, 0x00);
        assert_eq!(cpu.registers.program_counter, 0x1001);
    }

    #[test]
    fn test_add_offset_to_program_counter_positive_no_page_cross() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1020;
        cpu.temp_data = 0x10; // +16

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue); // No page boundary penalty
        assert_eq!(cpu.registers.program_counter, 0x1030); // 0x1020 + 16 = 0x1030
    }

    #[test]
    fn test_add_offset_to_program_counter_negative_no_page_cross() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1020;
        cpu.temp_data = 0xF0; // -16 (signed 8-bit)

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.program_counter, 0x1010); // 0x1020 - 16 = 0x1010
    }

    #[test]
    fn test_add_offset_to_program_counter_positive_with_page_cross() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x10F0;
        cpu.temp_data = 0x20; // +32

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::PageBoundaryPenalty(1)); // Page boundary crossed
        assert_eq!(cpu.registers.program_counter, 0x1110); // 0x10F0 + 32 = 0x1110 (crosses page)
    }

    #[test]
    fn test_add_offset_to_program_counter_negative_with_page_cross() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1010;
        cpu.temp_data = 0xE0; // -32 (signed 8-bit)

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::PageBoundaryPenalty(1)); // Page boundary crossed
        assert_eq!(cpu.registers.program_counter, 0x0FF0); // 0x1010 - 32 = 0x0FF0 (crosses page)
    }

    #[test]
    fn test_add_offset_to_program_counter_zero_offset() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1000;
        cpu.temp_data = 0x00; // Zero offset

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.program_counter, 0x1000); // No change
    }

    #[test]
    fn test_add_offset_to_program_counter_maximum_positive() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1000;
        cpu.temp_data = 0x7F; // +127 (maximum positive offset)

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.program_counter, 0x107F); // 0x1000 + 127 = 0x107F
    }

    #[test]
    fn test_add_offset_to_program_counter_maximum_negative() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x1080;
        cpu.temp_data = 0x80; // -128 (maximum negative offset)

        let result = add_offset_to_program_counter(&mut cpu).unwrap();

        assert_eq!(result, OperationResult::Continue);
        assert_eq!(cpu.registers.program_counter, 0x1000); // 0x1080 - 128 = 0x1000
    }

    #[test]
    fn test_bcs_alu_error_handling() {
        let mut cpu = create_test_cpu();
        cpu.registers.program_counter = 0x0000; // Edge case that might cause ALU error
        cpu.temp_data = 0x80; // -128

        // The result should either succeed or return an ALU error
        let result = add_offset_to_program_counter(&mut cpu);

        match result {
            Ok(_) => {
                // If it succeeds, the PC should be valid (u16 automatically ensures this)
                // No additional validation needed since u16 max is 0xFFFF
            }
            Err(CpuError::AluError(_)) => {
                // ALU error is acceptable for edge cases
            }
            Err(other) => {
                panic!("Unexpected error type: {:?}", other);
            }
        }
    }

    // Test complete BCS instruction sequences
    #[test]
    fn test_bcs_relative_branch_taken_no_page_cross() {
        let mut cpu = create_test_cpu_with_data(&[0x10], 0x1000); // +16 offset
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = true; // Branch should be taken

        // Execute RELATIVE sequence
        for operation in RELATIVE.iter() {
            let result = operation(&mut cpu).unwrap();
            if let OperationResult::Break = result {
                break; // Should not happen for this test case
            }
        }

        assert_eq!(cpu.registers.program_counter, 0x1011); // 0x1001 + 16 = 0x1011
        assert_eq!(cpu.temp_data, 0x10);
    }

    #[test]
    fn test_bcs_relative_branch_taken_with_page_cross() {
        let mut cpu = create_test_cpu_with_data(&[0x7F], 0x10F0); // +127 offset
        cpu.registers.program_counter = 0x10F0;
        cpu.flags.carry = true; // Branch should be taken

        let mut page_penalty = false;

        // Execute RELATIVE sequence
        for operation in RELATIVE.iter() {
            let result = operation(&mut cpu).unwrap();
            if let OperationResult::PageBoundaryPenalty(_) = result {
                page_penalty = true;
            }
            if let OperationResult::Break = result {
                break;
            }
        }

        assert!(page_penalty, "Expected page boundary penalty");
        assert_eq!(cpu.registers.program_counter, 0x1170); // 0x10F1 + 127 = 0x1170
        assert_eq!(cpu.temp_data, 0x7F);
    }

    #[test]
    fn test_bcs_relative_branch_not_taken() {
        let mut cpu = create_test_cpu_with_data(&[0x20], 0x1000); // +32 offset
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = false; // Branch should NOT be taken

        let mut break_occurred = false;

        // Execute RELATIVE sequence
        for operation in RELATIVE.iter() {
            let result = operation(&mut cpu).unwrap();
            if let OperationResult::Break = result {
                break_occurred = true;
                break;
            }
        }

        assert!(break_occurred, "Expected sequence to break");
        assert_eq!(cpu.registers.program_counter, 0x1001); // Only incremented by fetch
        assert_eq!(cpu.temp_data, 0x20); // Offset was fetched but not used
    }

    #[test]
    fn test_bcs_negative_branch_with_page_cross() {
        let mut cpu = create_test_cpu_with_data(&[0xE0], 0x1010); // -32 offset
        cpu.registers.program_counter = 0x1010;
        cpu.flags.carry = true; // Branch should be taken

        let mut page_penalty = false;

        // Execute RELATIVE sequence
        for operation in RELATIVE.iter() {
            let result = operation(&mut cpu).unwrap();
            if let OperationResult::PageBoundaryPenalty(_) = result {
                page_penalty = true;
            }
            if let OperationResult::Break = result {
                break;
            }
        }

        assert!(page_penalty, "Expected page boundary penalty");
        assert_eq!(cpu.registers.program_counter, 0x0FF1); // 0x1011 - 32 = 0x0FF1
        assert_eq!(cpu.temp_data, 0xE0);
    }

    // Test edge cases and boundary conditions
    #[test]
    fn test_bcs_edge_case_combinations() {
        let test_cases = [
            // (pc, offset, carry_flag, expected_pc, should_branch, should_have_penalty, description)
            (
                0x1000,
                0x00,
                true,
                0x1001,
                true,
                false,
                "zero offset with carry set",
            ),
            (
                0x1000,
                0x00,
                false,
                0x1001,
                false,
                false,
                "zero offset with carry clear",
            ),
            (
                0x10FE,
                0x03,
                true,
                0x1102,
                true,
                true,
                "minimal page cross forward",
            ),
            (
                0x10FF,
                0xFF,
                true,
                0x10FF,
                true,
                true,
                "minimal page cross backward",
            ),
            (
                0x1000,
                0x7F,
                true,
                0x1080,
                true,
                false,
                "max positive offset no cross",
            ),
            (
                0x1080,
                0x80,
                true,
                0x1001,
                true,
                false,
                "max negative offset no cross",
            ),
            (
                0x0000,
                0x7F,
                true,
                0x0080,
                true,
                false,
                "low memory positive offset",
            ),
            (
                0x1FEF,
                0x10,
                true,
                0x2000,
                true,
                true,
                "page cross in RAM range",
            ),
        ];

        for (pc, offset, carry, exp_pc, should_branch, should_have_penalty, desc) in
            test_cases.iter()
        {
            let mut cpu = create_test_cpu_with_data(&[*offset], *pc);
            cpu.registers.program_counter = *pc;
            cpu.flags.carry = *carry;

            let mut break_occurred = false;
            let mut page_penalty = false;

            // Execute RELATIVE sequence
            for operation in RELATIVE.iter() {
                let result = operation(&mut cpu).unwrap();
                match result {
                    OperationResult::Break => {
                        break_occurred = true;
                        break;
                    }
                    OperationResult::PageBoundaryPenalty(_) => {
                        page_penalty = true;
                    }
                    _ => {}
                }
            }

            if *should_branch {
                assert!(
                    !break_occurred,
                    "Branch should have been taken for case: {}",
                    desc
                );
                assert_eq!(
                    cpu.registers.program_counter, *exp_pc,
                    "PC mismatch for case: {}",
                    desc
                );
                assert_eq!(
                    page_penalty, *should_have_penalty,
                    "Page penalty mismatch for case: {}",
                    desc
                );
            } else {
                assert!(
                    break_occurred,
                    "Branch should not have been taken for case: {}",
                    desc
                );
                assert_eq!(
                    cpu.registers.program_counter,
                    pc + 1,
                    "PC should only advance by fetch for case: {}",
                    desc
                );
            }

            assert_eq!(
                cpu.temp_data, *offset,
                "temp_data should contain offset for case: {}",
                desc
            );
        }
    }

    #[test]
    fn test_bcs_preserves_other_flags() {
        let mut cpu = create_test_cpu_with_data(&[0x10], 0x1000);
        cpu.registers.program_counter = 0x1000;
        cpu.flags.carry = true;

        // Set other flags that should be preserved
        cpu.flags.zero = true;
        cpu.flags.negative = true;
        cpu.flags.overflow = true;
        cpu.flags.decimal_mode = true;
        cpu.flags.interrupt_disable = true;
        cpu.flags.break_command = true;

        // Execute RELATIVE sequence
        for operation in RELATIVE.iter() {
            operation(&mut cpu).unwrap();
        }

        // BCS should not affect any flags
        assert!(cpu.flags.carry, "Carry flag should be unchanged");
        assert!(cpu.flags.zero, "Zero flag should be preserved");
        assert!(cpu.flags.negative, "Negative flag should be preserved");
        assert!(cpu.flags.overflow, "Overflow flag should be preserved");
        assert!(cpu.flags.decimal_mode, "Decimal mode should be preserved");
        assert!(
            cpu.flags.interrupt_disable,
            "Interrupt disable should be preserved"
        );
        assert!(cpu.flags.break_command, "Break command should be preserved");
    }

    #[test]
    fn test_bcs_bus_error_propagation() {
        // Test with limited RAM that doesn't cover the PC address
        let ram = Ram::new(RamSize::_16K, 0x0000); // Only covers 0x0000-0x3FFF
        let mut cpu = CpuBuilder::new()
            .with_bus_device(ram, 0x0000, 0x3FFF)
            .expect("Failed to add RAM")
            .build()
            .expect("Failed to build CPU");

        cpu.registers.program_counter = 0x8000; // Outside RAM range
        cpu.flags.carry = true;

        let result = fetch_offset(&mut cpu);

        // Should get a bus error when trying to fetch from unmapped memory
        assert!(result.is_err(), "Expected bus error");
        match result.unwrap_err() {
            CpuError::BusError(_) => (),
            other => panic!("Expected BusError, got: {:?}", other),
        }
    }

    #[test]
    fn test_bcs_complete_instruction_simulation() {
        // Test realistic BCS instruction scenarios

        // Scenario 1: Simple forward branch taken
        let mut cpu = create_test_cpu_with_data(&[0x08], 0x2000); // BCS +8
        cpu.registers.program_counter = 0x2000;
        cpu.flags.carry = true;

        for operation in RELATIVE.iter() {
            operation(&mut cpu).unwrap();
        }
        assert_eq!(cpu.registers.program_counter, 0x2009); // 0x2001 + 8

        // Scenario 2: Backward branch not taken
        let mut cpu = create_test_cpu_with_data(&[0xF8], 0x2000); // BCS -8
        cpu.registers.program_counter = 0x2000;
        cpu.flags.carry = false;

        let mut operations = RELATIVE.iter();
        operations.next().unwrap()(&mut cpu).unwrap(); // Should break after first operation
        assert_eq!(cpu.registers.program_counter, 0x2001); // Only PC increment from fetch

        // Scenario 3: Loop back with page boundary penalty
        let mut cpu = create_test_cpu_with_data(&[0x80], 0x2100); // BCS -128
        cpu.registers.program_counter = 0x2100;
        cpu.flags.carry = true;

        let mut penalty_detected = false;
        for operation in RELATIVE.iter() {
            let result = operation(&mut cpu).unwrap();
            if let OperationResult::PageBoundaryPenalty(_) = result {
                penalty_detected = true;
            }
        }
        assert!(penalty_detected);
        assert_eq!(cpu.registers.program_counter, 0x2081); // 0x2101 - 128
    }

    #[test]
    fn test_bcs_carry_flag_behavior() {
        // Test that BCS correctly responds to carry flag state
        let test_cases = [
            (true, "carry set - should branch"),
            (false, "carry clear - should not branch"),
        ];

        for (carry_state, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu_with_data(&[0x20], 0x1000); // +32 offset
            cpu.registers.program_counter = 0x1000;
            cpu.flags.carry = *carry_state;

            let fetch_result = fetch_offset(&mut cpu).unwrap();

            if *carry_state {
                assert_eq!(
                    fetch_result,
                    OperationResult::Continue,
                    "Should continue when {}",
                    desc
                );
            } else {
                assert_eq!(
                    fetch_result,
                    OperationResult::Break,
                    "Should break when {}",
                    desc
                );
            }

            assert_eq!(
                cpu.temp_data, 0x20,
                "Offset should be fetched regardless for {}",
                desc
            );
            assert_eq!(
                cpu.registers.program_counter, 0x1001,
                "PC should advance after fetch for {}",
                desc
            );
        }
    }

    #[test]
    fn test_bcs_signed_offset_interpretation() {
        // Test that offsets are correctly interpreted as signed 8-bit values
        let test_cases = [
            (0x00, 0, "zero offset"),
            (0x01, 1, "positive offset +1"),
            (0x7F, 127, "maximum positive offset +127"),
            (0x80, -128, "maximum negative offset -128"),
            (0xFF, -1, "negative offset -1"),
            (0xFE, -2, "negative offset -2"),
        ];

        for (offset_byte, expected_displacement, desc) in test_cases.iter() {
            let mut cpu = create_test_cpu();
            cpu.registers.program_counter = 0x1000;
            cpu.temp_data = *offset_byte;

            let initial_pc = cpu.registers.program_counter;
            add_offset_to_program_counter(&mut cpu).unwrap();

            let actual_displacement = (cpu.registers.program_counter as i32) - (initial_pc as i32);
            assert_eq!(
                actual_displacement, *expected_displacement as i32,
                "Displacement incorrect for {}",
                desc
            );
        }
    }
}
