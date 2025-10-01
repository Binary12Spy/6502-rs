//! Push Accumulator onto Stack

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn push_accumulator_onto_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.push_stack_data(cpu.registers.accumulator)?;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<2> =
    [push_accumulator_onto_stack, common::push_stack_pointer];
