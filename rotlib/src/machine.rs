//! CHIP-8's machine representation.

use log::debug;

use crate::keyboard::Keys;

// Sizes

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
pub(crate) const VRAM_WIDTH: usize = 64;
/// VRAM height.
pub(crate) const VRAM_HEIGHT: usize = 32;

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

/// An array of [`bool`]s that represents the CHIP-8's screen.
pub type Vram = [bool; VRAM_WIDTH * VRAM_HEIGHT];
/// An array of [`u8`]s that represents a CHIP-8's ROM.
pub type Rom = [u8];

/// An array of [`u8`]s that represents the CHIP-8's RAM.
pub(crate) type Ram = [u8; MEMORY_SIZE];
/// An array of [`u16`]s that represents the CHIP-8's stack.
pub(crate) type Stack = [u16; STACK_SIZE];
/// An array of [`u8`]s that represents the CHIP-8's general use registers.
pub(crate) type GeneralRegisterBank = [u8; GENERAL_REGISTER_NUMBER];

#[derive(Debug)]
enum OperationResult {
    Next,
    NextAndRedraw,
    SkipNext,
    JumpTo(usize),
    WaitInput,
}

/// Represents the CHIP-8 machine.
#[derive(Debug)]
pub struct Machine {
    /// The machine RAM, where the ROM, font and etc aer loaded.
    ram: Ram,
    /// The machine VRAM, used to represente the screen state.
    vram: Vram,
    /// The machine call stack.
    stack: Stack,
    /// The machine general register representation. These are the `vN` registers, where N is in range `[0, 8]`.
    v: GeneralRegisterBank,
    /// The machine `I` register.
    i: usize,
    /// The machine Program Counter.
    pc: usize,
    /// The machine Stack Pointer.
    sp: usize,
    /// The machine Delay Timer.
    dt: u8,
    /// The machine Sound Timer.
    st: u8,
    /// A flag to tell if the screen should be redrawn.
    draw: bool,
}

impl Machine {
    /// Returns a reference to the machine VRAM. This value should be used to draw the screen.
    pub fn vram_as_ref(&self) -> &Vram {
        &self.vram
    }

    /// Loads the provided CHIP-8's ROM into the machine RAM/memory.
    pub fn load_rom(&mut self, rom: &Rom) {
        debug!("load_rom, rom={:?}", rom);

        self.ram[ROM_INITIAL_ADDRESS..ROM_INITIAL_ADDRESS + rom.len()].copy_from_slice(rom);

        debug!("load_rom_finished, ram={:?}", self.ram);
    }

    /// Does a machine instruction step.
    pub fn step(&mut self, keys: &Keys) {
        debug!("step_pc, pc={:#06x?}", self.pc);

        let instr = (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;

        debug!("step_instruction, instr={:#06x?}", instr);

        self.draw = false;
        self.run_instruction(instr, keys);
    }

    /// Updates the timers state.
    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }

