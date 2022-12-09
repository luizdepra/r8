//! CHIP-8's keyboard representation.

/// Number of Keys.
pub(crate) const NUM_KEYS: usize = 16;

/// An array of [`bool`]s hat represents the state of every CHIP-8's keyboard keys.
pub type Keys = [bool; NUM_KEYS];

/// CHIP-8's keys.
#[derive(Debug, Clone, Copy)]
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

#[cfg(test)]
mod test_keyboard {
    use super::*;

    #[test]
    fn test_press_key_and_release_key() {
        let values = [
            Key::_0,
            Key::_1,
            Key::_2,
            Key::_3,
            Key::_4,
            Key::_5,
            Key::_6,
            Key::_7,
            Key::_8,
            Key::_9,
            Key::A,
            Key::B,
            Key::C,
            Key::D,
            Key::E,
            Key::F,
        ];

        for v in values {
            let mut keyboard = Keyboard::default();

            keyboard.press_key(v);
            let keys = keyboard.keys_as_ref();
            for i in 0..NUM_KEYS {
                let expected = i == v as usize;

                assert_eq!(keys[i], expected, "keys {} value should be {}", i, expected)
            }

            keyboard.release_key(v);
            let keys = keyboard.keys_as_ref();
            for i in 0..NUM_KEYS {
                assert_eq!(keys[i], false, "keys {} value should be false", i)
            }
        }
    }
}
