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

/// A trait for CHIP-8 operations.
pub(crate) trait Operation {
    /// Executes the operation.
    fn exec(&self, machine: &mut Machine) -> OperationResult;
}

/// Implements the 00E0 (CLS) operation. Clear the display.
pub(crate) struct Op00e0;

impl Op00e0 {
    // Creates a new Op00e0.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00e0 {
    /// Execute the operation 00E0 (CLS).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00e0");

        machine.vram = [false; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }
}

/// Implements the 00EE (RET) operation. Return from a subroutine.
pub(crate) struct Op00ee;

impl Op00ee {
    // Creates a new Op00ee.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00ee {
    /// Execute the operation 00EE (RET).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00ee");

        machine.pc = machine.stack[machine.sp as usize] as usize;
        machine.sp -= 1;

        OperationResult::Next
    }
}

/// Implements the 1nnn (JP addr) operation. Jump to location `nnn`.
pub(crate) struct Op1nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op1nnn {
    // Creates a new Op1nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op1nnn {
    /// Execute the operation 1nnn (JP addr).
    fn exec(&self, _: &mut Machine) -> OperationResult {
        debug!("op_1nnn, nnn={:#06x?}", self.nnn);

        OperationResult::JumpTo(self.nnn as usize)
    }
}

/// Implements the 2nnn (CALL addr) operation. Call subroutine at `nnn`.
pub(crate) struct Op2nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op2nnn {
    // Creates a new Op2nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op2nnn {
    /// Execute the operation 2nnn (CALL addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_2nnn, nnn={:#06x?}", self.nnn);

        machine.sp += 1;
        machine.stack[machine.sp as usize] = machine.pc as u16;

        OperationResult::JumpTo(self.nnn as usize)
    }
}

/// Implements the 3xkk (SE Vx, byte) operation. Skip next instruction if `Vx = kk`.
pub(crate) struct Op3xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op3xkk {
    // Creates a new Op3xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op3xkk {
    /// Execute the operation 3xkk (SE Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_3xkk, x={}, kk={}", self.x, self.kk);

        if machine.v[self.x as usize] == self.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 4xkk (SNE Vx, byte) operation. Skip next instruction if `Vx != kk`.
pub(crate) struct Op4xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op4xkk {
    // Creates a new Op4xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op4xkk {
    /// Execute the operation 4xkk (SNE Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_4xkk, x={}, kk={}", self.x, self.kk);

        if machine.v[self.x as usize] != self.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 5xy0 (SE Vx, Vy) operation. Skip next instruction if `Vx = Vy`.
pub(crate) struct Op5xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op5xy0 {
    // Creates a new Op5xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op5xy0 {
    /// Execute the operation 5xy0 (SE Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_5xy0, x={}, y={}", self.x, self.y);

        if machine.v[self.x as usize] == machine.v[self.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the 6xkk (LD Vx, byte) operation. Set `Vx = kk`.
pub(crate) struct Op6xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op6xkk {
    // Creates a new Op6xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op6xkk {
    /// Execute the operation 6xkk (LD Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_6xkk, x={}, kk={}", self.x, self.kk);

        machine.v[self.x as usize] = self.kk;

        OperationResult::Next
    }
}

/// Implements the 7xkk (ADD Vx, byte) operation. Set `Vx = Vx + kk`.
pub(crate) struct Op7xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op7xkk {
    // Creates a new Op7xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op7xkk {
    /// Execute the operation 7xkk (ADD Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_7xkk, x={}, kk={}", self.x, self.kk);

        let ix = self.x as usize;
        machine.v[ix] = machine.v[ix].wrapping_add(self.kk);

        OperationResult::Next
    }
}

/// Implements the 8xy0 (LD Vx, Vy) operation. Set `Vx = Vy`.
pub(crate) struct Op8xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy0 {
    // Creates a new Op8xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy0 {
    /// Execute the operation 8xy0 (LD Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy0, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] = machine.v[self.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy1 (OR Vx, Vy) operation. Set `Vx = Vx OR Vy`.
pub(crate) struct Op8xy1 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy1 {
    // Creates a new Op8xy1.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy1 {
    /// Execute the operation 8xy1 (OR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy1, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] |= machine.v[self.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy2 (AND Vx, Vy) operation. Set `Vx = Vx AND Vy`.
pub(crate) struct Op8xy2 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy2 {
    // Creates a new Op8xy2.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy2 {
    /// Execute the operation 8xy2 (AND Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy2, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] &= machine.v[self.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy3 (XOR Vx, Vy) operation. Set `Vx = Vx XOR Vy`.
pub(crate) struct Op8xy3 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy3 {
    // Creates a new Op8xy3.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy3 {
    /// Execute the operation 8xy3 (XOR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy3, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] ^= machine.v[self.y as usize];

