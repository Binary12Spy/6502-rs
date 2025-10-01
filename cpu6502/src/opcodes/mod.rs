pub(crate) mod addressing_modes;
pub(crate) mod instruction_variants;
pub(crate) mod instructions;
pub(crate) mod microcode;

use instruction_variants::{INSTRUCTION_VARIANTS, InstructionVariant};
use instructions::Instruction;

/// Get Instruction Variant by Opcode
///
/// # Arguments
/// * `opcode` - The opcode byte to look up
///
/// # Returns
/// * `Option<&'static InstructionVariant>` - The corresponding instruction variant, if found
///
/// # Example
/// ``` ignore
/// use cpu6502::opcodes::{variant_by_opcode, instructions::Instruction, addressing_modes::AddressingMode};
///
/// let opcode = 0xA9; // LDA Immediate
/// if let Some(variant) = variant_by_opcode(opcode) {
///     assert_eq!(variant.instruction, Instruction::LDA(AddressingMode::Immediate));
/// }
/// ```
pub(crate) fn variant_by_opcode(opcode: u8) -> Option<&'static InstructionVariant> {
    INSTRUCTION_VARIANTS
        .iter()
        .find(|variant| variant.opcode == opcode)
}

/// Get Instruction Variant by Instruction
///
/// # Arguments
/// * `instruction` - The instruction to look up
///
/// # Returns
/// * `Option<&'static InstructionVariant>` - The corresponding instruction variant, if found
///
/// # Example
/// ``` ignore
/// use cpu6502::opcodes::{variant_by_instruction, instructions::Instruction, addressing_modes::AddressingMode};
///
/// let instruction = Instruction::LDA(AddressingMode::Immediate);
/// if let Some(variant) = variant_by_instruction(&instruction) {
///     assert_eq!(variant.opcode, 0xA9);
/// }
/// ```
pub(crate) fn variant_by_instruction(
    instruction: &Instruction,
) -> Option<&'static InstructionVariant> {
    INSTRUCTION_VARIANTS
        .iter()
        .find(|variant| &variant.instruction == instruction)
}
