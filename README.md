# 6502-rs

A cycle-accurate 6502 CPU emulator written in Rust.

## Overview

This project implements a precise emulation of the MOS Technology 6502 microprocessor, focusing on cycle-accurate timing and comprehensive instruction support. The emulator is built with a modular architecture that separates concerns between the CPU core, bus system, and memory devices.

## Architecture

- **cpu6502**: Core 6502 CPU implementation with microcode-based instruction execution
- **bus**: Bus controller for managing memory-mapped devices  
- **ram**: Random Access Memory implementation
- **rom**: Read-Only Memory implementation

The CPU executes instructions using microcode sequences that accurately replicate the timing and behavior of the original 6502, including page boundary crossing penalties and proper flag handling.

## Features

- Cycle-accurate instruction execution
- Complete 6502 instruction set with all addressing modes
- Memory-mapped device support via bus abstraction
- Comprehensive error handling
- Extensive unit test coverage

## Roadmap

### Core Implementation
- [ ] Complete unit tests for all official 6502 instructions
- [ ] Full address mode validation and testing
- [ ] IRQ/NMI interrupt handling
- [ ] Decimal mode arithmetic support

### System Integration  
- [ ] Example emulator binary with basic I/O
- [ ] Configuration system for different memory layouts
- [ ] Save state functionality

### Hardware Extensions
- [ ] Audio Processing Unit (APU) implementation
- [ ] System-specific sound chips (2A03, SID, etc.)
- [ ] Video controller interfaces
- [ ] Timer and I/O device implementations

### Tools and Utilities
- [ ] Assembly language support and tooling
- [ ] Debugging interface with breakpoints
- [ ] Performance profiling and analysis
- [ ] ROM loading and validation utilities

## License

This project is released into the public domain under the Unlicense.