        debug!("update_timers, dt={}, st={}", self.st, self.st);
    }

    /// Tells if the screen should be draw.
    pub fn should_draw(&self) -> bool {
        debug!("should_draw, dt={}, st={}", self.st, self.st);

        self.draw
    }

    /// Tells if the beep should be active.
    pub fn should_beep(&self) -> bool {
        debug!("should_beep, beep={}", self.st > 0);

        self.st > 0
    }

    /// Runs the operation extracted from the machine RAM.
    fn run_instruction(&mut self, instr: u16, keys: &Keys) -> bool {
        debug!("run_instruction, instr={:#06x?}, keys={:?}", instr, keys);

        let nibbles = (
            ((instr & 0xF000) >> 12) as u8,
            ((instr & 0x0F00) >> 8) as u8,
            ((instr & 0x00F0) >> 4) as u8,
            (instr & 0x000F) as u8,
        );

        debug!("run_instruction_nibbles, nibbles={:?}", nibbles);

        let nnn = instr & 0x0FFF;
        let kk = (instr & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        let action = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xye(x, y),
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nnn),
            (0xC, _, _, _) => self.op_cxkk(x, kk),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(x, keys),
            (0xE, _, 0xA, 0x1) => self.op_exa1(x, keys),
            (0xF, _, 0x0, 0x7) => self.op_fx07(x),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(x, keys),
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.op_fx29(x),
            (0xF, _, 0x3, 0x3) => self.op_fx33(x),
            (0xF, _, 0x5, 0x5) => self.op_fx55(x),
            (0xF, _, 0x6, 0x5) => self.op_fx65(x),
            _ => OperationResult::Next,
        };

        debug!("run_instruction_result, result={:?}", action);

        match action {
            OperationResult::Next => {
                self.pc += 2;
                false
            }
            OperationResult::NextAndRedraw => {
                self.pc += 2;
                self.draw = true;
                true
            }
            OperationResult::SkipNext => {
                self.pc += 4;
                false
            }
            OperationResult::JumpTo(addr) => {
                self.pc = addr;
                false
            }
            OperationResult::WaitInput => false,
        }
    }

    /// Draws a sprite in the machine VRAM.
    fn draw_sprite(&mut self, x: usize, y: usize, n: u8) {
        debug!("draw_sprite, x={}, y={}, n={}", x, y, n);

        for iy in 0..(n as usize) {
            let data = self.ram[self.i + iy];
            for ix in 0..SPRITE_WIDTH {
                let value = data & (0x80 >> ix) > 0;

                self.draw_pixel(value, x + ix, y + iy);
            }
        }
    }

    /// Draws a pixel in the machine VRAM.
    fn draw_pixel(&mut self, value: bool, x: usize, y: usize) {
        debug!("draw_pixel, x={}, y={}, value={}", x, y, value);

        if let Some(idx) = vram_index(x, y) {
            debug!("draw_pixel_ram_index, idx={}", idx);

            if value && self.vram[idx] {
                self.v[CARRY] = 1;
            }

            self.vram[idx] ^= value;
        }
    }

    /// Implements the 00E0 (CLS) operation. Clear the display.
    fn op_00e0(&mut self) -> OperationResult {
        debug!("op_00e0");

        self.vram = [false; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }

    /// Implements the 00EE (RET) operation. Return from a subroutine.
    fn op_00ee(&mut self) -> OperationResult {
        debug!("op_00ee");

        self.pc = self.stack[self.sp as usize] as usize;
        self.sp -= 1;

        OperationResult::Next
    }

    /// Implements the 1nnn (JP addr) operation. Jump to location `nnn`.
    fn op_1nnn(&mut self, nnn: u16) -> OperationResult {
        debug!("op_1nnn, nnn={:#06x?}", nnn);

        OperationResult::JumpTo(nnn as usize)
    }

    /// Implements the 2nnn (CALL addr) operation. Call subroutine at `nnn`.
    fn op_2nnn(&mut self, nnn: u16) -> OperationResult {
        debug!("op_2nnn, nnn={:#06x?}", nnn);

        self.sp += 1;
        self.stack[self.sp as usize] = self.pc as u16;

        OperationResult::JumpTo(nnn as usize)
    }

    /// Implements the 3xkk (SE Vx, byte) operation. Skip next instruction if `Vx = kk`.
    fn op_3xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        debug!("op_3xkk, x={}, kk={}", x, kk);

        if self.v[x as usize] == kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the 4xkk (SNE Vx, byte) operation. Skip next instruction if `Vx != kk`.
    fn op_4xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        debug!("op_4xkk, x={}, kk={}", x, kk);

        if self.v[x as usize] != kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the 5xy0 (SE Vx, Vy) operation. Skip next instruction if `Vx = Vy`.
    fn op_5xy0(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_5xy0, x={}, y={}", x, y);

        if self.v[x as usize] == self.v[y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the 6xkk (LD Vx, byte) operation. Set `Vx = kk`.
    fn op_6xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        debug!("op_6xkk, x={}, kk={}", x, kk);

        self.v[x as usize] = kk;

        OperationResult::Next
    }

    /// Implements the 7xkk (ADD Vx, byte) operation. Set `Vx = Vx + kk`.
    fn op_7xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        debug!("op_7xkk, x={}, kk={}", x, kk);

        let ix = x as usize;
        self.v[ix] = self.v[ix].wrapping_add(kk);

        OperationResult::Next
    }

    /// Implements the 8xy0 (LD Vx, Vy) operation. Set `Vx = Vy`.
    fn op_8xy0(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy0, x={}, y={}", x, y);

        self.v[x as usize] = self.v[y as usize];

        OperationResult::Next
    }

    /// Implements the 8xy1 (OR Vx, Vy) operation. Set `Vx = Vx OR Vy`.
    fn op_8xy1(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy1, x={}, y={}", x, y);

        self.v[x as usize] |= self.v[y as usize];

        OperationResult::Next
    }

    /// Implements the 8xy2 (AND Vx, Vy) operation. Set `Vx = Vx AND Vy`.
    fn op_8xy2(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy2, x={}, y={}", x, y);

        self.v[x as usize] &= self.v[y as usize];

        OperationResult::Next
    }

    /// Implements the 8xy3 (XOR Vx, Vy) operation. Set `Vx = Vx XOR Vy`.
    fn op_8xy3(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy3, x={}, y={}", x, y);

        self.v[x as usize] ^= self.v[y as usize];

        OperationResult::Next
    }

    /// Implements the 8xy4 (ADD Vx, Vy) operation. Set `Vx = Vx + Vy`, set `VF = carry`.
    fn op_8xy4(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy4, x={}, y={}", x, y);

        let ix = x as usize;
        let iy = y as usize;
        let result = self.v[ix].overflowing_add(self.v[iy]);
        self.v[ix] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    /// Implements the 8xy5 (SUB Vx, Vy) operation. Set `Vx = Vx - Vy`, set `VF = NOT borrow`.
    fn op_8xy5(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy5, x={}, y={}", x, y);

        let ix = x as usize;
        let iy = y as usize;
        let result = self.v[ix].overflowing_sub(self.v[iy]);
        self.v[ix] = result.0;
        self.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }

    /// Implements the 8xy6 (SHR Vx {, Vy}) operation. Set `Vx = Vx SHR 1`.
    ///
    /// Ignoring Vy value following modern interpreters implementation.
    fn op_8xy6(&mut self, x: u8, _: u8) -> OperationResult {
        debug!("op_8xy6, x={}", x);

        let ix = x as usize;
        let result = self.v[ix].overflowing_shr(1);
        self.v[ix] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    /// Implements the 8xy7 (SUBN Vx, Vy) operation. Set `Vx = Vy - Vx`, set `VF = NOT borrow`.
    fn op_8xy7(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_8xy7, x={}, y={}", x, y);

        let ix = x as usize;
        let iy = y as usize;
        let result = self.v[iy].overflowing_sub(self.v[ix]);
        self.v[ix] = result.0;
        self.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }

    /// Implements the 8xyE (SHL Vx {, Vy}) operation. Set `Vx = Vx SHL 1`.
    ///
    /// Ignoring Vy value following modern interpreters implementation.
    fn op_8xye(&mut self, x: u8, _: u8) -> OperationResult {
        debug!("op_8xye, x={}", x);

        let ix = x as usize;

        let result = self.v[ix].overflowing_shl(1);
        self.v[ix] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    /// Implements the 9xy0 (SNE Vx, Vy) operation. Skip next instruction if `Vx != Vy`.
    fn op_9xy0(&mut self, x: u8, y: u8) -> OperationResult {
        debug!("op_9xy0, x={}, y={}", x, y);

        if self.v[x as usize] != self.v[y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the Annn (LD I, addr) operation. Set `I = nnn`.
    fn op_annn(&mut self, nnn: u16) -> OperationResult {
        debug!("op_annn, nnn={}", nnn);

        self.i = nnn as usize;

        OperationResult::Next
    }

    /// Implements the Bnnn (JP V0, addr) operation. Jump to location `nnn + V0`.
    fn op_bnnn(&mut self, nnn: u16) -> OperationResult {
        debug!("op_bnnn, nnn={}", nnn);

        OperationResult::JumpTo((nnn + self.v[0x0] as u16) as usize)
    }

    /// Implements the Cxkk (RND Vx, byte) operation. Set `Vx = random byte AND kk`.
    fn op_cxkk(&mut self, x: u8, kk: u8) -> OperationResult {
        debug!("op_cxkk, x={}, kk={}", x, kk);

        let value = rand::random::<u8>();
        self.v[x as usize] = value & kk;

        OperationResult::Next
    }

    /// Implements the Dxyn (DRW Vx, Vy, nibble) operation. Display a n-byte sprite starting at memory location `I` at `(Vx, Vy)`, set `VF = collision`.
    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) -> OperationResult {
        debug!("op_dxyn, x={}, y={}, n={}", x, y, n);

        let sx = (self.v[x as usize] as usize) % VRAM_WIDTH;
        let sy = (self.v[y as usize] as usize) % VRAM_HEIGHT;

        self.v[CARRY] = 0;

        self.draw_sprite(sx, sy, n);

        OperationResult::NextAndRedraw
    }

    /// Implements the Ex9E (SKP Vx) operation. Skip next instruction if key with the value of `Vx` is pressed.
    fn op_ex9e(&mut self, x: u8, keys: &Keys) -> OperationResult {
        debug!("op_ex9e, x={}, keys={:?}", x, keys);

        if keys[self.v[x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the ExA1 (SKNP Vx) operation. Skip next instruction if key with the value of `Vx` is not pressed.
    fn op_exa1(&mut self, x: u8, keys: &Keys) -> OperationResult {
        debug!("op_exa1, x={}, keys={:?}", x, keys);

        if !keys[self.v[x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    /// Implements the Fx07 (LD Vx, DT) operation. Set `Vx = delay timer value`.
    fn op_fx07(&mut self, x: u8) -> OperationResult {
        debug!("op_fx07, x={}, dt={}", x, self.dt);

        self.v[x as usize] = self.dt;

        OperationResult::Next
    }

    /// Implements the Fx0A (LD Vx, K) operation. Wait for a key press, store the value of the key in `Vx`.
    fn op_fx0a(&mut self, x: u8, keys: &Keys) -> OperationResult {
        debug!("op_fx0a, x={}, keys={:?}", x, keys);

        if let Some(pos) = keys.iter().position(|&v| v) {
            self.v[x as usize] = pos as u8;
            return OperationResult::Next;
        }

        OperationResult::WaitInput
    }

    /// Implements the Fx15 (LD DT, Vx) operation. Set `delay timer = Vx`.
    fn op_fx15(&mut self, x: u8) -> OperationResult {
        debug!("op_fx15, x={}", x);

        self.dt = self.v[x as usize];

        OperationResult::Next
    }

    /// Implements the Fx18 (LD ST, Vx) operation. Set `sound timer = Vx`.
    fn op_fx18(&mut self, x: u8) -> OperationResult {
        debug!("op_fx15, x={}", x);

        self.st = self.v[x as usize];

        OperationResult::Next
    }

    /// Implements the Fx1E (ADD I, Vx) operation. Set `I = I + Vx`.
    fn op_fx1e(&mut self, x: u8) -> OperationResult {
        debug!("op_fx1e, x={}", x);

        self.i += self.v[x as usize] as usize;

        OperationResult::Next
    }

    /// Implements the Fx29 (LD F, Vx) operation. Set `I = location of sprite for digit Vx`.
    fn op_fx29(&mut self, x: u8) -> OperationResult {
        debug!("op_fx29, x={}", x);

        self.i = self.v[x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }

    /// Implements the Fx33 (LD B, Vx) operation. Store BCD representation of `Vx` in memory locations `I`, `I+1`, and `I+2`.
    fn op_fx33(&mut self, x: u8) -> OperationResult {
        debug!("op_fx33, x={}", x);

        let vx = self.v[x as usize];

        self.ram[self.i] = vx / 100 % 10;
        self.ram[self.i + 1] = vx / 10 % 10;
        self.ram[self.i + 2] = vx % 10;

        OperationResult::Next
    }

    /// Implements the Fx55 (LD [I], Vx) operation. Store registers `V0` through `Vx` in memory starting at location `I`.
    fn op_fx55(&mut self, x: u8) -> OperationResult {
        debug!("op_fx55, x={}", x);

        (0..=x).for_each(|n| self.ram[self.i + n as usize] = self.v[n as usize]);

        OperationResult::Next
    }

    /// Implements the Fx65 (LD Vx, [I]) operation. Read registers `V0` through `Vx` from memory starting at location `I`.
    fn op_fx65(&mut self, x: u8) -> OperationResult {
        debug!("op_fx65, x={}", x);

        (0..=x).for_each(|n| self.v[n as usize] = self.ram[self.i + n as usize]);

        OperationResult::Next
    }
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
            draw: false,
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
