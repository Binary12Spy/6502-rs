//! Unit tests for the ROM implementation
//! 
//! This module contains comprehensive tests for the 6502 ROM implementation,
//! testing ROM creation, data import/export, bus device functionality,
//! and error handling for read-only memory.

use rom::{Rom, rom_size::RomSize};
use bus::trait_bus_device::BusDevice;
use bus::errors::BusError;

// Test ROM creation and initialization
#[test]
fn test_rom_creation_default() {
    let rom = Rom::new(RomSize::_32K, 0x8000);
    
    // Test that ROM was created with correct size
    // We can't directly access memory field, so we test via export
    let exported = rom.export(0, 10);
    assert_eq!(exported.len(), 10);
    assert_eq!(exported, vec![0; 10]); // Should be initialized to zeros
}

#[test]
fn test_rom_creation_different_sizes() {
    let sizes = [
        (RomSize::_2K, 0x0800),
        (RomSize::_4K, 0x1000),
        (RomSize::_8K, 0x2000),
        (RomSize::_16K, 0x4000),
        (RomSize::_32K, 0x8000),
        (RomSize::_64K, 0x10000),
    ];
    
    for (size, expected_bytes) in sizes {
        let rom = Rom::new(size, 0x0000);
        let exported = rom.export(0, expected_bytes);
        assert_eq!(exported.len(), expected_bytes);
        assert_eq!(exported, vec![0; expected_bytes]);
    }
}

#[test]
fn test_rom_creation_different_start_addresses() {
    let start_addresses = [0x0000, 0x2000, 0x4000, 0x8000, 0xC000, 0xFF00];
    
    for start_addr in start_addresses {
        let rom = Rom::new(RomSize::_4K, start_addr);
        let exported = rom.export(0, 10);
        assert_eq!(exported, vec![0; 10]);
    }
}

// Test data import functionality
#[test]
fn test_import_basic() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    
    let result = rom.import(&data, 0);
    assert!(result.is_ok());
    
    // Verify data was imported
    let exported = rom.export(0, 4);
    assert_eq!(exported, data);
}

#[test]
fn test_import_with_offset() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    let data = vec![0x11, 0x22, 0x33];
    let offset = 100;
    
    let result = rom.import(&data, offset);
    assert!(result.is_ok());
    
    // Verify data was imported at correct offset
    let exported_before = rom.export(offset - 1, 1);
    assert_eq!(exported_before, vec![0x00]); // Should still be zero
    
    let exported_data = rom.export(offset, 3);
    assert_eq!(exported_data, data);
    
    let exported_after = rom.export(offset + 3, 1);
    assert_eq!(exported_after, vec![0x00]); // Should still be zero
}

#[test]
fn test_import_empty_data() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    let data: Vec<u8> = vec![];
    
    let result = rom.import(&data, 0);
    assert!(result.is_ok());
    
    // ROM should remain unchanged
    let exported = rom.export(0, 10);
    assert_eq!(exported, vec![0; 10]);
}

#[test]
fn test_import_full_rom() {
    let mut rom = Rom::new(RomSize::_2K, 0x0000);
    let data = vec![0xFF; 0x0800]; // Fill entire 2K ROM
    
    let result = rom.import(&data, 0);
    assert!(result.is_ok());
    
    // Verify entire ROM was filled
    let exported = rom.export(0, 0x0800);
    assert_eq!(exported.len(), 0x0800);
    assert_eq!(exported, data);
}

#[test]
fn test_import_exceeds_rom_size() {
    let mut rom = Rom::new(RomSize::_2K, 0x0000);
    let data = vec![0xFF; 0x0801]; // One byte too many for 2K ROM
    
    let result = rom.import(&data, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Data exceeds ROM size");
    
    // ROM should remain unchanged
    let exported = rom.export(0, 10);
    assert_eq!(exported, vec![0; 10]);
}

#[test]
fn test_import_with_offset_exceeds_rom() {
    let mut rom = Rom::new(RomSize::_2K, 0x0000);
    let data = vec![0xAA; 10];
    let offset = 0x0800 - 5; // This would go beyond ROM size
    
    let result = rom.import(&data, offset);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Data exceeds ROM size");
}

// Test data export functionality
#[test]
fn test_export_basic() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x9A];
    rom.import(&data, 10).unwrap();
    
    let exported = rom.export(10, 5);
    assert_eq!(exported, data);
}

