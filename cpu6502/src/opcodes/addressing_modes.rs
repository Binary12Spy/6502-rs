//! Addressing modes for 6502 CPU emulation

/// 6502 Addressing Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    /// No Operand
    Implied, // (e.g., CLC, NOP)
    /// Immediate Value
    Immediate, // #$nn (LDA #$42)
    /// Zero Page
    ZeroPage, // $nn (LDA $10)
    /// Zero Page X
    ZeroPageX, // $nn,X (LDA $10,X)
    /// Zero Page Y
    ZeroPageY, // $nn,Y (LDA $10,Y - Rare, only used with LDX/LDY)
    /// Absolute
    Absolute, // $nnnn (LDA $2000)
    /// Absolute X
    AbsoluteX, // $nnnn,X (LDA $2000,X)
    /// Absolute Y
    AbsoluteY, // $nnnn,Y (LDA $2000,Y)
    /// Indirect
    Indirect, // ($nnnn)  (JMP ($3000))
    /// Indirect X
    IndirectX, // ($nn,X)  (LDA ($10,X))
    /// Indirect Y
    IndirectY, // ($nn),Y  (LDA ($10),Y)
    /// Relative
    Relative, // Branching instructions (BNE, BEQ)
    /// Accumulator
    Accumulator, // Accumulator (ASL A)
}
