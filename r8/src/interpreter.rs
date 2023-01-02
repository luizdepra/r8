use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use log::debug;
use r8lib::{Key, Keyboard, Machine};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

#[derive(Debug)]
pub struct Interpreter {
    machine: Machine,
    keyboard: Keyboard,
    key_map: HashMap<VirtualKeyCode, Key>,
}

impl Interpreter {
    ///
    pub fn load(&mut self, path: &PathBuf) -> Result<()> {
        debug!("interpreter_load, path={path:?}");

        let rom = fs::read(path)?;

        self.machine.load_rom(&rom);

        Ok(())
    }

    pub fn should_draw(&self) -> bool {
        self.machine.should_draw()
    }

    pub fn should_beep(&self) -> bool {
        self.machine.should_beep()
    }

    pub fn update(&mut self) {
        debug!("interpreter_update");

        self.machine.step(self.keyboard.keys_as_ref());
    }

    pub fn update_timers(&mut self) {
        self.machine.update_timers()
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        debug!("interpreter_draw, redraw={}", self.should_draw());

        if !self.should_draw() {
            return;
        }

        for (dst, &src) in frame.chunks_exact_mut(4).zip(self.machine.vram_as_ref().iter()) {
            dst.copy_from_slice(if src { WHITE.as_slice() } else { BLACK.as_slice() });
        }
    }

    pub fn read_input(&mut self, input: &WinitInputHelper) {
        for (key, value) in self.key_map.iter() {
            if input.key_pressed(*key) {
                self.keyboard.press_key(*value);
            }
            if input.key_released(*key) {
                self.keyboard.release_key(*value);
            }
        }

        debug!("interpreter_read_input, keys={:?}", self.keyboard.keys_as_ref());
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            machine: Machine::default(),
            keyboard: Keyboard::default(),
            key_map: HashMap::from([
                (VirtualKeyCode::Key1, Key::_1),
                (VirtualKeyCode::Key2, Key::_2),
                (VirtualKeyCode::Key3, Key::_3),
                (VirtualKeyCode::Key4, Key::C),
                (VirtualKeyCode::Q, Key::_4),
                (VirtualKeyCode::W, Key::_5),
                (VirtualKeyCode::E, Key::_6),
                (VirtualKeyCode::R, Key::D),
                (VirtualKeyCode::A, Key::_7),
                (VirtualKeyCode::S, Key::_8),
                (VirtualKeyCode::D, Key::_9),
                (VirtualKeyCode::F, Key::E),
                (VirtualKeyCode::Z, Key::A),
                (VirtualKeyCode::X, Key::_0),
                (VirtualKeyCode::C, Key::B),
                (VirtualKeyCode::V, Key::F),
            ]),
        }
    }
}