#[test]
fn test_export_zero_length() {
    let rom = Rom::new(RomSize::_4K, 0x0000);
    let exported = rom.export(0, 0);
    assert_eq!(exported.len(), 0);
}

#[test]
fn test_export_exceeds_rom_size() {
    let rom = Rom::new(RomSize::_2K, 0x0000);
    // Try to export more data than ROM size
    let exported = rom.export(0, 0x1000); // Request 4K from 2K ROM
    assert_eq!(exported.len(), 0x0800); // Should only get 2K
}

#[test]
fn test_export_with_offset_exceeds_rom() {
    let rom = Rom::new(RomSize::_2K, 0x0000);
    let exported = rom.export(0x0700, 0x200); // Start near end, request more than available
    assert_eq!(exported.len(), 0x0100); // Should only get what's available
}

#[test]
fn test_export_offset_at_rom_boundary() {
    let rom = Rom::new(RomSize::_2K, 0x0000);
    // Test with offset exactly at end of ROM (should return empty)
    let exported = rom.export(0x0800, 100); // 2K = 0x0800 bytes
    assert_eq!(exported.len(), 0);
}

// Test BusDevice trait implementation - READ operations
#[test]
fn test_bus_device_read_basic() {
    let mut rom = Rom::new(RomSize::_4K, 0x2000);
    
    // Import data first
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    rom.import(&data, 0).unwrap();
    
    // Read data back via bus interface
    let result = rom.read(0x2000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xAA);
    
    let result = rom.read(0x2001);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xBB);
}

#[test]
fn test_bus_device_read_different_addresses() {
    let mut rom = Rom::new(RomSize::_8K, 0x4000);
    
    let test_data = [
        (0, 0x11), // Offset 0
        (1, 0x22), // Offset 1
        (0x1FFF, 0x33), // Last valid offset (0x2000 - 1)
    ];
    
    // Import test data at specific offsets
    for (offset, data) in test_data {
        rom.import(&[data], offset).unwrap();
    }
    
    // Read test data back via bus
    for (offset, expected) in test_data {
        let address = 0x4000 + offset as u16;
        let result = rom.read(address);
        assert!(result.is_ok(), "Failed to read from address 0x{:04X}", address);
        assert_eq!(result.unwrap(), expected, "Data mismatch at address 0x{:04X}", address);
    }
}

#[test]
fn test_bus_device_read_before_start_address() {
    let rom = Rom::new(RomSize::_4K, 0x8000);
    
    let result = rom.read(0x7FFF); // One address before start
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BusError::AddressOutOfRange(0x7FFF)));
}

#[test]
fn test_bus_device_read_after_end_address() {
    let rom = Rom::new(RomSize::_4K, 0x8000);
    
    let result = rom.read(0x9000); // Beyond end address (0x8000 + 0x1000)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BusError::AddressOutOfRange(0x9000)));
}

// Test BusDevice trait implementation - WRITE operations (should all fail)
#[test]
fn test_bus_device_write_fails_read_only() {
    let mut rom = Rom::new(RomSize::_4K, 0x8000);
    
    // All write attempts should fail with ReadOnly error
    let addresses = [0x8000, 0x8001, 0x8FFF]; // Various valid addresses
    
    for address in addresses {
        let result = rom.write(address, 0xFF);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BusError::ReadOnly(addr) if addr == address));
    }
}

#[test]
fn test_bus_device_write_fails_even_invalid_addresses() {
    let mut rom = Rom::new(RomSize::_4K, 0x8000);
    
    // Even invalid addresses should return ReadOnly error, not AddressOutOfRange
    // This tests that ROM checks for write permission before address validation
    let invalid_addresses = [0x7FFF, 0x9000];
    
    for address in invalid_addresses {
        let result = rom.write(address, 0xFF);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BusError::ReadOnly(addr) if addr == address));
    }
}

#[test]
fn test_bus_device_write_does_not_modify_data() {
    let mut rom = Rom::new(RomSize::_4K, 0x8000);
    
    // Import initial data
    let initial_data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    rom.import(&initial_data, 0).unwrap();
    
    // Attempt to write (should fail)
    let _ = rom.write(0x8000, 0xFF);
    let _ = rom.write(0x8001, 0x00);
    
    // Verify data remains unchanged
    let result = rom.read(0x8000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xAA);
    
    let result = rom.read(0x8001);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xBB);
}

