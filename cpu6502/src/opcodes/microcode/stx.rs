//! Store X Register

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn x_register_into_temp_address(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.registers.x;
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    Ok(OperationResult::Continue)
}

pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    x_register_into_temp_address,
];
pub(crate) static ZEROPAGE_Y: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_y_register_zero_page,
    x_register_into_temp_address,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    x_register_into_temp_address,
];
