//! Instruction variants for 6502 CPU emulation

use super::addressing_modes::AddressingMode;
use super::instructions::Instruction;
use super::microcode::{
    MicrocodeStep, adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl, brk, bvc, bvs, clc, cld, cli,
    clv, cmp, cpx, cpy, dec, dex, dey, eor, inc, inx, iny, jmp, jsr, lda, ldx, ldy, lsr, nop, ora,
    pha, php, pla, plp, rol, ror, rti, rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa,
    txs, tya,
};

/// Instruction Variant
pub(crate) struct InstructionVariant {
    /// The instruction associated with this variant
    pub instruction: Instruction,
    /// The opcode for this instruction variant
    pub opcode: u8,
    /// The number of CPU cycles this instruction takes
    pub microcode_sequence: &'static [MicrocodeStep],
}

/// Default Instruction Variant (NOP)
pub(crate) static DEFAULT_INSTRUCTION_VARIANT: &InstructionVariant = &INSTRUCTION_VARIANTS[150];

/// Instruction Variants
pub(crate) static INSTRUCTION_VARIANTS: [InstructionVariant; 151] = [
    // --- Transfer Instructions ---
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::Immediate),
        opcode: 0xA9,
        microcode_sequence: &lda::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::ZeroPage),
        opcode: 0xA5,
        microcode_sequence: &lda::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::ZeroPageX),
        opcode: 0xB5,
        microcode_sequence: &lda::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::Absolute),
        opcode: 0xAD,
        microcode_sequence: &lda::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::AbsoluteX),
        opcode: 0xBD,
        microcode_sequence: &lda::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::AbsoluteY),
        opcode: 0xB9,
        microcode_sequence: &lda::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::IndirectX),
        opcode: 0xA1,
        microcode_sequence: &lda::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::LDA(AddressingMode::IndirectY),
        opcode: 0xB1,
        microcode_sequence: &lda::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::LDX(AddressingMode::Immediate),
        opcode: 0xA2,
        microcode_sequence: &ldx::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::LDX(AddressingMode::ZeroPage),
        opcode: 0xA6,
        microcode_sequence: &ldx::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::LDX(AddressingMode::ZeroPageY),
        opcode: 0xB6,
        microcode_sequence: &ldx::ZEROPAGE_Y,
    },
    InstructionVariant {
        instruction: Instruction::LDX(AddressingMode::Absolute),
        opcode: 0xAE,
        microcode_sequence: &ldx::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::LDX(AddressingMode::AbsoluteY),
        opcode: 0xBE,
        microcode_sequence: &ldx::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::LDY(AddressingMode::Immediate),
        opcode: 0xA0,
        microcode_sequence: &ldy::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::LDY(AddressingMode::ZeroPage),
        opcode: 0xA4,
        microcode_sequence: &ldy::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::LDY(AddressingMode::ZeroPageX),
        opcode: 0xB4,
        microcode_sequence: &ldy::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::LDY(AddressingMode::Absolute),
        opcode: 0xAC,
        microcode_sequence: &ldy::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::LDY(AddressingMode::AbsoluteX),
        opcode: 0xBC,
        microcode_sequence: &ldy::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::ZeroPage),
        opcode: 0x85,
        microcode_sequence: &sta::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::ZeroPageX),
        opcode: 0x95,
        microcode_sequence: &sta::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::Absolute),
        opcode: 0x8D,
        microcode_sequence: &sta::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::AbsoluteX),
        opcode: 0x9D,
        microcode_sequence: &sta::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::AbsoluteY),
        opcode: 0x99,
        microcode_sequence: &sta::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::IndirectX),
        opcode: 0x81,
        microcode_sequence: &sta::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::STA(AddressingMode::IndirectY),
        opcode: 0x91,
        microcode_sequence: &sta::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::STX(AddressingMode::ZeroPage),
        opcode: 0x86,
        microcode_sequence: &stx::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::STX(AddressingMode::ZeroPageY),
        opcode: 0x96,
        microcode_sequence: &stx::ZEROPAGE_Y,
    },
    InstructionVariant {
        instruction: Instruction::STX(AddressingMode::Absolute),
        opcode: 0x8E,
        microcode_sequence: &stx::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::STY(AddressingMode::ZeroPage),
        opcode: 0x84,
        microcode_sequence: &sty::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::STY(AddressingMode::ZeroPageX),
        opcode: 0x94,
        microcode_sequence: &sty::ZEROPAGE_Y,
    },
    InstructionVariant {
        instruction: Instruction::STY(AddressingMode::Absolute),
        opcode: 0x8C,
        microcode_sequence: &sty::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::TAX(AddressingMode::Implied),
        opcode: 0xAA,
        microcode_sequence: &tax::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::TAY(AddressingMode::Implied),
        opcode: 0xA8,
        microcode_sequence: &tay::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::TSX(AddressingMode::Implied),
        opcode: 0xBA,
        microcode_sequence: &tsx::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::TXA(AddressingMode::Implied),
        opcode: 0x8A,
        microcode_sequence: &txa::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::TXS(AddressingMode::Implied),
        opcode: 0x9A,
        microcode_sequence: &txs::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::TYA(AddressingMode::Implied),
        opcode: 0x98,
        microcode_sequence: &tya::IMPLIED,
    },
    // --- Stack Instructions ---
    InstructionVariant {
        instruction: Instruction::PHA(AddressingMode::Implied),
        opcode: 0x48,
        microcode_sequence: &pha::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::PHP(AddressingMode::Implied),
        opcode: 0x08,
        microcode_sequence: &php::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::PLA(AddressingMode::Implied),
        opcode: 0x68,
        microcode_sequence: &pla::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::PLP(AddressingMode::Implied),
        opcode: 0x28,
        microcode_sequence: &plp::IMPLIED,
    },
    // --- Increment & Decrement Instructions ---
    InstructionVariant {
        instruction: Instruction::DEC(AddressingMode::ZeroPage),
        opcode: 0xC6,
        microcode_sequence: &dec::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::DEC(AddressingMode::ZeroPageX),
        opcode: 0xD6,
        microcode_sequence: &dec::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::DEC(AddressingMode::Absolute),
        opcode: 0xCE,
        microcode_sequence: &dec::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::DEC(AddressingMode::AbsoluteX),
        opcode: 0xDE,
        microcode_sequence: &dec::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::DEX(AddressingMode::Implied),
        opcode: 0xCA,
        microcode_sequence: &dex::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::DEY(AddressingMode::Implied),
        opcode: 0x88,
        microcode_sequence: &dey::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::INC(AddressingMode::ZeroPage),
        opcode: 0xE6,
        microcode_sequence: &inc::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::INC(AddressingMode::ZeroPageX),
        opcode: 0xF6,
        microcode_sequence: &inc::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::INC(AddressingMode::Absolute),
        opcode: 0xEE,
        microcode_sequence: &inc::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::INC(AddressingMode::AbsoluteX),
        opcode: 0xFE,
        microcode_sequence: &inc::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::INX(AddressingMode::Implied),
        opcode: 0xE8,
        microcode_sequence: &inx::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::INY(AddressingMode::Implied),
        opcode: 0xC8,
        microcode_sequence: &iny::IMPLIED,
    },
    // --- Arithmetic Instructions ---
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::Immediate),
        opcode: 0x69,
        microcode_sequence: &adc::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::ZeroPage),
        opcode: 0x65,
        microcode_sequence: &adc::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::ZeroPageX),
        opcode: 0x75,
        microcode_sequence: &adc::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::Absolute),
        opcode: 0x6D,
        microcode_sequence: &adc::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::AbsoluteX),
        opcode: 0x7D,
        microcode_sequence: &adc::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::AbsoluteY),
        opcode: 0x79,
        microcode_sequence: &adc::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::IndirectX),
        opcode: 0x61,
        microcode_sequence: &adc::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::ADC(AddressingMode::IndirectY),
        opcode: 0x71,
        microcode_sequence: &adc::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::Immediate),
        opcode: 0xE9,
        microcode_sequence: &sbc::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::ZeroPage),
        opcode: 0xE5,
        microcode_sequence: &sbc::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::ZeroPageX),
        opcode: 0xF5,
        microcode_sequence: &sbc::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::Absolute),
        opcode: 0xED,
        microcode_sequence: &sbc::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::AbsoluteX),
        opcode: 0xFD,
        microcode_sequence: &sbc::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::AbsoluteY),
        opcode: 0xF9,
        microcode_sequence: &sbc::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::IndirectX),
        opcode: 0xE1,
        microcode_sequence: &sbc::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::SBC(AddressingMode::IndirectY),
        opcode: 0xF1,
        microcode_sequence: &sbc::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::Immediate),
        opcode: 0x29,
        microcode_sequence: &and::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::ZeroPage),
        opcode: 0x25,
        microcode_sequence: &and::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::ZeroPageX),
        opcode: 0x35,
        microcode_sequence: &and::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::Absolute),
        opcode: 0x2D,
        microcode_sequence: &and::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::AbsoluteX),
        opcode: 0x3D,
        microcode_sequence: &and::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::AbsoluteY),
        opcode: 0x39,
        microcode_sequence: &and::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::IndirectX),
        opcode: 0x21,
        microcode_sequence: &and::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::AND(AddressingMode::IndirectY),
        opcode: 0x31,
        microcode_sequence: &and::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::Immediate),
        opcode: 0x09,
        microcode_sequence: &ora::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::ZeroPage),
        opcode: 0x05,
        microcode_sequence: &ora::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::ZeroPageX),
        opcode: 0x15,
        microcode_sequence: &ora::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::Absolute),
        opcode: 0x0D,
        microcode_sequence: &ora::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::AbsoluteX),
        opcode: 0x1D,
        microcode_sequence: &ora::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::AbsoluteY),
        opcode: 0x19,
        microcode_sequence: &ora::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::IndirectX),
        opcode: 0x01,
        microcode_sequence: &ora::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::ORA(AddressingMode::IndirectY),
        opcode: 0x11,
        microcode_sequence: &ora::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::Immediate),
        opcode: 0x49,
        microcode_sequence: &eor::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::ZeroPage),
        opcode: 0x45,
        microcode_sequence: &eor::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::ZeroPageX),
        opcode: 0x55,
        microcode_sequence: &eor::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::Absolute),
        opcode: 0x4D,
        microcode_sequence: &eor::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::AbsoluteX),
        opcode: 0x5D,
        microcode_sequence: &eor::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::AbsoluteY),
        opcode: 0x59,
        microcode_sequence: &eor::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::IndirectX),
        opcode: 0x41,
        microcode_sequence: &eor::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::EOR(AddressingMode::IndirectY),
        opcode: 0x51,
        microcode_sequence: &eor::INDIRECT_Y,
    },
    // --- Bitwise Shift & Rotate Instructions ---
    InstructionVariant {
        instruction: Instruction::ASL(AddressingMode::Accumulator),
        opcode: 0x0A,
        microcode_sequence: &asl::ACCUMULATOR,
    },
    InstructionVariant {
        instruction: Instruction::ASL(AddressingMode::ZeroPage),
        opcode: 0x06,
        microcode_sequence: &asl::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::ASL(AddressingMode::ZeroPageX),
        opcode: 0x16,
        microcode_sequence: &asl::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::ASL(AddressingMode::Absolute),
        opcode: 0x0E,
        microcode_sequence: &asl::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::ASL(AddressingMode::AbsoluteX),
        opcode: 0x1E,
        microcode_sequence: &asl::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::LSR(AddressingMode::Accumulator),
        opcode: 0x4A,
        microcode_sequence: &lsr::ACCUMULATOR,
    },
    InstructionVariant {
        instruction: Instruction::LSR(AddressingMode::ZeroPage),
        opcode: 0x46,
        microcode_sequence: &lsr::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::LSR(AddressingMode::ZeroPageX),
        opcode: 0x56,
        microcode_sequence: &lsr::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::LSR(AddressingMode::Absolute),
        opcode: 0x4E,
        microcode_sequence: &lsr::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::LSR(AddressingMode::AbsoluteX),
        opcode: 0x5E,
        microcode_sequence: &lsr::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::ROL(AddressingMode::Accumulator),
        opcode: 0x2A,
        microcode_sequence: &rol::ACCUMULATOR,
    },
    InstructionVariant {
        instruction: Instruction::ROL(AddressingMode::ZeroPage),
        opcode: 0x26,
        microcode_sequence: &rol::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::ROL(AddressingMode::ZeroPageX),
        opcode: 0x36,
        microcode_sequence: &rol::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::ROL(AddressingMode::Absolute),
        opcode: 0x2E,
        microcode_sequence: &rol::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::ROL(AddressingMode::AbsoluteX),
        opcode: 0x3E,
        microcode_sequence: &rol::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::ROR(AddressingMode::Accumulator),
        opcode: 0x6A,
        microcode_sequence: &ror::ACCUMULATOR,
    },
    InstructionVariant {
        instruction: Instruction::ROR(AddressingMode::ZeroPage),
        opcode: 0x66,
        microcode_sequence: &ror::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::ROR(AddressingMode::ZeroPageX),
        opcode: 0x76,
        microcode_sequence: &ror::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::ROR(AddressingMode::Absolute),
        opcode: 0x6E,
        microcode_sequence: &ror::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::ROR(AddressingMode::AbsoluteX),
        opcode: 0x7E,
        microcode_sequence: &ror::ABSOLUTE_X,
    },
    // --- Flag Instructions ---
    InstructionVariant {
        instruction: Instruction::CLC(AddressingMode::Implied),
        opcode: 0x18,
        microcode_sequence: &clc::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::CLD(AddressingMode::Implied),
        opcode: 0xD8,
        microcode_sequence: &cld::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::CLI(AddressingMode::Implied),
        opcode: 0x58,
        microcode_sequence: &cli::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::CLV(AddressingMode::Implied),
        opcode: 0xB8,
        microcode_sequence: &clv::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::SEC(AddressingMode::Implied),
        opcode: 0x38,
        microcode_sequence: &sec::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::SED(AddressingMode::Implied),
        opcode: 0xF8,
        microcode_sequence: &sed::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::SEI(AddressingMode::Implied),
        opcode: 0x78,
        microcode_sequence: &sei::IMPLIED,
    },
    // --- Compare Instructions ---
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::Immediate),
        opcode: 0xC9,
        microcode_sequence: &cmp::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::ZeroPage),
        opcode: 0xC5,
        microcode_sequence: &cmp::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::ZeroPageX),
        opcode: 0xD5,
        microcode_sequence: &cmp::ZEROPAGE_X,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::Absolute),
        opcode: 0xCD,
        microcode_sequence: &cmp::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::AbsoluteX),
        opcode: 0xDD,
        microcode_sequence: &cmp::ABSOLUTE_X,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::AbsoluteY),
        opcode: 0xD9,
        microcode_sequence: &cmp::ABSOLUTE_Y,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::IndirectX),
        opcode: 0xC1,
        microcode_sequence: &cmp::INDIRECT_X,
    },
    InstructionVariant {
        instruction: Instruction::CMP(AddressingMode::IndirectY),
        opcode: 0xD1,
        microcode_sequence: &cmp::INDIRECT_Y,
    },
    InstructionVariant {
        instruction: Instruction::CPX(AddressingMode::Immediate),
        opcode: 0xE0,
        microcode_sequence: &cpx::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::CPX(AddressingMode::ZeroPage),
        opcode: 0xE4,
        microcode_sequence: &cpx::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::CPX(AddressingMode::Absolute),
        opcode: 0xEC,
        microcode_sequence: &cpx::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::CPY(AddressingMode::Immediate),
        opcode: 0xC0,
        microcode_sequence: &cpy::IMMEDIATE,
    },
    InstructionVariant {
        instruction: Instruction::CPY(AddressingMode::ZeroPage),
        opcode: 0xC4,
        microcode_sequence: &cpy::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::CPY(AddressingMode::Absolute),
        opcode: 0xCC,
        microcode_sequence: &cpy::ABSOLUTE,
    },
    // --- Branch Instructions ---
    InstructionVariant {
        instruction: Instruction::BCC(AddressingMode::Relative),
        opcode: 0x90,
        microcode_sequence: &bcc::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BCS(AddressingMode::Relative),
        opcode: 0xB0,
        microcode_sequence: &bcs::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BEQ(AddressingMode::Relative),
        opcode: 0xF0,
        microcode_sequence: &beq::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BMI(AddressingMode::Relative),
        opcode: 0x30,
        microcode_sequence: &bmi::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BNE(AddressingMode::Relative),
        opcode: 0xD0,
        microcode_sequence: &bne::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BPL(AddressingMode::Relative),
        opcode: 0x10,
        microcode_sequence: &bpl::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BVC(AddressingMode::Relative),
        opcode: 0x50,
        microcode_sequence: &bvc::RELATIVE,
    },
    InstructionVariant {
        instruction: Instruction::BVS(AddressingMode::Relative),
        opcode: 0x70,
        microcode_sequence: &bvs::RELATIVE,
    },
    // --- Jump & Subroutine Instructions ---
    InstructionVariant {
        instruction: Instruction::JMP(AddressingMode::Absolute),
        opcode: 0x4C,
        microcode_sequence: &jmp::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::JMP(AddressingMode::Indirect),
        opcode: 0x6C,
        microcode_sequence: &jmp::INDIRECT,
    },
    InstructionVariant {
        instruction: Instruction::JSR(AddressingMode::Absolute),
        opcode: 0x20,
        microcode_sequence: &jsr::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::RTS(AddressingMode::Implied),
        opcode: 0x60,
        microcode_sequence: &rts::IMPLIED,
    },
    // --- Interrupt Instructions ---
    InstructionVariant {
        instruction: Instruction::BRK(AddressingMode::Implied),
        opcode: 0x00,
        microcode_sequence: &brk::IMPLIED,
    },
    InstructionVariant {
        instruction: Instruction::RTI(AddressingMode::Implied),
        opcode: 0x40,
        microcode_sequence: &rti::IMPLIED,
    },
    // --- Other Instructions ---
    InstructionVariant {
        instruction: Instruction::BIT(AddressingMode::ZeroPage),
        opcode: 0x24,
        microcode_sequence: &bit::ZEROPAGE,
    },
    InstructionVariant {
        instruction: Instruction::BIT(AddressingMode::Absolute),
        opcode: 0x2C,
        microcode_sequence: &bit::ABSOLUTE,
    },
    InstructionVariant {
        instruction: Instruction::NOP(AddressingMode::Implied),
        opcode: 0xEA,
        microcode_sequence: &nop::IMPLIED,
    },
];

