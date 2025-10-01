//! Force Interrupt

use super::common;
use super::{MicrocodeSequence, OperationResult};
use crate::cpu::Cpu;
use crate::errors::CpuError;
use bus::trait_bus_device::BusDevice;

fn return_address_high_to_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let return_address = cpu.registers.program_counter.wrapping_add(1);
    cpu.push_stack_data((return_address >> 8) as u8)?;
    cpu.push_stack_ptr()?;
    Ok(OperationResult::Continue)
}

fn return_address_low_to_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    let return_address = cpu.registers.program_counter.wrapping_add(1);
    cpu.push_stack_data((return_address & 0x00FF) as u8)?;
    cpu.push_stack_ptr()?;
    Ok(OperationResult::Continue)
}

fn flags_to_stack(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.flags.break_command = true;
    cpu.push_stack_data(cpu.flags.into())?;
    cpu.push_stack_ptr()?;
    Ok(OperationResult::Continue)
}

fn irq_vector_low_into_temp_address_low(cpu: &mut Cpu) -> Result<OperationResult, CpuError> {
    cpu.temp_address = cpu.bus.read(0xFFFE).map_err(CpuError::BusError)? as u16;
    Ok(OperationResult::Continue)
}

fn irq_vector_high_into_temp_address_high_into_program_counter(
    cpu: &mut Cpu,
) -> Result<OperationResult, CpuError> {
    cpu.temp_address |= (cpu.bus.read(0xFFFF).map_err(CpuError::BusError)? as u16) << 8;
    Ok(OperationResult::Continue)
}

pub(crate) static IMPLIED: MicrocodeSequence<6> = [
    common::none,
    return_address_high_to_stack,
    return_address_low_to_stack,
    flags_to_stack,
    irq_vector_low_into_temp_address_low,
    irq_vector_high_into_temp_address_high_into_program_counter,
];