// Test BusDevice trait methods that don't do anything for ROM
#[test]
fn test_bus_device_tick() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    
    // Import some data
    let data = vec![0xFF, 0xEE, 0xDD];
    rom.import(&data, 0).unwrap();
    
    // tick() should not panic or change anything
    rom.tick();
    
    // ROM should still work normally and data should be unchanged
    assert_eq!(rom.read(0x0000).unwrap(), 0xFF);
    assert_eq!(rom.read(0x0001).unwrap(), 0xEE);
    assert_eq!(rom.read(0x0002).unwrap(), 0xDD);
}

#[test]
fn test_bus_device_interrupts() {
    let rom = Rom::new(RomSize::_4K, 0x0000);
    
    // ROM should never generate interrupts
    assert_eq!(rom.check_irq(), false);
    assert_eq!(rom.check_nmi(), false);
}

// Test RomSize enum
#[test]
fn test_rom_size_values() {
    assert_eq!(RomSize::_2K as usize, 0x0800);
    assert_eq!(RomSize::_4K as usize, 0x1000);
    assert_eq!(RomSize::_8K as usize, 0x2000);
    assert_eq!(RomSize::_16K as usize, 0x4000);
    assert_eq!(RomSize::_32K as usize, 0x8000);
    assert_eq!(RomSize::_64K as usize, 0x10000);
}

#[test]
fn test_rom_size_default() {
    assert_eq!(RomSize::default(), RomSize::_32K);
}

#[test]
fn test_rom_size_traits() {
    let size1 = RomSize::_4K;
    let size2 = RomSize::_4K;
    let size3 = RomSize::_8K;
    
    // Test Clone
    let cloned = size1.clone();
    assert_eq!(cloned, size1);
    
    // Test Copy
    let copied = size1;
    assert_eq!(copied, size1);
    
    // Test PartialEq and Eq
    assert_eq!(size1, size2);
    assert_ne!(size1, size3);
    
    // Test Debug
    let debug_str = format!("{:?}", size1);
    assert!(debug_str.contains("_4K"));
}

// Integration tests combining multiple features
#[test]
fn test_rom_integration_import_export_bus() {
    let mut rom = Rom::new(RomSize::_4K, 0x2000);
    
    // Import data using import method
    let original_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    rom.import(&original_data, 100).unwrap();
    
    // Verify via export method
    let exported = rom.export(100, 8);
    assert_eq!(exported, original_data);
    
    // Verify via bus read
    for (i, &expected) in original_data.iter().enumerate() {
        let address = 0x2000 + 100 + i as u16;
        let result = rom.read(address);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
    
    // Verify that write attempts fail
    for i in 0..original_data.len() {
        let address = 0x2000 + 100 + i as u16;
        let result = rom.write(address, 0xFF);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BusError::ReadOnly(addr) if addr == address));
    }
    
    // Verify data is still unchanged after write attempts
    let exported_after_writes = rom.export(100, 8);
    assert_eq!(exported_after_writes, original_data);
}

#[test]
fn test_rom_persistence_across_operations() {
    let mut rom = Rom::new(RomSize::_2K, 0x0000);
    
    // Fill ROM with pattern
    let mut pattern_data = Vec::new();
    for i in 0..256 {
        pattern_data.push((i % 256) as u8);
    }
    rom.import(&pattern_data, 0).unwrap();
    
    // Verify pattern via bus reads
    for i in 0..256 {
        let result = rom.read(i);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (i % 256) as u8);
    }
    
    // Call tick multiple times and attempt writes (should not affect data)
    for i in 0..100 {
        rom.tick();
        let _ = rom.write((i % 256) as u16, 0xFF); // These should all fail
    }
    
    // Verify pattern is still intact
    for i in 0..256 {
        let result = rom.read(i);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (i % 256) as u8);
    }
}

#[test]
fn test_rom_boundary_conditions() {
    let mut rom = Rom::new(RomSize::_4K, 0x8000);
    
    // Import data at boundaries
    rom.import(&[0xAA], 0).unwrap(); // First byte
    rom.import(&[0xBB], 0x0FFF).unwrap(); // Last byte (0x1000 - 1)
    
    // Test first address
    assert_eq!(rom.read(0x8000).unwrap(), 0xAA);
    
    // Test last address (0x8000 + 0x1000 - 1 = 0x8FFF)
    assert_eq!(rom.read(0x8FFF).unwrap(), 0xBB);
    
    // Test one address before start (should fail)
    assert!(rom.read(0x7FFF).is_err());
    
    // Test one address after end (should fail)
    assert!(rom.read(0x9000).is_err());
    
    // Test write attempts at boundaries (should all fail)
    assert!(rom.write(0x8000, 0xCC).is_err());
    assert!(rom.write(0x8FFF, 0xDD).is_err());
    
    // Verify boundary data is unchanged
    assert_eq!(rom.read(0x8000).unwrap(), 0xAA);
    assert_eq!(rom.read(0x8FFF).unwrap(), 0xBB);
}

