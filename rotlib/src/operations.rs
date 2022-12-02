//! The implementation of all CHIP-8 operations.

use crate::{Keys, Machine, CARRY, FONT_CHAR_SIZE, SPRITE_WIDTH, VRAM_HEIGHT, VRAM_WIDTH, ZERO};
use log::debug;

#[derive(Debug)]
pub(crate) enum OperationResult {
    Next,
    NextAndRedraw,
    SkipNext,
    JumpTo(usize),
    WaitInput,
}

/// Represets all possible operation parameters.
#[derive(Debug)]
pub(crate) struct OperationParams<'a> {
    pub(crate) nnn: u16,
    pub(crate) kk: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) n: u8,
    pub(crate) keys: &'a Keys,
}

impl<'a> OperationParams<'a> {
    pub fn new(nnn: u16, kk: u8, x: u8, y: u8, n: u8, keys: &'a Keys) -> OperationParams<'a> {
        Self {
            nnn,
            kk,
            x,
            y,
            n,
            keys,
        }
    }
}

/// A trait for CHIP-8 operations.
pub(crate) trait Operation {
    /// Executes the operation.
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult;
}

/// Implements the 00E0 (CLS) operation. Clear the display.
pub(crate) struct Op00e0;

impl Operation for Op00e0 {
    /// Execute the operation 00E0 (CLS).
    fn exec(&self, machine: &mut Machine, _: OperationParams) -> OperationResult {
        debug!("op_00e0");

        machine.vram = [false; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }
}

/// Implements the 00EE (RET) operation. Return from a subroutine.
pub(crate) struct Op00ee;

impl Operation for Op00ee {
    /// Execute the operation 00EE (RET).
    fn exec(&self, machine: &mut Machine, _: OperationParams) -> OperationResult {
        debug!("op_00ee");

        machine.pc = machine.stack[machine.sp as usize] as usize;
        machine.sp -= 1;

        OperationResult::Next
    }
}

/// Implements the 1nnn (JP addr) operation. Jump to location `nnn`.
pub(crate) struct Op1nnn;

impl Operation for Op1nnn {
    /// Execute the operation 1nnn (JP addr).
    fn exec(&self, _: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_1nnn, nnn={:#06x?}", params.nnn);

        OperationResult::JumpTo(params.nnn as usize)
    }
}

/// Implements the 2nnn (CALL addr) operation. Call subroutine at `nnn`.
pub(crate) struct Op2nnn;

impl Operation for Op2nnn {
    /// Execute the operation 2nnn (CALL addr).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_2nnn, nnn={:#06x?}", params.nnn);

        machine.sp += 1;
        machine.stack[machine.sp as usize] = machine.pc as u16;

        OperationResult::JumpTo(params.nnn as usize)
    }
}

/// Implements the 3xkk (SE Vx, byte) operation. Skip next instruction if `Vx = kk`.
pub(crate) struct Op3xkk;

impl Operation for Op3xkk {
    /// Execute the operation 3xkk (SE Vx, byte).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_3xkk, x={}, kk={}", params.x, params.kk);

        if machine.v[params.x as usize] == params.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 4xkk (SNE Vx, byte) operation. Skip next instruction if `Vx != kk`.
pub(crate) struct Op4xkk;

impl Operation for Op4xkk {
    /// Execute the operation 4xkk (SNE Vx, byte).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_4xkk, x={}, kk={}", params.x, params.kk);

        if machine.v[params.x as usize] != params.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 5xy0 (SE Vx, Vy) operation. Skip next instruction if `Vx = Vy`.
pub(crate) struct Op5xy0;

impl Operation for Op5xy0 {
    /// Execute the operation 5xy0 (SE Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_5xy0, x={}, y={}", params.x, params.y);

        if machine.v[params.x as usize] == machine.v[params.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 6xkk (LD Vx, byte) operation. Set `Vx = kk`.
pub(crate) struct Op6xkk;

impl Operation for Op6xkk {
    /// Execute the operation 6xkk (LD Vx, byte).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_6xkk, x={}, kk={}", params.x, params.kk);

        machine.v[params.x as usize] = params.kk;

        OperationResult::Next
    }
}

/// Implements the 7xkk (ADD Vx, byte) operation. Set `Vx = Vx + kk`.
pub(crate) struct Op7xkk;

impl Operation for Op7xkk {
    /// Execute the operation 7xkk (ADD Vx, byte).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_7xkk, x={}, kk={}", params.x, params.kk);

        let ix = params.x as usize;
        machine.v[ix] = machine.v[ix].wrapping_add(params.kk);

        OperationResult::Next
    }
}

/// Implements the 8xy0 (LD Vx, Vy) operation. Set `Vx = Vy`.
pub(crate) struct Op8xy0;

impl Operation for Op8xy0 {
    /// Execute the operation 8xy0 (LD Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy0, x={}, y={}", params.x, params.y);

