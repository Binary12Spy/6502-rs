//! Subtract with Carry

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn operand_subtract_from_accumulator_with_carry(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    cpu.registers.accumulator = alu::sub(cpu.registers.accumulator, cpu.temp_data, &mut cpu.flags)
        .map_err(|e| CpuError::AluError(e))?;

    Ok(OperationResult::Continue)
}

fn accumulator_subtract_temp_address_data_with_carry(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.registers.accumulator = alu::sub(cpu.registers.accumulator, cpu.temp_data, &mut cpu.flags)
        .map_err(|e| CpuError::AluError(e))?;

    Ok(OperationResult::Continue)
}

pub(crate) static IMMEDIATE: MicrocodeSequence<1> = [operand_subtract_from_accumulator_with_carry];
pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_page_boundary_check,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static ABSOLUTE_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_y_page_boundary_check,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static INDIRECT_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register,
    common::temp_address_data_into_temp_data,
    common::temp_data_low_and_temp_address_inc_high_into_temp_address,
    accumulator_subtract_temp_address_data_with_carry,
];
pub(crate) static INDIRECT_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    common::temp_address_inc_data_as_temp_address_high_add_y_page_boundary_check,
    accumulator_subtract_temp_address_data_with_carry,
];
