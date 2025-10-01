//! Load Accumulator

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn operand_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    cpu.registers.accumulator = cpu.temp_data;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

fn temp_address_data_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.temp_data = data;
    cpu.registers.accumulator = data;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
    Ok(OperationResult::Continue)
}

fn operand_into_temp_address_high_add_x_page_boundary_check(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_address |= (cpu.fetch_operand()? as u16) << 8;
    let target_address = cpu.temp_address.wrapping_add(cpu.registers.x as u16);
    if cpu.determine_page_cross_penalty(cpu.temp_address, target_address) {
        return Ok(OperationResult::PageBoundaryPenalty(1));
    }
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

fn temp_address_inc_data_as_temp_address_high_page_boundary_check(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_address = (cpu.temp_data as u16)
        | (cpu
            .bus
            .read(cpu.temp_address.wrapping_add(1) & 0x00FF)
            .map_err(CpuError::BusError)? as u16)
            << 8;

    let target_address = cpu.temp_address.wrapping_add(cpu.registers.y as u16);
    if cpu.determine_page_cross_penalty(cpu.temp_address, target_address) {
        return Ok(OperationResult::PageBoundaryPenalty(1));
    }
    Ok(OperationResult::Continue)
}

fn temp_address_add_x_data_into_accumulator(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_address = cpu.temp_address.wrapping_add(cpu.registers.x as u16);
    let data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    cpu.temp_data = data;
    cpu.registers.accumulator = data;
    cpu.update_zero_negative_flags(cpu.registers.accumulator);
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

pub(crate) static IMMEDIATE: MicrocodeSequence<1> = [operand_into_accumulator];
pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    temp_address_data_into_accumulator,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register,
    temp_address_data_into_accumulator,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    temp_address_data_into_accumulator,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    operand_into_temp_address_high_add_x_page_boundary_check,
    temp_address_add_x_data_into_accumulator,
];
pub(crate) static ABSOLUTE_Y: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    operand_into_temp_address_high_add_y_page_boundary_check,
    temp_address_add_y_data_into_accumulator,
];
pub(crate) static INDIRECT_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register,
    common::temp_address_data_into_temp_data,
    common::temp_data_low_and_temp_address_inc_high_into_temp_address,
    temp_address_data_into_accumulator,
];
pub(crate) static INDIRECT_Y: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    temp_address_inc_data_as_temp_address_high_page_boundary_check,
    temp_address_add_y_data_into_accumulator,
];
