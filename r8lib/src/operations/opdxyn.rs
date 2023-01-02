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
                // Gets the `ix`-significative bit of `data`. We store 8 pixels inside a `u8` value.
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

        // Loop back if `x` and `y` values if they are greater than the VRAM sizes.
        let sx = (machine.v[self.x as usize] as usize) % VRAM_WIDTH;
        let sy = (machine.v[self.y as usize] as usize) % VRAM_HEIGHT;

        machine.v[CARRY] = 0;

        self.draw_sprite(machine, sx, sy, self.n);

        OperationResult::NextAndRedraw
    }
}

#[cfg(test)]
mod test_opdxyn {
    use super::*;

    struct TestCase {
        x: u8,
        y: u8,
        expected_positions: Vec<usize>,
    }

    #[test]
    fn test_opdxyn_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;
        let n = 0x5;

        machine.i = 0xFF0;
        // The sprite.
        machine.ram[0xFF0] = 0b11111111;
        machine.ram[0xFF1] = 0b10000001;
        machine.ram[0xFF2] = 0b10011001;
        machine.ram[0xFF3] = 0b10000001;
        machine.ram[0xFF4] = 0b11111111;

        let cases = [
            // Normal draw
            TestCase {
                x: 1,
                y: 1,
                expected_positions: vec![
                    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x81, 0x88, 0xC1, 0xC4, 0xC5, 0xC8, 0x101, 0x108,
                    0x141, 0x142, 0x143, 0x144, 0x145, 0x146, 0x147, 0x148,
                ],
            },
            // Normal draw, wrapping x and y
            TestCase {
                x: 65,
                y: 33,
                expected_positions: vec![
                    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x81, 0x88, 0xC1, 0xC4, 0xC5, 0xC8, 0x101, 0x108,
                    0x141, 0x142, 0x143, 0x144, 0x145, 0x146, 0x147, 0x148,
                ],
            },
            // Clipping draw
            TestCase {
                x: 60,
                y: 28,
                expected_positions: vec![0x73C, 0x73D, 0x73E, 0x73F, 0x77C, 0x7BC, 0x7BF, 0x7FC],
            },
        ];

        for (i, case) in cases.into_iter().enumerate() {
            machine.v[x as usize] = case.x;
            machine.v[y as usize] = case.y;

            machine.vram.iter_mut().for_each(|v| *v = false);

            let op = Opdxyn::new(x, y, n);
            let result = op.exec(&mut machine);

            assert_eq!(
                result,
                OperationResult::NextAndRedraw,
                "should return NextAndRedraw in test case {}",
                i
            );
            assert_eq!(
                machine.v[CARRY], 0x0,
                "machine v[0xF] should be 0 because there was no collision in test case {}",
                i
            );

            for pos in 0..VRAM_WIDTH * VRAM_HEIGHT {
                let value = case.expected_positions.contains(&pos);
                assert_eq!(
                    machine.vram[pos], value,
                    "machine vram at {:#02x?} index should be {} in test case {}",
                    pos, !value, i
                );
            }
        }
    }

    #[test]
    fn test_opdxyn_exec_with_collisions() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;
        let n = 0x5;

        machine.v[x as usize] = 0x1;
        machine.v[y as usize] = 0x1;
        machine.i = 0xFF0;
        // The sprite.
        machine.ram[0xFF0] = 0b11111111;
        machine.ram[0xFF1] = 0b10000001;
        machine.ram[0xFF2] = 0b10011001;
        machine.ram[0xFF3] = 0b10000001;
        machine.ram[0xFF4] = 0b11111111;
        // The vram has initial values.
        machine.vram[0x48] = true;
        machine.vram[0x141] = true;

        let op = Opdxyn::new(x, y, n);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::NextAndRedraw, "should return NextAndRedraw");
        assert_eq!(
            machine.v[CARRY], 1,
            "machine v[0xF] should be 0 because there was no collision"
        );

        let expected_positions = [
            0x41usize, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x81, 0x88, 0xC1, 0xC4, 0xC5, 0xC8, 0x101, 0x108, 0x142,
            0x143, 0x144, 0x145, 0x146, 0x147, 0x148,
        ];
        for pos in 0..VRAM_WIDTH * VRAM_HEIGHT {
            let value = expected_positions.contains(&pos);
            assert_eq!(
                machine.vram[pos], value,
                "machine vram at {:#02x?} index should be {}",
                pos, !value
            );
        }
    }
}
