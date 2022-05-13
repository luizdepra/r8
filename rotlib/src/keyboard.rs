pub const NUM_KEYS: usize = 16;

pub const KEY_0: usize = 0x0;
pub const KEY_1: usize = 0x1;
pub const KEY_2: usize = 0x2;
pub const KEY_3: usize = 0x3;
pub const KEY_4: usize = 0x4;
pub const KEY_5: usize = 0x5;
pub const KEY_6: usize = 0x6;
pub const KEY_7: usize = 0x7;
pub const KEY_8: usize = 0x8;
pub const KEY_9: usize = 0x9;
pub const KEY_A: usize = 0xA;
pub const KEY_B: usize = 0xB;
pub const KEY_C: usize = 0xC;
pub const KEY_D: usize = 0xD;
pub const KEY_E: usize = 0xE;
pub const KEY_F: usize = 0xF;

pub type Keys = [bool; NUM_KEYS];

pub struct Keyboard {
    keys: Keys,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }
}
