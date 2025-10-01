//! Flags register for the 6502 CPU

/// The flags register is an 8-bit register where each bit represents a specific status flag.
/// The flags are as follows (from least significant bit to most significant bit):
/// - Bit 0: Carry Flag (C)
/// - Bit 1: Zero Flag (Z)
/// - Bit 2: Interrupt Disable (I)
/// - Bit 3: Decimal Mode (D)
/// - Bit 4: Break Command (B)
/// - Bit 5: Unused (always set to 1)
/// - Bit 6: Overflow Flag (V)
/// - Bit 7: Negative Flag (N)
#[derive(Clone, Copy, Debug)]
pub struct Flags {
    /// Carry Flag
    pub carry: bool,
    /// Zero Flag
    pub zero: bool,
    /// Interrupt Disable
    pub interrupt_disable: bool,
    /// Decimal Mode
    pub decimal_mode: bool,
    /// Break Command
    pub break_command: bool,
    /// Unused (always set to 1)
    pub unused: bool,
    /// Overflow Flag
    pub overflow: bool,
    /// Negative Flag
    pub negative: bool,
}

impl Default for Flags {
    /// Create a new Flags instance with default values.
    ///
    /// Defaults:
    /// - Carry: false
    /// - Zero: false
    /// - Interrupt Disable: true
    /// - Decimal Mode: false
    /// - Break Command: false
    /// - Unused: true
    /// - Overflow: false
    /// - Negative: false
    /// The unused bit is always set to true.
    ///
    /// # Example
    /// ```
    /// use cpu6502::flags::Flags;
    /// let flags = Flags::default();
    /// assert_eq!(flags.carry, false);
    /// assert_eq!(flags.zero, false);
    /// assert_eq!(flags.interrupt_disable, true);
    /// assert_eq!(flags.decimal_mode, false);
    /// assert_eq!(flags.break_command, false);
    /// assert_eq!(flags.unused, true);
    /// assert_eq!(flags.overflow, false);
    /// assert_eq!(flags.negative, false);
    /// ```
    fn default() -> Flags {
        Flags {
            carry: false,
            zero: false,
            interrupt_disable: true,
            decimal_mode: false,
            break_command: false,
            unused: true,
            overflow: false,
            negative: false,
        }
    }
}

impl TryFrom<u8> for Flags {
    type Error = String;

    /// Convert a byte into a Flags struct.
    /// The unused bit (bit 5) must always be set to 1, otherwise an error is returned.
    ///
    /// # Errors
    /// Returns an error if the unused bit is not set to 1.
    ///
    /// # Example
    /// ```
    /// use cpu6502::flags::Flags;
    ///
    /// let byte = 0b00100000;
    /// let flags = Flags::try_from(byte);
    /// assert!(flags.is_ok());
    /// ```
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        if byte & 0b00100000 != 0b00100000 {
            return Err(format!("Invalid flags byte: {:08b}", byte));
        }
        let mut flags = Flags::default();
        flags.carry = byte & 0b00000001 != 0;
        flags.zero = byte & 0b00000010 != 0;
        flags.interrupt_disable = byte & 0b00000100 != 0;
        flags.decimal_mode = byte & 0b00001000 != 0;
        flags.break_command = byte & 0b00010000 != 0;
        flags.overflow = byte & 0b01000000 != 0;
        flags.negative = byte & 0b10000000 != 0;
        Ok(flags)
    }
}