// Performance-related tests
#[test]
fn test_rom_large_operations() {
    let mut rom = Rom::new(RomSize::_64K, 0x0000);
    
    // Import large amount of data
    let large_data = vec![0xFF; 32768]; // 32K of data
    let result = rom.import(&large_data, 0);
    assert!(result.is_ok());
    
    // Export large amount of data
    let exported = rom.export(0, 32768);
    assert_eq!(exported.len(), 32768);
    assert_eq!(exported, large_data);
    
    // Verify specific locations via bus reads
    assert_eq!(rom.read(0x0000).unwrap(), 0xFF);
    assert_eq!(rom.read(0x7FFF).unwrap(), 0xFF);
    assert_eq!(rom.read(0x8000).unwrap(), 0x00); // Beyond imported data
}

#[test]
fn test_rom_typical_bootloader_scenario() {
    let mut rom = Rom::new(RomSize::_32K, 0x8000);
    
    // Simulate a typical 6502 bootloader ROM setup
    // Reset vector at 0xFFFC-0xFFFF (last 4 bytes of ROM)
    let reset_vector = vec![0x00, 0x80, 0x00, 0x80]; // Points to 0x8000
    rom.import(&reset_vector, 0x7FFC).unwrap(); // Offset 0x7FFC in 32K ROM
    
    // Some bootloader code at start
    let bootloader_code = vec![0xA9, 0x00, 0x8D, 0x00, 0x02]; // LDA #$00, STA $0200
    rom.import(&bootloader_code, 0).unwrap();
    
    // Verify reset vector via bus reads
    assert_eq!(rom.read(0xFFFC).unwrap(), 0x00);
    assert_eq!(rom.read(0xFFFD).unwrap(), 0x80);
    assert_eq!(rom.read(0xFFFE).unwrap(), 0x00);
    assert_eq!(rom.read(0xFFFF).unwrap(), 0x80);
    
    // Verify bootloader code
    assert_eq!(rom.read(0x8000).unwrap(), 0xA9);
    assert_eq!(rom.read(0x8001).unwrap(), 0x00);
    assert_eq!(rom.read(0x8002).unwrap(), 0x8D);
    
    // Verify writes are prevented (ROM is read-only)
    assert!(rom.write(0x8000, 0xFF).is_err());
    assert!(rom.write(0xFFFC, 0xFF).is_err());
}

#[test]
fn test_debug_trait() {
    let rom = Rom::new(RomSize::_4K, 0x8000);
    let debug_string = format!("{:?}", rom);
    
    // Should contain key information
    assert!(debug_string.contains("Rom"));
    // The exact format may vary, but it should be printable
    assert!(!debug_string.is_empty());
}

// Test edge cases and error conditions
#[test]
fn test_rom_zero_offset_import_export() {
    let mut rom = Rom::new(RomSize::_4K, 0x0000);
    let data = vec![0x01, 0x02, 0x03, 0x04];
    
    // Import at offset 0
    rom.import(&data, 0).unwrap();
    
    // Export from offset 0
    let exported = rom.export(0, 4);
    assert_eq!(exported, data);
    
    // Verify via bus reads
    for (i, &expected) in data.iter().enumerate() {
        assert_eq!(rom.read(i as u16).unwrap(), expected);
    }
}

#[test]
fn test_rom_address_wrapping_behavior() {
    let mut rom = Rom::new(RomSize::_4K, 0xF000);
    
    // Import data near the high end of address space
    let data = vec![0xAA, 0xBB];
    rom.import(&data, 0).unwrap();
    
    // Test address wrapping behavior at boundaries
    assert_eq!(rom.read(0xF000).unwrap(), 0xAA);
    assert_eq!(rom.read(0xF001).unwrap(), 0xBB);
    
    // Address just before start should fail
    assert!(rom.read(0xEFFF).is_err());
}
