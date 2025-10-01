//! Set Decimal Mode Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn set_decimal_mode_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.decimal_mode = true;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [set_decimal_mode_flag];
