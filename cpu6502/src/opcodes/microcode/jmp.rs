//! Jump

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;

fn operand_into_temp_address_high_into_program_counter(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_address = (cpu.fetch_operand()? as u16) << 8;
    cpu.registers.program_counter = cpu.temp_address;
    Ok(OperationResult::Continue)
}

pub(crate) static ABSOLUTE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    operand_into_temp_address_high_into_program_counter,
];
pub(crate) static INDIRECT: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_data_into_temp_data,
    common::temp_data_low_and_temp_address_inc_high_into_temp_address,
    operand_into_temp_address_high_into_program_counter,
];
