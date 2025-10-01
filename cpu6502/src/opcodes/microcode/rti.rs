//! Return from Interrupt

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn stack_to_flags(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.pop_stack_ptr()?;
    cpu.flags = cpu.pop_stack_data()?.try_into().map_err(CpuError::Other)?;
    Ok(OperationResult::Continue)
}

fn stack_to_return_address_low(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.pop_stack_ptr()?;
    let low = cpu.pop_stack_data()?;
    cpu.registers.program_counter = (cpu.registers.program_counter & 0xFF00) | (low as u16);
    Ok(OperationResult::Continue)
}

fn stack_to_return_address_high(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.pop_stack_ptr()?;
    let high = cpu.pop_stack_data()?;
    cpu.registers.program_counter = (cpu.registers.program_counter & 0x00FF) | ((high as u16) << 8);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<5> = [
    common::none,
    stack_to_flags,
    stack_to_return_address_low,
    stack_to_return_address_high,
    common::none,
];
