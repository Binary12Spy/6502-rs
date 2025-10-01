//! Clear Carry Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn clear_carry_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.carry = false;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [clear_carry_flag];