        OperationResult::Next
    }
}

/// Implements the 8xy4 (ADD Vx, Vy) operation. Set `Vx = Vx + Vy`, set `VF = carry`.
pub(crate) struct Op8xy4 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy4 {
    // Creates a new Op8xy4.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy4 {
    /// Execute the operation 8xy4 (ADD Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy4, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[ix].overflowing_add(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy5 (SUB Vx, Vy) operation. Set `Vx = Vx - Vy`, set `VF = NOT borrow`.
pub(crate) struct Op8xy5 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy5 {
    // Creates a new Op8xy5.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy5 {
    /// Execute the operation 8xy5 (SUB Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy5, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[ix].overflowing_sub(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy6 (SHR Vx {, Vy}) operation. Set `Vx = Vx SHR 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xy6 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy6 {
    // Creates a new Op8xy6.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy6 {
    /// Execute the operation 8xy6 (SHR Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy6, x={}", self.x);

        let ix = self.x as usize;
        let result = machine.v[ix].overflowing_shr(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xy7 (SUBN Vx, Vy) operation. Set `Vx = Vy - Vx`, set `VF = NOT borrow`.
pub(crate) struct Op8xy7 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy7 {
    // Creates a new Op8xy7.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy7 {
    /// Execute the operation 1nnn 8xy7 (SUBN Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy7, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[iy].overflowing_sub(machine.v[ix]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 8xyE (SHL Vx {, Vy}) operation. Set `Vx = Vx SHL 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xye {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xye {
    // Creates a new Op8xye.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xye {
    /// Execute the operation 8xyE (SHL Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xye, x={}", self.x);

        let ix = self.x as usize;

        let result = machine.v[ix].overflowing_shl(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}

/// Implements the 9xy0 (SNE Vx, Vy) operation. Skip next instruction if `Vx != Vy`.
pub(crate) struct Op9xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op9xy0 {
    // Creates a new Op9xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op9xy0 {
    /// Execute the operation 9xy0 (SNE Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_9xy0, x={}, y={}", self.x, self.y);

        if machine.v[self.x as usize] != machine.v[self.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the Annn (LD I, addr) operation. Set `I = nnn`.
pub(crate) struct Opannn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opannn {
    // Creates a new Opannn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opannn {
    /// Execute the operation Annn (LD I, addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_annn, nnn={}", self.nnn);

        machine.i = self.nnn as usize;

        OperationResult::Next
    }
}

/// Implements the Bnnn (JP V0, addr) operation. Jump to location `nnn + V0`.
pub(crate) struct Opbnnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opbnnn {
    // Creates a new Opbnnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opbnnn {
    /// Execute the operation Bnnn (JP V0, addr) .
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_bnnn, nnn={}", self.nnn);

        OperationResult::JumpTo((self.nnn + machine.v[ZERO] as u16) as usize)
    }
}

/// Implements the Cxkk (RND Vx, byte) operation. Set `Vx = random byte AND kk`.
pub(crate) struct Opcxkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Opcxkk {
    // Creates a new Opcxkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Opcxkk {
    /// Execute the operation Cxkk (RND Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_cxkk, x={}, kk={}", self.x, self.kk);

        let value = rand::random::<u8>();
        machine.v[self.x as usize] = value & self.kk;

        OperationResult::Next
    }
}

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

/// Implements the Ex9E (SKP Vx) operation. Skip next instruction if key with the value of `Vx` is pressed.
pub(crate) struct Opex9e<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opex9e<'a> {
    // Creates a new Opex9e.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opex9e<'_> {
    /// Execute the operation Ex9E (SKP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_ex9e, x={}, keys={:?}", self.x, self.keys);

        if self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the ExA1 (SKNP Vx) operation. Skip next instruction if key with the value of `Vx` is not pressed.
pub(crate) struct Opexa1<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opexa1<'a> {
    // Creates a new Opexa1.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opexa1<'_> {
    /// Execute the operation ExA1 (SKNP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_exa1, x={}, keys={:?}", self.x, self.keys);

        if !self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

/// Implements the Fx07 (LD Vx, DT) operation. Set `Vx = delay timer value`.
pub(crate) struct Opfx07 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx07 {
    // Creates a new Opexa1.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx07 {
    /// Execute the operation Fx07 (LD Vx, DT).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx07, x={}, dt={}", self.x, machine.dt);

        machine.v[self.x as usize] = machine.dt;

        OperationResult::Next
    }
}

/// Implements the Fx0A (LD Vx, K) operation. Wait for a key press, store the value of the key in `Vx`.
pub(crate) struct Opfx0a<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opfx0a<'a> {
    // Creates a new Opfx0a.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opfx0a<'_> {
    /// Execute the operation Fx0A (LD Vx, K).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx0a, x={}, keys={:?}", self.x, self.keys);

