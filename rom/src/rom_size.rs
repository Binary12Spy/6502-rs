//! Defines the size of the ROM in bytes.

/// Rom size in bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomSize {
    /// 2KB
    _2K = 0x0800,
    /// 4KB
    _4K = 0x1000,
    /// 8KB
    _8K = 0x2000,
    /// 16KB
    _16K = 0x4000,
    /// 32KB
    _32K = 0x8000,
    /// 64KB
    _64K = 0x10000,
}

impl Default for RomSize {
    /// Default ROM size is 32KB
    fn default() -> Self {
        RomSize::_32K
    }
}
