//! CHIP-8's keyboard representation.

use crate::constants::NUM_KEYS;
use crate::types::Keys;

/// CHIP-8's keys.
#[derive(Debug)]
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

/// CHIP-8's keyboard.
#[derive(Debug)]
pub struct Keyboard {
    keys: Keys,
}

impl Keyboard {
    /// Returns the [`Keys`] array as a reference.
    pub fn keys_as_ref(&self) -> &Keys {
        &self.keys
    }

    /// Change a keys state to pressed.
    pub fn press_key(&mut self, key: Key) {
        self.keys[key as usize] = true;
    }

    /// Change a keys state to released.
    pub fn release_key(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }
}

impl Default for Keyboard {
    /// Creates a Keyboard with all keys in released state.
    fn default() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }
}
