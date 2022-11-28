//! CHIP8 constant values

// Sizes

/// Number of Keys.
pub const NUM_KEYS: usize = 16;
/// Machine RAM size.
pub(crate) const MEMORY_SIZE: usize = 4096;
/// Machine Stack size.
pub(crate) const STACK_SIZE: usize = 16;
/// Font in memory size.
pub(crate) const FONT_MEMORY_SIZE: usize = 80;
/// Font character size.
pub(crate) const FONT_CHAR_SIZE: usize = 5;
/// Sprite width.
pub(crate) const SPRITE_WIDTH: usize = 8;
/// VRAM width.
pub const VRAM_WIDTH: usize = 64;
/// VRAM height.
pub const VRAM_HEIGHT: usize = 32;

// Registers

/// Number of general use registers.
pub(crate) const GENERAL_REGISTER_NUMBER: usize = 16;
/// Carry register index.
pub(crate) const CARRY: usize = 0xF;
/// Initial PC value.
pub(crate) const INITIAL_PC_VALUE: usize = 0x200;

// Memory Addresses and Values

/// Font initial address in memory.
pub(crate) const FONT_INITIAL_ADDRESS: usize = 0;
/// ROM initial address in memory.
pub(crate) const ROM_INITIAL_ADDRESS: usize = 512;
/// Font values.
pub(crate) const FONT: [u8; FONT_MEMORY_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