        if let Some(pos) = self.keys.iter().position(|&v| v) {
            machine.v[self.x as usize] = pos as u8;
            return OperationResult::Next;
        }

        OperationResult::WaitInput
    }
}

/// Implements the Fx15 (LD DT, Vx) operation. Set `delay timer = Vx`.
pub(crate) struct Opfx15 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx15 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx15 {
    /// Execute the operation Fx15 (LD DT, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.dt = machine.v[self.x as usize];

        OperationResult::Next
    }
}

/// Implements the Fx18 (LD ST, Vx) operation. Set `sound timer = Vx`.
pub(crate) struct Opfx18 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx18 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx18 {
    /// Execute the operation Fx18 (LD ST, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.st = machine.v[self.x as usize];

        OperationResult::Next
    }
}

/// Implements the Fx1E (ADD I, Vx) operation. Set `I = I + Vx`.
pub(crate) struct Opfx1e {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx1e {
    // Creates a new Opfx1e.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx1e {
    /// Execute the operation Fx1E (ADD I, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx1e, x={}", self.x);

        machine.i += machine.v[self.x as usize] as usize;

        OperationResult::Next
    }
}

/// Implements the Fx29 (LD F, Vx) operation. Set `I = location of sprite for digit Vx`.
pub(crate) struct Opfx29 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx29 {
    // Creates a new Opfx29.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx29 {
    /// Execute the operation Fx29 (LD F, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx29, x={}", self.x);

        machine.i = machine.v[self.x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }
}

/// Implements the Fx33 (LD B, Vx) operation. Store BCD representation of `Vx` in memory locations `I`, `I+1`, and `I+2`.
pub(crate) struct Opfx33 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx33 {
    // Creates a new Opfx33.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx33 {
    /// Execute the operation Fx33 (LD B, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx33, x={}", self.x);

        let vx = machine.v[self.x as usize];

        machine.ram[machine.i] = vx / 100 % 10;
        machine.ram[machine.i + 1] = vx / 10 % 10;
        machine.ram[machine.i + 2] = vx % 10;

        OperationResult::Next
    }
}

/// Implements the Fx55 (LD [I], Vx) operation. Store registers `V0` through `Vx` in memory starting at location `I`.
pub(crate) struct Opfx55 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx55 {
    // Creates a new Opfx55.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx55 {
    /// Execute the operation Fx55 (LD [I], Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx55, x={}", self.x);

        (0..=self.x).for_each(|n| machine.ram[machine.i + n as usize] = machine.v[n as usize]);

        OperationResult::Next
    }
}

/// Implements the Fx65 (LD Vx, [I]) operation. Read registers `V0` through `Vx` from memory starting at location `I`.
pub(crate) struct Opfx65 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx65 {
    // Creates a new Opfx65.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx65 {
    /// Execute the operation Fx65 (LD Vx, [I]).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx65, x={}", self.x);

        (0..=self.x).for_each(|n| machine.v[n as usize] = machine.ram[machine.i + n as usize]);

        OperationResult::Next
    }
}

pub(crate) struct OpInvalid;

impl OpInvalid {
    // Creates a new OpInvalid.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for OpInvalid {
    /// This operation execution wull be skiped because the operation is invalid.
    fn exec(&self, _: &mut Machine) -> OperationResult {
        debug!("op_invalid");
        OperationResult::Next
    }
}