        machine.v[params.x as usize] = machine.v[params.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy1 (OR Vx, Vy) operation. Set `Vx = Vx OR Vy`.
pub(crate) struct Op8xy1;

impl Operation for Op8xy1 {
    /// Execute the operation 8xy1 (OR Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy1, x={}, y={}", params.x, params.y);

        machine.v[params.x as usize] |= machine.v[params.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy2 (AND Vx, Vy) operation. Set `Vx = Vx AND Vy`.
pub(crate) struct Op8xy2;

impl Operation for Op8xy2 {
    /// Execute the operation 8xy2 (AND Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy2, x={}, y={}", params.x, params.y);

        machine.v[params.x as usize] &= machine.v[params.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy3 (XOR Vx, Vy) operation. Set `Vx = Vx XOR Vy`.
pub(crate) struct Op8xy3;

impl Operation for Op8xy3 {
    /// Execute the operation 8xy3 (XOR Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy3, x={}, y={}", params.x, params.y);

        machine.v[params.x as usize] ^= machine.v[params.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy4 (ADD Vx, Vy) operation. Set `Vx = Vx + Vy`, set `VF = carry`.
pub(crate) struct Op8xy4;

impl Operation for Op8xy4 {
    /// Execute the operation 8xy4 (ADD Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy4, x={}, y={}", params.x, params.y);

        let ix = params.x as usize;
        let iy = params.y as usize;
        let result = machine.v[ix].overflowing_add(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy5 (SUB Vx, Vy) operation. Set `Vx = Vx - Vy`, set `VF = NOT borrow`.
pub(crate) struct Op8xy5;

impl Operation for Op8xy5 {
    /// Execute the operation 8xy5 (SUB Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy5, x={}, y={}", params.x, params.y);

        let ix = params.x as usize;
        let iy = params.y as usize;
        let result = machine.v[ix].overflowing_sub(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy6 (SHR Vx {, Vy}) operation. Set `Vx = Vx SHR 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xy6;

impl Operation for Op8xy6 {
    /// Execute the operation 8xy6 (SHR Vx {, Vy}).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy6, x={}", params.x);

        let ix = params.x as usize;
        let result = machine.v[ix].overflowing_shr(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy7 (SUBN Vx, Vy) operation. Set `Vx = Vy - Vx`, set `VF = NOT borrow`.
pub(crate) struct Op8xy7;

impl Operation for Op8xy7 {
    /// Execute the operation 1nnn 8xy7 (SUBN Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xy7, x={}, y={}", params.x, params.y);

        let ix = params.x as usize;
        let iy = params.y as usize;
        let result = machine.v[iy].overflowing_sub(machine.v[ix]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xyE (SHL Vx {, Vy}) operation. Set `Vx = Vx SHL 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xye;

impl Operation for Op8xye {
    /// Execute the operation 8xyE (SHL Vx {, Vy}).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_8xye, x={}", params.x);

        let ix = params.x as usize;

        let result = machine.v[ix].overflowing_shl(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 9xy0 (SNE Vx, Vy) operation. Skip next instruction if `Vx != Vy`.
pub(crate) struct Op9xy0;

impl Operation for Op9xy0 {
    /// Execute the operation 9xy0 (SNE Vx, Vy).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_9xy0, x={}, y={}", params.x, params.y);

        if machine.v[params.x as usize] != machine.v[params.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the Annn (LD I, addr) operation. Set `I = nnn`.
pub(crate) struct Opannn;

impl Operation for Opannn {
    /// Execute the operation Annn (LD I, addr).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_annn, nnn={}", params.nnn);

        machine.i = params.nnn as usize;

        OperationResult::Next
    }
}

/// Implements the Bnnn (JP V0, addr) operation. Jump to location `nnn + V0`.
pub(crate) struct Opbnnn;

impl Operation for Opbnnn {
    /// Execute the operation Bnnn (JP V0, addr) .
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_bnnn, nnn={}", params.nnn);

        OperationResult::JumpTo((params.nnn + machine.v[ZERO] as u16) as usize)
    }
}

/// Implements the Cxkk (RND Vx, byte) operation. Set `Vx = random byte AND kk`.
pub(crate) struct Opcxkk;

impl Operation for Opcxkk {
    /// Execute the operation Cxkk (RND Vx, byte).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_cxkk, x={}, kk={}", params.x, params.kk);

        let value = rand::random::<u8>();
        machine.v[params.x as usize] = value & params.kk;

        OperationResult::Next
    }
}

/// Implements the Dxyn (DRW Vx, Vy, nibble) operation. Display a n-byte sprite starting at memory location `I` at `(Vx, Vy)`, set `VF = collision`.
pub(crate) struct Opdxyn;

impl Opdxyn {
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
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_dxyn, x={}, y={}, n={}", params.x, params.y, params.n);

        let sx = (machine.v[params.x as usize] as usize) % VRAM_WIDTH;
        let sy = (machine.v[params.y as usize] as usize) % VRAM_HEIGHT;

        machine.v[CARRY] = 0;

        self.draw_sprite(machine, sx, sy, params.n);

        OperationResult::NextAndRedraw
    }
}

/// Implements the Ex9E (SKP Vx) operation. Skip next instruction if key with the value of `Vx` is pressed.
pub(crate) struct Opex9e;

impl Operation for Opex9e {
    /// Execute the operation Ex9E (SKP Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_ex9e, x={}, keys={:?}", params.x, params.keys);

        if params.keys[machine.v[params.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the ExA1 (SKNP Vx) operation. Skip next instruction if key with the value of `Vx` is not pressed.
pub(crate) struct Opexa1;

impl Operation for Opexa1 {
    /// Execute the operation ExA1 (SKNP Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_exa1, x={}, keys={:?}", params.x, params.keys);

        if !params.keys[machine.v[params.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the Fx07 (LD Vx, DT) operation. Set `Vx = delay timer value`.
pub(crate) struct Opfx07;

impl Operation for Opfx07 {
    /// Execute the operation Fx07 (LD Vx, DT).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx07, x={}, dt={}", params.x, machine.dt);

        machine.v[params.x as usize] = machine.dt;

        OperationResult::Next
    }
}

/// Implements the Fx0A (LD Vx, K) operation. Wait for a key press, store the value of the key in `Vx`.
pub(crate) struct Opfx0a;

impl Operation for Opfx0a {
    /// Execute the operation Fx0A (LD Vx, K).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx0a, x={}, keys={:?}", params.x, params.keys);

        if let Some(pos) = params.keys.iter().position(|&v| v) {
            machine.v[params.x as usize] = pos as u8;
            return OperationResult::Next;
        }

        OperationResult::WaitInput
    }
}

/// Implements the Fx15 (LD DT, Vx) operation. Set `delay timer = Vx`.
pub(crate) struct Opfx15;

impl Operation for Opfx15 {
    /// Execute the operation Fx15 (LD DT, Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx15, x={}", params.x);

        machine.dt = machine.v[params.x as usize];

        OperationResult::Next
    }
}

/// Implements the Fx18 (LD ST, Vx) operation. Set `sound timer = Vx`.
pub(crate) struct Opfx18;

impl Operation for Opfx18 {
    /// Execute the operation Fx18 (LD ST, Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx15, x={}", params.x);

        machine.st = machine.v[params.x as usize];

        OperationResult::Next
    }
}

/// Implements the Fx1E (ADD I, Vx) operation. Set `I = I + Vx`.
pub(crate) struct Opfx1e;

impl Operation for Opfx1e {
    /// Execute the operation Fx1E (ADD I, Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx1e, x={}", params.x);

        machine.i += machine.v[params.x as usize] as usize;

        OperationResult::Next
    }
}

/// Implements the Fx29 (LD F, Vx) operation. Set `I = location of sprite for digit Vx`.
pub(crate) struct Opfx29;

impl Operation for Opfx29 {
    /// Execute the operation Fx29 (LD F, Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx29, x={}", params.x);

        machine.i = machine.v[params.x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }
}

/// Implements the Fx33 (LD B, Vx) operation. Store BCD representation of `Vx` in memory locations `I`, `I+1`, and `I+2`.
pub(crate) struct Opfx33;

impl Operation for Opfx33 {
    /// Execute the operation Fx33 (LD B, Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx33, x={}", params.x);

        let vx = machine.v[params.x as usize];

        machine.ram[machine.i] = vx / 100 % 10;
        machine.ram[machine.i + 1] = vx / 10 % 10;
        machine.ram[machine.i + 2] = vx % 10;

        OperationResult::Next
    }
}

/// Implements the Fx55 (LD [I], Vx) operation. Store registers `V0` through `Vx` in memory starting at location `I`.
pub(crate) struct Opfx55;

impl Operation for Opfx55 {
    /// Execute the operation Fx55 (LD [I], Vx).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx55, x={}", params.x);

        (0..=params.x).for_each(|n| machine.ram[machine.i + n as usize] = machine.v[n as usize]);

        OperationResult::Next
    }
}

/// Implements the Fx65 (LD Vx, [I]) operation. Read registers `V0` through `Vx` from memory starting at location `I`.
pub(crate) struct Opfx65;

impl Operation for Opfx65 {
    /// Execute the operation Fx65 (LD Vx, [I]).
    fn exec(&self, machine: &mut Machine, params: OperationParams) -> OperationResult {
        debug!("op_fx65, x={}", params.x);

        (0..=params.x).for_each(|n| machine.v[n as usize] = machine.ram[machine.i + n as usize]);

        OperationResult::Next
    }
}

pub(crate) struct OpInvalid;

impl Operation for OpInvalid {
    /// This operation execution wull be skiped because the operation is invalid.
    fn exec(&self, _: &mut Machine, _: OperationParams) -> OperationResult {
        debug!("op_invalid");
        OperationResult::Next
    }
}
