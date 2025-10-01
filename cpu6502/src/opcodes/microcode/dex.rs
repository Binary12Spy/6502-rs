//! Decrement X Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn decrement_x_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.x = cpu.registers.x.wrapping_sub(1);
    cpu.update_zero_negative_flags(cpu.registers.x);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [decrement_x_register];
