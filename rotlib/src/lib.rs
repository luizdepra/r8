use log::debug;

// Sizes
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16;
const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const FONT_MEMORY_SIZE: usize = 80;
const FONT_CHAR_SIZE: usize = 5;
const SPRITE_WIDTH: usize = 8;
const VRAM_WIDTH: usize = 64;
const VRAM_HEIGHT: usize = 32;

// Registers
const GENERAL_REGISTER_NUMBER: usize = 16;
const CARRY: usize = 0xF;

// Memory
const FONT_INITIAL_ADDRESS: usize = 0;
const ROM_INITIAL_ADDRESS: usize = 512;

// Initial values
const INITIAL_PC_VALUE: usize = 0x200;
const FONT: [u8; FONT_MEMORY_SIZE] = [
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

pub type Vram = [u8; VRAM_WIDTH * VRAM_HEIGHT];
pub type Rom = [u8];
pub type Keys = [bool; NUM_KEYS];

type Ram = [u8; MEMORY_SIZE];
type Stack = [u16; STACK_SIZE];
type GeneralRegisterBank = [u8; GENERAL_REGISTER_NUMBER];

#[repr(usize)]
pub enum Key {
    _0 = 0x0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug)]
enum OperationResult {
    Next,
    NextAndRedraw,
    SkipNext,
    JumpTo(usize),
    WaitInput,
}

#[derive(Debug)]
pub struct Keyboard {
    keys: Keys,
}

impl Keyboard {
    pub fn keys_as_ref(&self) -> &Keys {
        &self.keys
    }

    pub fn press_key(&mut self, key: Key) {
        self.keys[key as usize] = true;
    }

    pub fn release_key(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }
}

#[derive(Debug)]
pub struct StepResult {
    pub redraw: bool,
    pub beep: bool,
}

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

impl Machine {
    pub fn vram_as_ref(&self) -> &Vram {
        &self.vram
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        self.ram[ROM_INITIAL_ADDRESS..ROM_INITIAL_ADDRESS + rom.len()].copy_from_slice(rom);

        debug!("rom_loaded, ram={:?}", self.ram);
    }

    pub fn step(&mut self, keys: &Keys) -> StepResult {
        debug!("step_pc, pc={:#06x?}", self.pc);

        let instr = (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;

        debug!("step_instruction, instr={:#06x?}", instr);

        let redraw = self.run_instruction(instr, keys);

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }

        let beep = self.st > 0;

        StepResult { redraw, beep }
    }

