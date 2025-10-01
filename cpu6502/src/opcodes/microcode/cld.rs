//! Clear Decimal Mode Flag

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn clear_decimal_mode_flag(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.decimal_mode = false;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [clear_decimal_mode_flag];
