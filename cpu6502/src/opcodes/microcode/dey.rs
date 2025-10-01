//! Decrement Y Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn decrement_y_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    cpu.update_zero_negative_flags(cpu.registers.y);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [decrement_y_register];
