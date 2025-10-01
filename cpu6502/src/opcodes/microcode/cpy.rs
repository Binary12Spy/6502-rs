//! Compare Y Register

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn y_register_cmp_operand(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.fetch_operand()?;
    alu::cmp(cpu.registers.y, cpu.temp_data, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

fn y_register_cmp_temp_address_data(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    alu::cmp(cpu.registers.y, cpu.temp_data, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

pub(crate) static IMMEDIATE: MicrocodeSequence<1> = [y_register_cmp_operand];
pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    y_register_cmp_temp_address_data,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    y_register_cmp_temp_address_data,
];
