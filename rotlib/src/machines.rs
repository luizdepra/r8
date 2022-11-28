//! CHIP-8's machine representation.

use crate::{
    constants::{
        FONT, FONT_INITIAL_ADDRESS, FONT_MEMORY_SIZE, GENERAL_REGISTER_NUMBER, INITIAL_PC_VALUE,
        MEMORY_SIZE, STACK_SIZE, VRAM_HEIGHT, VRAM_WIDTH,
    },
    types::{GeneralRegisterBank, Ram, Stack, Vram},
};

/// A machine step/cycle result. [`redraw`] tells if the screen should be redraw, and [`beep`] if a beep sound should play.
#[derive(Debug)]
pub struct StepResult {
    pub redraw: bool,
    pub beep: bool,
}

/// Represents the CHIP-8 machine.
#[derive(Debug)]
pub struct Machine {
    ram: Ram,
    vram: Vram,
    stack: Stack,
    v: GeneralRegisterBank,
    i: usize,
    pc: usize,
    sp: usize,
    dt: u8,
    st: u8,
}

impl Default for Machine {
    /// Creates a [`Machine`] with the default values.
    fn default() -> Self {
        let mut ram = [0; MEMORY_SIZE];
        ram[FONT_INITIAL_ADDRESS..FONT_MEMORY_SIZE].copy_from_slice(&FONT);

        Self {
            ram,
            vram: [false; VRAM_WIDTH * VRAM_HEIGHT],
            stack: [0; STACK_SIZE],
            v: [0; GENERAL_REGISTER_NUMBER],
            i: 0,
            pc: INITIAL_PC_VALUE,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }
}

/// Returns the computed VRAM index using the provided `x` and `y` screen values.
fn vram_index(x: usize, y: usize) -> Option<usize> {
    if x >= VRAM_WIDTH || y >= VRAM_HEIGHT {
        return None;
    }

    Some(y * VRAM_WIDTH + x)
}
