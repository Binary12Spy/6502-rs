//! Transfer Accumulator to Y Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn accumulator_into_y_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.y = cpu.registers.accumulator;
    cpu.update_zero_negative_flags(cpu.registers.y);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [accumulator_into_y_register];
