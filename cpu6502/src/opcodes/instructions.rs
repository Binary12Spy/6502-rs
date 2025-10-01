//! 6502 Instructions

use super::addressing_modes::AddressingMode;

/// 6502 Instructions
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Instruction {
    /// --- Transfer Instructions ---
    /// Load Accumulator
    LDA(AddressingMode),
    /// Load X Register
    LDX(AddressingMode),
    /// Load Y Register
    LDY(AddressingMode),
    /// Store Accumulator
    STA(AddressingMode),
    /// Store X Register
    STX(AddressingMode),
    /// Store Y Register
    STY(AddressingMode),
    /// Transfer Accumulator to X
    TAX(AddressingMode),
    /// Transfer Accumulator to Y
    TAY(AddressingMode),
    /// Transfer Stack Pointer to X
    TSX(AddressingMode),
    /// Transfer X to Accumulator
    TXA(AddressingMode),
    /// Transfer X to Stack Pointer
    TXS(AddressingMode),
    /// Transfer Y to Accumulator
    TYA(AddressingMode),

    /// --- Stack Instructions ---
    /// Push Accumulator
    PHA(AddressingMode),
    /// Push Processor Status
    PHP(AddressingMode),
    /// Pull Accumulator
    PLA(AddressingMode),
    /// Pull Processor Status
    PLP(AddressingMode),

    /// --- Increment & Decrement Instructions ---
    /// Decrement Memory
    DEC(AddressingMode),
    /// Decrement X Register
    DEX(AddressingMode),
    /// Decrement Y Register
    DEY(AddressingMode),
    /// Increment Memory
    INC(AddressingMode),
    /// Increment X Register
    INX(AddressingMode),
    /// Increment Y Register
    INY(AddressingMode),

    /// --- Arithmetic Instructions ---
    /// Add with Carry
    ADC(AddressingMode),
    /// Subtract with Carry
    SBC(AddressingMode),

    // --- Logical Instructions ---
    /// Logical AND
    AND(AddressingMode),
    /// Logical OR
    ORA(AddressingMode),
    /// Exclusive OR
    EOR(AddressingMode),

    /// --- Bitwise Shift & Rotate Instructions ---
    /// Arithmetic Shift Left
    ASL(AddressingMode),
    /// Logical Shift Right
    LSR(AddressingMode),
    /// Rotate Left
    ROL(AddressingMode),
    /// Rotate Right
    ROR(AddressingMode),

    /// --- Flag Instructions ---
    /// Clear Carry Flag
    CLC(AddressingMode),
    /// Clear Decimal Mode
    CLD(AddressingMode),
    /// Clear Interrupt Disable
    CLI(AddressingMode),
    /// Clear Overflow Flag
    CLV(AddressingMode),
    /// Set Carry Flag
    SEC(AddressingMode),
    /// Set Decimal Mode
    SED(AddressingMode),
    /// Set Interrupt Disable
    SEI(AddressingMode),

    /// --- Compare Instructions ---
    /// Compare Accumulator
    CMP(AddressingMode),
    /// Compare X Register
    CPX(AddressingMode),
    /// Compare Y Register
    CPY(AddressingMode),

    /// --- Branch Instructions ---
    /// Branch if Carry Clear
    BCC(AddressingMode),
    /// Branch if Carry Set
    BCS(AddressingMode),
    /// Branch if Equal
    BEQ(AddressingMode),
    /// Branch if Minus
    BMI(AddressingMode),
    /// Branch if Not Equal
    BNE(AddressingMode),
    /// Branch if Positive
    BPL(AddressingMode),
    /// Branch if Overflow Clear
    BVC(AddressingMode),
    /// Branch if Overflow Set
    BVS(AddressingMode),

    /// --- Jump & Subroutine Instructions ---
    /// Jump
    JMP(AddressingMode),
    /// Jump to Subroutine
    JSR(AddressingMode),
    /// Return from Subroutine
    RTS(AddressingMode),

    /// --- Interrupt Instructions ---
    /// Force Interrupt
    BRK(AddressingMode),
    /// Return from Interrupt
    RTI(AddressingMode),

    /// --- Other Instructions ---
    /// Bit Test
    BIT(AddressingMode),
    /// No Operation
    NOP(AddressingMode),
}
