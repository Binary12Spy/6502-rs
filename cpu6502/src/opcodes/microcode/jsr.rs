//! Jump to Subroutine

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn return_address_high_to_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let return_address = cpu.registers.program_counter.wrapping_sub(1);
    cpu.push_stack_data((return_address >> 8) as u8)?;
    cpu.push_stack_ptr()?;
    Ok(OperationResult::Continue)
}

fn return_address_low_to_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let return_address = cpu.registers.program_counter.wrapping_sub(1);
    cpu.push_stack_data((return_address & 0x00FF) as u8)?;
    cpu.push_stack_ptr()?;
    Ok(OperationResult::Continue)
}

fn jump_to_temp_address(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.program_counter = cpu.temp_address;
    Ok(OperationResult::Continue)
}

pub(crate) static ABSOLUTE: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    return_address_high_to_stack,
    return_address_low_to_stack,
    jump_to_temp_address,
];
