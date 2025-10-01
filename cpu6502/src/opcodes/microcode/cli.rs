//! Clear Interrupt Disable Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn clear_interrupt_disable_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.interrupt_disable = false;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [clear_interrupt_disable_flag];
