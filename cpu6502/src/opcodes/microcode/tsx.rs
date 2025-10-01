//! Transfer Stack Pointer to X Register

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn stack_pointer_into_x_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.x = cpu.registers.stack_pointer;
    cpu.update_zero_negative_flags(cpu.registers.x);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [stack_pointer_into_x_register];
