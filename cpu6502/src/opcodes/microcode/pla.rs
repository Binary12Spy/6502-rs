//! Pull Accumulator from Stack

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn pull_accumulator_from_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.accumulator = cpu.pop_stack_data()?;
    Ok(OperationResult::Continue)
}

fn update_zero_negative_flags(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<3> = [
    common::pop_stack_pointer,
    pull_accumulator_from_stack,
    update_zero_negative_flags,
];
