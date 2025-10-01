//! Set Interrupt Disable Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn set_interrupt_disable_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.interrupt_disable = true;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [set_interrupt_disable_flag];
