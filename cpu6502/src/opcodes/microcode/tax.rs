//! Transfer Accumulator to X Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn accumulator_into_x_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.x = cpu.registers.accumulator;
    cpu.update_zero_negative_flags(cpu.registers.x);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [accumulator_into_x_register];
