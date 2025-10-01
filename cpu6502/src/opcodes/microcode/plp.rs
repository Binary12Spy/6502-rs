//! Pull Flags from Stack

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use crate::flags::Flags;

fn flags_from_temp_data(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags = Flags::try_from(cpu.temp_data).map_err(CpuError::Other)?;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<3> = [
    common::pop_stack_pointer,
    common::pop_stack_to_temp_data,
    flags_from_temp_data,
];
