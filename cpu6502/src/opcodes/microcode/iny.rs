//! Increment Y Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn increment_y_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.y = cpu.registers.y.wrapping_add(1);
    cpu.update_zero_negative_flags(cpu.registers.y);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [increment_y_register];
