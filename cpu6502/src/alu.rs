#![allow(dead_code)]

use crate::flags::Flags;

/// Perform ADC (Add with Carry)
///
/// Returns a tuple containing the result, carry, zero, negative, and overflow flags.
///
/// # Arguments
/// * `a` - Accumulator register
/// * `operand` - Operand to add
/// * `carry` - Carry flag
/// * `decimal_mode` - Decimal mode flag
/// * `flags` - Mutable reference to Flags struct to update CPU flags
///
/// # Returns
/// * `result` - Result of the addition
///
/// # Notes
/// The ADC instruction is implemented as a simple addition.
/// The carry flag is added to the result.
/// The overflow flag is set if the result is outside the range of a signed byte.
/// The decimal mode flag changes the behavior of the addition.
pub fn add(a: u8, operand: u8, flags: &mut Flags) -> Result<u8, String> {
    let carry_in = if flags.carry { 1 } else { 0 };
    let result = (a as u16)
        .wrapping_add(operand as u16)
        .wrapping_add(carry_in as u16);

    let mut result_byte = result as u8;
    flags.carry = result > 0xFF;
    flags.overflow = false;

    if flags.decimal_mode {
        let mut adjust = 0;

        // Adjust lower nibble (0x0F)
        if (a & 0x0F) + (operand & 0x0F) + carry_in > 9 {
            adjust += 0x06;
        }

        // Adjust upper nibble (0xF0)
        if result > 0x99 {
            adjust += 0x60;
            flags.carry = true;
        } else {
            flags.carry = false;
        }

        result_byte = result.wrapping_add(adjust) as u8;
    }

    // Overflow detection
    let a_sign = (a & 0x80) != 0;
    let op_sign = (operand & 0x80) != 0;
    let res_sign = (result_byte & 0x80) != 0;
    flags.overflow = (a_sign == op_sign) && (a_sign != res_sign);

    // Zero and Negative Flags
    flags.zero = result_byte == 0;
    flags.negative = (result_byte & 0x80) != 0;

    Ok(result_byte)
}

pub(crate) fn add_pc_with_signed_offset(pc: u16, offset: u8) -> Result<u16, String> {
    let signed_offset = offset as i8 as i16; // Convert to signed
    let new_pc = (pc as i16).wrapping_add(signed_offset);
    if new_pc > i16::MAX {
        return Err("Program Counter out of bounds".to_string());
    }
    Ok(new_pc as u16)
}

/// Perform SBC (Subtract with Carry)
///
/// Returns a tuple containing the result, carry, zero, negative, and overflow flags.
///
/// # Arguments
/// * `a` - Accumulator register
/// * `operand` - Operand to subtract
/// * `carry` - Carry flag
/// * `decimal_mode` - Decimal mode flag
/// * `flags` - Mutable reference to Flags struct to update CPU flags
///
/// # Returns
/// * `result` - Result of the subtraction
///
/// # Notes
/// The SBC instruction is implemented as an addition of the 1's complement of the operand.
/// The carry flag is inverted before the addition.
/// The overflow flag is set if the result is outside the range of a signed byte.
/// The decimal mode flag changes the behavior of the subtraction.
pub fn sub(a: u8, operand: u8, flags: &mut Flags) -> Result<u8, String> {
    // SBC is implemented as A + (~M) + C
    // Where ~M is the bitwise complement and C is the carry flag
    let operand_complement = !operand;
    let carry_in = if flags.carry { 1 } else { 0 };

    let result = (a as u16)
        .wrapping_add(operand_complement as u16)
        .wrapping_add(carry_in as u16);

    let mut result_byte = result as u8;
    flags.carry = result > 0xFF; // Carry set if result > 255

    if flags.decimal_mode {
        // In decimal mode, we need to do BCD (Binary Coded Decimal) arithmetic
        // This is complex and we'll do a simplified version
        let mut al = (a & 0x0F) as i16;
        let mut ah = (a >> 4) as i16;
        let bl = (operand & 0x0F) as i16;
        let bh = (operand >> 4) as i16;
        let c = if flags.carry { 1 } else { 0 };

        // Subtract lower nibble
        al = al - bl - (1 - c);
        if al < 0 {
            al += 10;
            ah -= 1;
        }

        // Subtract upper nibble
        ah = ah - bh;
        if ah < 0 {
            ah += 10;
            flags.carry = false;
        } else {
            flags.carry = true;
        }

        result_byte = ((ah << 4) | al) as u8;
    }

    // Overflow detection
    let a_sign = (a & 0x80) != 0;
    let op_sign = (operand & 0x80) != 0;
    let res_sign = (result_byte & 0x80) != 0;
    flags.overflow = (a_sign != op_sign) && (a_sign != res_sign);

    // Zero and Negative Flags
    flags.zero = result_byte == 0;
    flags.negative = (result_byte & 0x80) != 0;

    Ok(result_byte)
}

pub(crate) fn and(a: u8, operand: u8, flags: &mut Flags) -> u8 {
    let result = a & operand;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn ora(a: u8, operand: u8, flags: &mut Flags) -> u8 {
    let result = a | operand;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn eor(a: u8, operand: u8, flags: &mut Flags) -> u8 {
    let result = a ^ operand;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn asl(value: u8, flags: &mut Flags) -> u8 {
    let result = value << 1;
    flags.carry = (value & 0x80) != 0;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn lsr(value: u8, flags: &mut Flags) -> u8 {
    let result = value >> 1;
    flags.carry = (value & 0x01) != 0;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn rol(value: u8, flags: &mut Flags) -> u8 {
    let carry_in = if flags.carry { 1 } else { 0 };
    let result = (value << 1) | carry_in;
    flags.carry = (value & 0x80) != 0;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn ror(value: u8, flags: &mut Flags) -> u8 {
    let carry_in = if flags.carry { 0x80 } else { 0 };
    let result = (value >> 1) | carry_in;
    flags.carry = (value & 0x01) != 0;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
    result
}

pub(crate) fn cmp(a: u8, operand: u8, flags: &mut Flags) {
    let result = a.wrapping_sub(operand);
    flags.carry = a >= operand;
    flags.zero = result == 0;
    flags.negative = (result & 0x80) != 0;
}

pub(crate) fn bit(a: u8, operand: u8, flags: &mut Flags) {
    let result = a & operand;
    flags.zero = result == 0;
    flags.overflow = (operand & 0x40) != 0;
    flags.negative = (operand & 0x80) != 0;
}
