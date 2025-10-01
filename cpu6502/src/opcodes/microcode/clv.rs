//! Clear Overflow Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn clear_overflow_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.overflow = false;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [clear_overflow_flag];
