//! Transfer Y Register to Accumulator

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn y_register_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.accumulator = cpu.registers.y;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [y_register_into_accumulator];
