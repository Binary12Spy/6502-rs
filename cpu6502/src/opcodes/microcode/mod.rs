/// Add with Carry
pub(crate) mod adc;
/// Logical AND
pub(crate) mod and;
/// Arithmetic Shift Left
pub(crate) mod asl;
/// Branch if Carry Clear
pub(crate) mod bcc;
/// Branch if Carry Set
pub(crate) mod bcs;
/// Branch if Equal
pub(crate) mod beq;
/// Bit Test
pub(crate) mod bit;
/// Branch if Minus
pub(crate) mod bmi;
/// Branch if Not Equal
pub(crate) mod bne;
/// Branch if Positive
pub(crate) mod bpl;
/// Force Interrupt
pub(crate) mod brk;
/// Branch if Overflow Clear
pub(crate) mod bvc;
/// Branch if Overflow Set
pub(crate) mod bvs;
/// Clear Carry Flag
pub(crate) mod clc;
/// Clear Decimal Mode Flag
pub(crate) mod cld;
/// Clear Interrupt Disable Flag
pub(crate) mod cli;
/// Clear Overflow Flag
pub(crate) mod clv;
/// Compare Accumulator
pub(crate) mod cmp;
/// Common microcode steps used across multiple instructions
pub(crate) mod common;
/// Compare X Register
pub(crate) mod cpx;
/// Compare Y Register
pub(crate) mod cpy;
/// Decrement Memory
pub(crate) mod dec;
/// Decrement X Register
pub(crate) mod dex;
/// Decrement Y Register
pub(crate) mod dey;
/// Logical Exclusive OR
pub(crate) mod eor;
/// Increment Memory
pub(crate) mod inc;
/// Increment X Register
pub(crate) mod inx;
/// Increment Y Register
pub(crate) mod iny;
/// Jump
pub(crate) mod jmp;
/// Jump to Subroutine
pub(crate) mod jsr;
/// Load Accumulator
pub(crate) mod lda;
/// Load X Register
pub(crate) mod ldx;
/// Load Y Register
pub(crate) mod ldy;
/// Logical Shift Right
pub(crate) mod lsr;
/// No Operation
pub(crate) mod nop;
/// Logical Inclusive OR
pub(crate) mod ora;
/// Push Accumulator onto Stack
pub(crate) mod pha;
/// Push flags onto Stack
pub(crate) mod php;
/// Pull Accumulator from Stack
pub(crate) mod pla;
/// Pull Flags from Stack
pub(crate) mod plp;
/// Rotate Left
pub(crate) mod rol;
/// Rotate Right
pub(crate) mod ror;
/// Return from Interrupt
pub(crate) mod rti;
/// Return from Subroutine
pub(crate) mod rts;
/// Subtract with Carry
pub(crate) mod sbc;
/// Set Carry Flag
pub(crate) mod sec;
/// Set Decimal Mode Flag
pub(crate) mod sed;
/// Set Interrupt Disable Flag
pub(crate) mod sei;
/// Store Accumulator
pub(crate) mod sta;
/// Store X Register
pub(crate) mod stx;
/// Store Y Register
pub(crate) mod sty;
/// Transfer Accumulator to X Register
pub(crate) mod tax;
/// Transfer Accumulator to Y Register
pub(crate) mod tay;
/// Transfer Stack Pointer to X Register
pub(crate) mod tsx;
/// Transfer X Register to Accumulator
pub(crate) mod txa;
/// Transfer X Register to Stack Pointer
pub(crate) mod txs;
/// Transfer Y Register to Accumulator
pub(crate) mod tya;

use crate::cpu::Cpu;
use crate::errors::CpuError;

/// Type alias for a microcode step function
pub(crate) type MicrocodeStep = fn(&mut Cpu) -> Result<OperationResult, CpuError>;
/// Type alias for a microcode sequence of fixed length N
pub(crate) type MicrocodeSequence<const N: usize> = [MicrocodeStep; N];

/// Result of a microcode step execution
#[derive(Debug, PartialEq)]
pub(crate) enum OperationResult {
    /// Continue to the next step
    Continue,
    /// Indicates a page boundary penalty, with the number of extra cycles
    PageBoundaryPenalty(u8),
    /// Break the current instruction execution early (e.g., for BCC instruction)
    Break,
}