impl Into<u8> for Flags {
    /// Convert the Flags struct into a byte.
    ///
    /// # Returns
    /// A byte representation of the Flags struct.
    ///
    /// # Example
    /// ```
    /// use cpu6502::flags::Flags;
    ///
    /// let flags = Flags {
    ///     carry: true,
    ///     zero: false,
    ///     interrupt_disable: true,
    ///     decimal_mode: false,
    ///     break_command: false,
    ///     unused: true,
    ///     overflow: false,
    ///     negative: false,
    /// };
    /// let byte: u8 = flags.into();
    /// assert_eq!(byte, 0b00100101);
    /// ```
    fn into(self) -> u8 {
        let mut byte = 0b00100000;
        if self.carry {
            byte |= 0b00000001;
        }
        if self.zero {
            byte |= 0b00000010;
        }
        if self.interrupt_disable {
            byte |= 0b00000100;
        }
        if self.decimal_mode {
            byte |= 0b00001000;
        }
        if self.break_command {
            byte |= 0b00010000;
        }
        if self.overflow {
            byte |= 0b01000000;
        }
        if self.negative {
            byte |= 0b10000000;
        }
        byte
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    // Test Default implementation
    #[test]
    fn test_flags_default() {
        let flags = Flags::default();

        assert_eq!(flags.carry, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.interrupt_disable, true); // Should be true by default
        assert_eq!(flags.decimal_mode, false);
        assert_eq!(flags.break_command, false);
        assert_eq!(flags.unused, true); // Should always be true
        assert_eq!(flags.overflow, false);
        assert_eq!(flags.negative, false);
    }

    // Test TryFrom<u8> implementation
    #[test]
    fn test_flags_try_from_valid_byte() {
        // Test with minimal valid byte (only unused bit set)
        let byte = 0b00100000;
        let flags = Flags::try_from(byte).unwrap();

        assert_eq!(flags.carry, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.interrupt_disable, false);
        assert_eq!(flags.decimal_mode, false);
        assert_eq!(flags.break_command, false);
        assert_eq!(flags.unused, true);
        assert_eq!(flags.overflow, false);
        assert_eq!(flags.negative, false);
    }

    #[test]
    fn test_flags_try_from_all_flags_set() {
        // Test with all flags set (0b11111111)
        let byte = 0b11111111;
        let flags = Flags::try_from(byte).unwrap();

        assert_eq!(flags.carry, true);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.interrupt_disable, true);
        assert_eq!(flags.decimal_mode, true);
        assert_eq!(flags.break_command, true);
        assert_eq!(flags.unused, true);
        assert_eq!(flags.overflow, true);
        assert_eq!(flags.negative, true);
    }

    #[test]
    fn test_flags_try_from_individual_bits() {
        // Test Carry flag (bit 0)
        let flags = Flags::try_from(0b00100001).unwrap();
        assert_eq!(flags.carry, true);
        assert_eq!(flags.zero, false);

        // Test Zero flag (bit 1)
        let flags = Flags::try_from(0b00100010).unwrap();
        assert_eq!(flags.carry, false);
        assert_eq!(flags.zero, true);

        // Test Interrupt Disable flag (bit 2)
        let flags = Flags::try_from(0b00100100).unwrap();
        assert_eq!(flags.interrupt_disable, true);

        // Test Decimal Mode flag (bit 3)
        let flags = Flags::try_from(0b00101000).unwrap();
        assert_eq!(flags.decimal_mode, true);

        // Test Break Command flag (bit 4)
        let flags = Flags::try_from(0b00110000).unwrap();
        assert_eq!(flags.break_command, true);

        // Test Overflow flag (bit 6)
        let flags = Flags::try_from(0b01100000).unwrap();
        assert_eq!(flags.overflow, true);

        // Test Negative flag (bit 7)
        let flags = Flags::try_from(0b10100000).unwrap();
        assert_eq!(flags.negative, true);
    }

    #[test]
    fn test_flags_try_from_invalid_byte_no_unused_bit() {
        // Test with unused bit not set (should fail)
        let byte = 0b11011111; // All bits set except unused (bit 5)
        let result = Flags::try_from(byte);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid flags byte"));
    }

    #[test]
    fn test_flags_try_from_invalid_byte_zero() {
        // Test with zero byte (unused bit not set)
        let byte = 0b00000000;
        let result = Flags::try_from(byte);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid flags byte"));
    }

    // Test Into<u8> implementation
    #[test]
    fn test_flags_into_u8_default() {
        let flags = Flags::default();
        let byte: u8 = flags.into();

        // Default should be 0b00100100 (unused=1, interrupt_disable=1)
        assert_eq!(byte, 0b00100100);
    }

    #[test]
    fn test_flags_into_u8_all_flags_set() {
        let flags = Flags {
            carry: true,
            zero: true,
            interrupt_disable: true,
            decimal_mode: true,
            break_command: true,
            unused: true,
            overflow: true,
            negative: true,
        };
        let byte: u8 = flags.into();

        assert_eq!(byte, 0b11111111);
    }

