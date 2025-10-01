//! Bit Test

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::alu;
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn accumulator_bit_temp_address_data(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_data = cpu.bus.read(cpu.temp_address).map_err(CpuError::BusError)?;
    alu::bit(cpu.registers.accumulator, cpu.temp_data, &mut cpu.flags);

    Ok(OperationResult::Continue)
}

pub(crate) static ZEROPAGE: MicrocodeSequence<2> = [
    common::operand_into_temp_address_low,
    accumulator_bit_temp_address_data,
];
pub(crate) static ABSOLUTE: MicrocodeSequence<3> = [
    common::operand_into_temp_address_low,
    common::operand_into_temp_address_high,
    accumulator_bit_temp_address_data,
];
