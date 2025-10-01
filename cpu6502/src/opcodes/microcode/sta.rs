//! Store Accumulator

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn accumulator_into_temp_address(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.registers.accumulator;
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    Ok(OperationResult::Continue)
}

fn temp_address_data_into_temp_address_low_zero_page(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    let low_byte = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)? as u16;
    cpu.increment_pc();
    cpu.temp_address = (low_byte as u16) & 0x00FF;
    Ok(OperationResult::Continue)
}

fn temp_address_data_into_temp_address_high(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_address |= (cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)? as u16) << 8;
    cpu.increment_pc();
    Ok(OperationResult::Continue)
}

pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    accumulator_into_temp_address,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    accumulator_into_temp_address,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    accumulator_into_temp_address,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_register,
    accumulator_into_temp_address,
];
pub(crate) static ABSOLUTE_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_y_register,
    accumulator_into_temp_address,
];
pub(crate) static INDIRECT_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    temp_address_data_into_temp_address_low_zero_page,
    temp_address_data_into_temp_address_high,
    accumulator_into_temp_address,
];
pub(crate) static INDIRECT_Y: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    temp_address_data_into_temp_address_low_zero_page,
    temp_address_data_into_temp_address_high,
    common::temp_address_add_y_register,
    accumulator_into_temp_address,
];