    #[test]
    fn test_flags_into_u8_individual_bits() {
        // Test Carry flag only
        let flags = Flags {
            carry: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        assert_eq!(byte & 0b00000001, 0b00000001); // Carry bit set

        // Test Zero flag only
        let flags = Flags {
            zero: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        assert_eq!(byte & 0b00000010, 0b00000010); // Zero bit set

        // Test Decimal Mode flag only
        let flags = Flags {
            decimal_mode: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        assert_eq!(byte & 0b00001000, 0b00001000); // Decimal mode bit set

        // Test Overflow flag only
        let flags = Flags {
            overflow: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        assert_eq!(byte & 0b01000000, 0b01000000); // Overflow bit set

        // Test Negative flag only
        let flags = Flags {
            negative: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        assert_eq!(byte & 0b10000000, 0b10000000); // Negative bit set
    }

    #[test]
    fn test_flags_into_u8_unused_bit_always_set() {
        // Even when unused is set to false, it should still be set in the byte
        let flags = Flags {
            unused: false, // This should still result in bit 5 being set
            ..Default::default()
        };
        let byte: u8 = flags.into();

        // The implementation always sets bit 5, regardless of the unused field
        assert_eq!(byte & 0b00100000, 0b00100000);
    }

    // Round-trip tests (TryFrom -> Into -> TryFrom)
    #[test]
    fn test_flags_roundtrip_conversion() {
        let test_bytes = [
            0b00100000, // Minimal (only unused bit)
            0b00100101, // Carry + Interrupt Disable + Unused
            0b00101010, // Zero + Decimal + Unused
            0b11111111, // All bits set
            0b10100000, // Negative + Unused
            0b01100000, // Overflow + Unused
        ];

        for &original_byte in &test_bytes {
            let flags = Flags::try_from(original_byte).unwrap();
            let converted_byte: u8 = flags.into();
            let flags_again = Flags::try_from(converted_byte).unwrap();

            // Check that all fields are preserved
            assert_eq!(flags.carry, flags_again.carry);
            assert_eq!(flags.zero, flags_again.zero);
            assert_eq!(flags.interrupt_disable, flags_again.interrupt_disable);
            assert_eq!(flags.decimal_mode, flags_again.decimal_mode);
            assert_eq!(flags.break_command, flags_again.break_command);
            assert_eq!(flags.unused, flags_again.unused);
            assert_eq!(flags.overflow, flags_again.overflow);
            assert_eq!(flags.negative, flags_again.negative);
        }
    }

    // Test specific flag combinations that are common in CPU operations
    #[test]
    fn test_flags_common_combinations() {
        // Test addition result: carry + zero
        let flags = Flags {
            carry: true,
            zero: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        let restored_flags = Flags::try_from(byte).unwrap();

        assert_eq!(restored_flags.carry, true);
        assert_eq!(restored_flags.zero, true);
        assert_eq!(restored_flags.negative, false);
        assert_eq!(restored_flags.overflow, false);

        // Test subtraction result: negative + overflow
        let flags = Flags {
            negative: true,
            overflow: true,
            unused: true,
            ..Default::default()
        };
        let byte: u8 = flags.into();
        let restored_flags = Flags::try_from(byte).unwrap();

        assert_eq!(restored_flags.negative, true);
        assert_eq!(restored_flags.overflow, true);
        assert_eq!(restored_flags.carry, false);
        assert_eq!(restored_flags.zero, false);
    }

    // Test edge cases
    #[test]
    fn test_flags_edge_cases() {
        // Test maximum valid byte value
        let flags = Flags::try_from(0xFF).unwrap();
        let byte: u8 = flags.into();
        assert_eq!(byte, 0xFF);

        // Test minimum valid byte value
        let flags = Flags::try_from(0x20).unwrap();
        let byte: u8 = flags.into();
        assert_eq!(byte, 0x20);
    }

    // Test Clone and Copy traits
    #[test]
    fn test_flags_clone_copy() {
        let original_flags = Flags {
            carry: true,
            zero: false,
            negative: true,
            unused: true,
            ..Default::default()
        };

        // Test copy
        let copied_flags = original_flags;
        assert_eq!(copied_flags.carry, original_flags.carry);
        assert_eq!(copied_flags.zero, original_flags.zero);
        assert_eq!(copied_flags.negative, original_flags.negative);

        // Test clone
        let cloned_flags = original_flags.clone();
        assert_eq!(cloned_flags.carry, original_flags.carry);
        assert_eq!(cloned_flags.zero, original_flags.zero);
        assert_eq!(cloned_flags.negative, original_flags.negative);

        // Modify copy to ensure independence
        let mut modified_flags = original_flags;
        modified_flags.carry = false;
        assert_eq!(original_flags.carry, true); // Original unchanged
        assert_eq!(modified_flags.carry, false); // Copy changed
    }

    // Test Debug trait
    #[test]
    fn test_flags_debug() {
        let flags = Flags {
            carry: true,
            zero: false,
            unused: true,
            ..Default::default()
        };

        let debug_string = format!("{:?}", flags);
        assert!(debug_string.contains("carry: true"));
        assert!(debug_string.contains("zero: false"));
        assert!(debug_string.contains("Flags"));
    }
}