    fn run_instruction(&mut self, instr: u16, keys: &Keys) -> bool {
        let nibbles = (
            ((instr & 0xF000) >> 12) as u8,
            ((instr & 0x0F00) >> 8) as u8,
            ((instr & 0x00F0) >> 4) as u8,
            (instr & 0x000F) as u8,
        );

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

        match action {
            OperationResult::Next => {
                self.pc += 2;
                false
            }
            OperationResult::NextAndRedraw => {
                self.pc += 2;
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

    fn draw_sprite(&mut self, x: usize, y: usize, n: u8) {
        debug!("draw_sprite, x={}, y={}, n={}", x, y, n);

        for iy in 0..(n as usize) {
            let data = self.ram[self.i + iy];
            for ix in 0..SPRITE_WIDTH {
                let pixel = data & (0x80 >> ix);

                self.draw_pixel(pixel, ix, iy);
            }
        }
    }

    fn draw_pixel(&mut self, pixel: u8, x: usize, y: usize) {
        debug!("draw_pixel, x={}, y={}, pixel={}", x, y, pixel);

        if let Some(idx) = ram_index(x, y) {
            debug!("draw_pixel_ram_index, idx={}", idx);

            if pixel == 1 && self.vram[idx] == 1 {
                self.v[CARRY] = 1;
            }

            self.vram[idx] ^= pixel;
        }
    }

    // 00E0 - CLS
    // Clear the display.
    fn op_00e0(&mut self) -> OperationResult {
        self.vram = [0; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }

    // 00EE - RET
    // Return from a subroutine.
    fn op_00ee(&mut self) -> OperationResult {
        self.pc = self.stack[self.sp as usize] as usize;
        self.sp -= 1;

        OperationResult::Next
    }

    // 1nnn - JP addr
    // Jump to location nnn.
    fn op_1nnn(&mut self, nnn: u16) -> OperationResult {
        OperationResult::JumpTo(nnn as usize)
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.
    fn op_2nnn(&mut self, nnn: u16) -> OperationResult {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc as u16;

        OperationResult::JumpTo(nnn as usize)
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    fn op_3xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        if self.v[x as usize] == kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    fn op_4xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        if self.v[x as usize] != kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.
    fn op_5xy0(&mut self, x: u8, y: u8) -> OperationResult {
        if self.v[x as usize] == self.v[y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // 6xkk - LD Vx, byte
    // Set Vx = kk.
    fn op_6xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        self.v[x as usize] = kk;

        OperationResult::Next
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    fn op_7xkk(&mut self, x: u8, kk: u8) -> OperationResult {
        let x = x as usize;

        self.v[x] = self.v[x].wrapping_add(kk);

        OperationResult::Next
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    fn op_8xy0(&mut self, x: u8, y: u8) -> OperationResult {
        self.v[x as usize] = self.v[y as usize];

        OperationResult::Next
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx OR Vy.
    fn op_8xy1(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        self.v[x] |= self.v[y];

        OperationResult::Next
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.
    fn op_8xy2(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        self.v[x] |= self.v[y];

        OperationResult::Next
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy.
    fn op_8xy3(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        self.v[x] ^= self.v[y];

        OperationResult::Next
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.
    fn op_8xy4(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        let result = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.
    fn op_8xy5(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        let result = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result.0;
        self.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }

    // 8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx SHR 1.
    //
    // Ignoring Vy value following modern interpreters implementation.
    fn op_8xy6(&mut self, x: u8, _: u8) -> OperationResult {
        let x = x as usize;

        let result = self.v[x].overflowing_shr(1);
        self.v[x] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.
    fn op_8xy7(&mut self, x: u8, y: u8) -> OperationResult {
        let x = x as usize;
        let y = y as usize;

        let result = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = result.0;
        self.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.
    //
    // Ignoring Vy value following modern interpreters implementation.
    fn op_8xye(&mut self, x: u8, _: u8) -> OperationResult {
        let x = x as usize;

        let result = self.v[x].overflowing_shl(1);
        self.v[x] = result.0;
        self.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: u8, y: u8) -> OperationResult {
        if self.v[x as usize] != self.v[y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // Annn - LD I, addr
    // Set I = nnn.
    fn op_annn(&mut self, nnn: u16) -> OperationResult {
        self.i = nnn as usize;

        OperationResult::Next
    }

    // Bnnn - JP V0, addr
    // Jump to location nnn + V0.
    fn op_bnnn(&mut self, nnn: u16) -> OperationResult {
        OperationResult::JumpTo((nnn + self.v[0x0] as u16) as usize)
    }

    // Cxkk - RND Vx, byte
    // Set Vx = random byte AND kk.
    fn op_cxkk(&mut self, x: u8, kk: u8) -> OperationResult {
        let value = rand::random::<u8>();
        self.v[x as usize] = value & kk;

        OperationResult::Next
    }

    // Dxyn - DRW Vx, Vy, nibble
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) -> OperationResult {
        let x = (self.v[x as usize] as usize) % SCREEN_WIDTH;
        let y = (self.v[y as usize] as usize) % SCREEN_HEIGHT;

        self.v[CARRY] = 0;

        self.draw_sprite(x, y, n);

        OperationResult::NextAndRedraw
    }

    // Ex9E - SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: u8, keys: &Keys) -> OperationResult {
        if keys[self.v[x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // ExA1 - SKNP Vx
    // Skip next instruction if key with the value of Vx is not pressed.
    fn op_exa1(&mut self, x: u8, keys: &Keys) -> OperationResult {
        if !keys[self.v[x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }

    // Fx07 - LD Vx, DT
    // Set Vx = delay timer value.
    fn op_fx07(&mut self, x: u8) -> OperationResult {
        self.v[x as usize] = self.dt;

        OperationResult::Next
    }

    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: u8, keys: &Keys) -> OperationResult {
        if let Some(pos) = keys.iter().position(|&v| v) {
            self.v[x as usize] = pos as u8;
            return OperationResult::Next;
        }

        OperationResult::WaitInput
    }

    // Fx15 - LD DT, Vx
    // Set delay timer = Vx.
    fn op_fx15(&mut self, x: u8) -> OperationResult {
        self.dt = self.v[x as usize];

        OperationResult::Next
    }

    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.
    fn op_fx18(&mut self, x: u8) -> OperationResult {
        self.st = self.v[x as usize];

        OperationResult::Next
    }

    // Fx1E - ADD I, Vx
    // Set I = I + Vx.
    fn op_fx1e(&mut self, x: u8) -> OperationResult {
        self.i += self.v[x as usize] as usize;

        OperationResult::Next
    }

    // Fx29 - LD F, Vx
    // Set I = location of sprite for digit Vx.
    fn op_fx29(&mut self, x: u8) -> OperationResult {
        self.i = self.v[x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }

    // Fx33 - LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn op_fx33(&mut self, x: u8) -> OperationResult {
        let vx = self.v[x as usize];

        self.ram[self.i] = vx / 100 % 10;
        self.ram[self.i + 1] = vx / 10 % 10;
        self.ram[self.i + 2] = vx % 10;

        OperationResult::Next
    }

    // Fx55 - LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.
    fn op_fx55(&mut self, x: u8) -> OperationResult {
        (0..=x).for_each(|n| self.ram[self.i + n as usize] = self.v[n as usize]);

        OperationResult::Next
    }

    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.
    fn op_fx65(&mut self, x: u8) -> OperationResult {
        (0..=x).for_each(|n| self.v[n as usize] = self.ram[self.i + n as usize]);

        OperationResult::Next
    }
}

impl Default for Machine {
    fn default() -> Self {
        let mut ram = [0; MEMORY_SIZE];
        ram[FONT_INITIAL_ADDRESS..FONT_MEMORY_SIZE].copy_from_slice(&FONT);

        Self {
            ram,
            vram: [0; VRAM_WIDTH * VRAM_HEIGHT],
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

fn ram_index(x: usize, y: usize) -> Option<usize> {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        return None;
    }

    Some(y * SCREEN_HEIGHT + x)
}
