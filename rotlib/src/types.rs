//! Common interpreter types.

use crate::constants::{
    GENERAL_REGISTER_NUMBER, MEMORY_SIZE, NUM_KEYS, STACK_SIZE, VRAM_HEIGHT, VRAM_WIDTH,
};

/// An array of [`bool`]s that represents the CHIP-8's screen.
pub type Vram = [bool; VRAM_WIDTH * VRAM_HEIGHT];
/// An array of [`u8`]s that represents a CHIP-8's ROM.
pub type Rom = [u8];
/// An array of [`bool`]s hat represents the state of every CHIP-8's keyboard keys.
pub type Keys = [bool; NUM_KEYS];

/// An array of [`u8`]s that represents the CHIP-8's RAM.
pub(crate) type Ram = [u8; MEMORY_SIZE];
/// An array of [`u16`]s that represents the CHIP-8's stack.
pub(crate) type Stack = [u16; STACK_SIZE];
/// An array of [`u8`]s that represents the CHIP-8's general use registers.
pub(crate) type GeneralRegisterBank = [u8; GENERAL_REGISTER_NUMBER];
