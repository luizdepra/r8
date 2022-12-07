//! CHIP-8's machine representation.

use std::fmt;

use log::debug;
use rand::RngCore;

use crate::keyboard::Keys;
use crate::operations::*;

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
/// Zero register index.
pub(crate) const ZERO: usize = 0x0;
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

/// Represents the CHIP-8 machine.
pub struct Machine {
    /// The machine RAM, where the ROM, font and etc aer loaded.
    pub(crate) ram: Ram,
    /// The machine VRAM, used to represente the screen state.
    pub(crate) vram: Vram,
    /// The machine call stack.
    pub(crate) stack: Stack,
    /// The machine general register representation. These are the `vN` registers, where N is in range `[0, 8]`.
    pub(crate) v: GeneralRegisterBank,
    /// The machine `I` register.
    pub(crate) i: usize,
    /// The machine Program Counter.
    pub(crate) pc: usize,
    /// The machine Stack Pointer.
    pub(crate) sp: usize,
    /// The machine Delay Timer.
    pub(crate) dt: u8,
    /// The machine Sound Timer.
    pub(crate) st: u8,
    /// A flag to tell if the screen should be redrawn.
    pub(crate) draw: bool,
    /// A random number generator.
    pub(crate) rng: Box<dyn RngCore>,
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

        let op: Box<dyn Operation> = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Box::new(Op00e0::new()),
            (0x0, 0x0, 0xE, 0xE) => Box::new(Op00ee::new()),
            (0x1, _, _, _) => Box::new(Op1nnn::new(nnn)),
            (0x2, _, _, _) => Box::new(Op2nnn::new(nnn)),
            (0x3, _, _, _) => Box::new(Op3xkk::new(x, kk)),
            (0x4, _, _, _) => Box::new(Op4xkk::new(x, kk)),
            (0x5, _, _, 0x0) => Box::new(Op5xy0::new(x, y)),
            (0x6, _, _, _) => Box::new(Op6xkk::new(x, kk)),
            (0x7, _, _, _) => Box::new(Op7xkk::new(x, kk)),
            (0x8, _, _, 0x0) => Box::new(Op8xy0::new(x, y)),
            (0x8, _, _, 0x1) => Box::new(Op8xy1::new(x, y)),
            (0x8, _, _, 0x2) => Box::new(Op8xy2::new(x, y)),
            (0x8, _, _, 0x3) => Box::new(Op8xy3::new(x, y)),
            (0x8, _, _, 0x4) => Box::new(Op8xy4::new(x, y)),
            (0x8, _, _, 0x5) => Box::new(Op8xy5::new(x, y)),
            (0x8, _, _, 0x6) => Box::new(Op8xy6::new(x)),
            (0x8, _, _, 0x7) => Box::new(Op8xy7::new(x, y)),
            (0x8, _, _, 0xE) => Box::new(Op8xye::new(x)),
            (0x9, _, _, 0x0) => Box::new(Op9xy0::new(x, y)),
            (0xA, _, _, _) => Box::new(Opannn::new(nnn)),
            (0xB, _, _, _) => Box::new(Opbnnn::new(nnn)),
            (0xC, _, _, _) => Box::new(Opcxkk::new(x, kk)),
            (0xD, _, _, _) => Box::new(Opdxyn::new(x, y, n)),
            (0xE, _, 0x9, 0xE) => Box::new(Opex9e::new(x, keys)),
            (0xE, _, 0xA, 0x1) => Box::new(Opexa1::new(x, keys)),
            (0xF, _, 0x0, 0x7) => Box::new(Opfx07::new(x)),
            (0xF, _, 0x0, 0xA) => Box::new(Opfx0a::new(x, keys)),
            (0xF, _, 0x1, 0x5) => Box::new(Opfx15::new(x)),
            (0xF, _, 0x1, 0x8) => Box::new(Opfx18::new(x)),
            (0xF, _, 0x1, 0xE) => Box::new(Opfx1e::new(x)),
            (0xF, _, 0x2, 0x9) => Box::new(Opfx29::new(x)),
            (0xF, _, 0x3, 0x3) => Box::new(Opfx33::new(x)),
            (0xF, _, 0x5, 0x5) => Box::new(Opfx55::new(x)),
            (0xF, _, 0x6, 0x5) => Box::new(Opfx65::new(x)),
            _ => Box::new(OpInvalid::new()),
        };

        let action = op.exec(self);

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
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
            .field("ram", &self.ram)
            .field("vram", &self.vram)
            .field("stack", &self.stack)
            .field("v", &self.v)
            .field("i", &self.i)
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("dt", &self.dt)
            .field("st", &self.st)
            .field("draw", &self.draw)
            .finish()
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
            rng: Box::new(rand::thread_rng()),
        }
    }
}
