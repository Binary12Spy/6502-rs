//! Increment Memory

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn inc_temp_data_no_flags(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    cpu.temp_data = cpu.temp_data.wrapping_add(1);
    Ok(OperationResult::Continue)
}

fn temp_data_into_temp_address_flags(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    cpu.update_zero_negative_flags(cpu.temp_data);
    Ok(OperationResult::Continue)
}

pub(crate) static ZEROPAGE: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    inc_temp_data_no_flags,
    temp_data_into_temp_address_flags,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    common::temp_address_data_into_temp_data,
    inc_temp_data_no_flags,
    temp_data_into_temp_address_flags,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_data_into_temp_data,
    inc_temp_data_no_flags,
    temp_data_into_temp_address_flags,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<6> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_page_boundary_check,
    common::temp_address_data_into_temp_data,
    inc_temp_data_no_flags,
    temp_data_into_temp_address_flags,
];
