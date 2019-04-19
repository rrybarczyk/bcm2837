const CORE_CLK_HZ: u64 = 250_000_000;

/// From Section 1.2.2 ARM virtual addresses
/// Physical addresses range from 0x3F000000 to 0x3FFFFFFF for peripherals. The bus address for
/// peripherals are set up to map on to the peripheral bus address ranges starting at 0x7E000000.
/// Thus a peripheral advertised here at bus address 0x7Ennnnnn is available at physical address
/// 0x3Fnnnnnn.
pub const PERIPHERAL_BASE: usize = 0x3F000000;
