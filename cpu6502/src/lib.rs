//! 6502 CPU Emulator Library

/// 6502 ALU operations
mod alu;
/// 6502 CPU implementation
pub mod cpu;
/// Errors related to CPU operations
pub mod errors;
/// 6502 Flags
pub mod flags;
/// 6502 opcode variants
pub mod opcodes;
/// 6502 Registers
pub mod registers;

#[cfg(test)]
mod test_cpu_builder;
