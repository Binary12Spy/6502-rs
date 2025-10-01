//! Transfer X Register to Accumulator

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn x_register_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.accumulator = cpu.registers.x;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [x_register_into_accumulator];
