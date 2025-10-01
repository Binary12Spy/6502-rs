//! Branch if Minus

use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn fetch_offset(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    if !cpu.flags.negative {
        return Ok(OperationResult::Break);
    }
    Ok(OperationResult::Continue)
}

fn add_offset_to_program_counter(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let old_pc = cpu.registers.program_counter;
    cpu.registers.program_counter =
        alu::add_pc_with_signed_offset(cpu.registers.program_counter, cpu.temp_data)
            .map_err(|e| CpuError::AluError(e))?;
    if (old_pc & 0xFF00) != (cpu.registers.program_counter & 0xFF00) {
        return Ok(OperationResult::PageBoundaryPenalty(1));
    }
    Ok(OperationResult::Continue)
}

pub(crate) static RELATIVE: MicrocodeSequence<2> = [fetch_offset, add_offset_to_program_counter];
