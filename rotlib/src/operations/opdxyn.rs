//! The implementation of the Dxyn (DRW Vx, Vy, nibble) operation.

use log::debug;

use crate::{Machine, CARRY, SPRITE_WIDTH, VRAM_HEIGHT, VRAM_WIDTH};

use super::{Operation, OperationResult};

/// Implements the Dxyn (DRW Vx, Vy, nibble) operation. Display a n-byte sprite starting at memory location `I` at `(Vx, Vy)`, set `VF = collision`.
pub(crate) struct Opdxyn {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
    /// The `n` operation parameter.
    n: u8,
}

impl Opdxyn {
    // Creates a new Opdxyn.
    pub(crate) fn new(x: u8, y: u8, n: u8) -> Self {
        Self { x, y, n }
    }

    /// Returns the computed VRAM index using the provided `x` and `y` screen values.
    fn vram_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= VRAM_WIDTH || y >= VRAM_HEIGHT {
            return None;
        }

        Some(y * VRAM_WIDTH + x)
    }

    /// Draws a sprite in the machine VRAM.
    fn draw_sprite(&self, machine: &mut Machine, x: usize, y: usize, n: u8) {
        debug!("draw_sprite, x={}, y={}, n={}", x, y, n);

        for iy in 0..(n as usize) {
            let data = machine.ram[machine.i + iy];
            for ix in 0..SPRITE_WIDTH {
                let value = data & (0x80 >> ix) > 0;

                self.draw_pixel(machine, value, x + ix, y + iy);
            }
        }
    }

    /// Draws a pixel in the machine VRAM.
    fn draw_pixel(&self, machine: &mut Machine, value: bool, x: usize, y: usize) {
        debug!("draw_pixel, x={}, y={}, value={}", x, y, value);

        if let Some(idx) = self.vram_index(x, y) {
            debug!("draw_pixel_ram_index, idx={}", idx);

            if value && machine.vram[idx] {
                machine.v[CARRY] = 1;
            }

            machine.vram[idx] ^= value;
        }
    }
}

impl Operation for Opdxyn {
    /// Execute the operation Dxyn (DRW Vx, Vy, nibble).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_dxyn, x={}, y={}, n={}", self.x, self.y, self.n);

        let sx = (machine.v[self.x as usize] as usize) % VRAM_WIDTH;
        let sy = (machine.v[self.y as usize] as usize) % VRAM_HEIGHT;

        machine.v[CARRY] = 0;

        self.draw_sprite(machine, sx, sy, self.n);

        OperationResult::NextAndRedraw
    }
}
