//! Load X Register

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn operand_into_x_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    cpu.registers.x = cpu.temp_data;
    cpu.update_zero_negative_flags(cpu.registers.x);
    Ok(OperationResult::Continue)
}

fn temp_address_data_into_x_register(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.temp_data = data;
    cpu.registers.x = data;
    cpu.update_zero_negative_flags(cpu.registers.x);
    Ok(OperationResult::Continue)
}

fn operand_into_temp_address_high_add_y_page_boundary_check(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_address |= (cpu.fetch_operand()? as u16) << 8;
    let target_address = cpu.temp_address.wrapping_add(cpu.registers.y as u16);
    if cpu.determine_page_cross_penalty(cpu.temp_address, target_address) {
        return Ok(OperationResult::PageBoundaryPenalty(1));
    }
    Ok(OperationResult::Continue)
}

fn temp_address_add_y_data_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_address = cpu.temp_address.wrapping_add(cpu.registers.y as u16);
    let data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.temp_data = data;
    cpu.registers.accumulator = data;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

pub(crate) static IMMEDIATE: MicrocodeSequence<1> = [operand_into_x_register];
pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    temp_address_data_into_x_register,
];
pub(crate) static ZEROPAGE_Y: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_y_register,
    temp_address_data_into_x_register,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    temp_address_data_into_x_register,
];
pub(crate) static ABSOLUTE_Y: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    operand_into_temp_address_high_add_y_page_boundary_check,
    temp_address_add_y_data_into_accumulator,
];
