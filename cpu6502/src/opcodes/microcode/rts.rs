//! Return from Subroutine

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn stack_data_to_temp_address_low(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.pop_stack_ptr()?;
    cpu.temp_address = cpu.pop_stack_data()? as u16;
    Ok(OperationResult::Continue)
}

fn stack_data_to_temp_address_high(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.pop_stack_ptr()?;
    cpu.temp_address |= (cpu.pop_stack_data()? as u16) << 8;
    Ok(OperationResult::Continue)
}

fn increment_temp_address(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_address = cpu.temp_address.wrapping_add(1);
    Ok(OperationResult::Continue)
}

fn jump_to_temp_address(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.program_counter = cpu.temp_address;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<5> = [
    common::none,
    stack_data_to_temp_address_low,
    stack_data_to_temp_address_high,
    increment_temp_address,
    jump_to_temp_address,
];
