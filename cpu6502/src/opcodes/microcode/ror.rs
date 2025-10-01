//! Rotate Right

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn accumulator_ror(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.registers.accumulator = alu::ror(cpu.registers.accumulator, &mut cpu.flags);
    Ok(OperationResult::Continue)
}

fn temp_data_ror(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = alu::ror(cpu.temp_data, &mut cpu.flags);
    cpu.bus
        .write(cpu.temp_address, cpu.temp_data)
        .map_err(CpuError::BusError)?;
    Ok(OperationResult::Continue)
}

pub(crate) static ACCUMULATOR: MicrocodeSequence<1> = [accumulator_ror];
pub(crate) static ZEROPAGE: MicrocodeSequence<4> = [
    common::operand_into_temp_address_low,
    common::temp_address_data_into_temp_data,
    temp_data_ror,
    common::temp_data_into_temp_address,
];
pub(crate) static ZEROPAGE_X: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::temp_address_add_x_register_zero_page,
    common::temp_address_data_into_temp_data,
    temp_data_ror,
    common::temp_data_into_temp_address,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<5> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_data_into_temp_data,
    temp_data_ror,
    common::temp_data_into_temp_address,
];
pub(crate) static ABSOLUTE_X: MicrocodeSequence<6> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    common::temp_address_add_x_register,
    common::temp_address_data_into_temp_data,
    temp_data_ror,
    common::temp_data_into_temp_address,
];