#[cfg(test)]
mod unit_tests {
    use super::*;

    // Test addressing mode coverage
    #[test]
    fn test_addressing_mode_coverage() {
        use std::collections::HashSet;
        let mut used_modes = HashSet::new();

        for variant in &INSTRUCTION_VARIANTS {
            match &variant.instruction {
                Instruction::LDA(mode)
                | Instruction::STA(mode)
                | Instruction::ADC(mode)
                | Instruction::SBC(mode)
                | Instruction::AND(mode)
                | Instruction::ORA(mode)
                | Instruction::EOR(mode)
                | Instruction::ASL(mode)
                | Instruction::LSR(mode)
                | Instruction::ROL(mode)
                | Instruction::ROR(mode)
                | Instruction::CMP(mode)
                | Instruction::LDX(mode)
                | Instruction::STX(mode)
                | Instruction::LDY(mode)
                | Instruction::STY(mode)
                | Instruction::CPX(mode)
                | Instruction::CPY(mode)
                | Instruction::INC(mode)
                | Instruction::DEC(mode)
                | Instruction::BIT(mode)
                | Instruction::JMP(mode) => {
                    used_modes.insert(*mode);
                }
                _ => {} // Instructions without addressing modes
            }
        }

        // Verify we're using most common addressing modes
        assert!(used_modes.contains(&AddressingMode::Immediate));
        assert!(used_modes.contains(&AddressingMode::ZeroPage));
        assert!(used_modes.contains(&AddressingMode::Absolute));
        assert!(used_modes.contains(&AddressingMode::AbsoluteX));
        assert!(used_modes.contains(&AddressingMode::AbsoluteY));
        assert!(used_modes.contains(&AddressingMode::IndirectX));
        assert!(used_modes.contains(&AddressingMode::IndirectY));
        assert!(used_modes.contains(&AddressingMode::Accumulator));
    }

    // Test instruction frequency (some instructions should have multiple addressing modes)
    #[test]
    fn test_instruction_variants_completeness() {
        use std::collections::HashMap;
        let mut instruction_counts = HashMap::new();

        for variant in &INSTRUCTION_VARIANTS {
            let base_instruction = match &variant.instruction {
                Instruction::LDA(_) => "LDA",
                Instruction::STA(_) => "STA",
                Instruction::ADC(_) => "ADC",
                Instruction::SBC(_) => "SBC",
                Instruction::AND(_) => "AND",
                Instruction::ORA(_) => "ORA",
                Instruction::EOR(_) => "EOR",
                _ => continue,
            };
            *instruction_counts.entry(base_instruction).or_insert(0) += 1;
        }

        // These instructions should have multiple addressing modes
        assert!(*instruction_counts.get("LDA").unwrap_or(&0) >= 8); // LDA has 8 modes
        assert!(*instruction_counts.get("STA").unwrap_or(&0) >= 6); // STA has 6 modes
        assert!(*instruction_counts.get("ADC").unwrap_or(&0) >= 8); // ADC has 8 modes
        assert!(*instruction_counts.get("SBC").unwrap_or(&0) >= 8); // SBC has 8 modes
    }
}
