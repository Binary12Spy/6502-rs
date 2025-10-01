//! Transfer X Register to Stack Pointer

use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn x_register_into_stack_pointer(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.stack_pointer = cpu.registers.x;
    cpu.update_zero_negative_flags(cpu.registers.stack_pointer);
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<1> = [x_register_into_stack_pointer];
