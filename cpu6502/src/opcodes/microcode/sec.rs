//! Set Carry Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn set_carry_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.carry = true;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [set_carry_flag];
