use std::{fs, path::PathBuf};

use anyhow::Result;
use log::debug;
use r8lib::{Key, Keyboard, Machine};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

#[derive(Debug, Default)]
pub struct Interpreter {
    machine: Machine,
    keyboard: Keyboard,
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

    pub fn draw(&mut self, frame: &mut [u8]) {
        debug!("interpreter_draw, redraw={}", self.should_draw());

        if !self.should_draw() {
            return;
        }

        for (dst, &src) in frame.chunks_exact_mut(4).zip(self.machine.vram_as_ref().iter()) {
            dst.copy_from_slice(if src { WHITE.as_slice() } else { BLACK.as_slice() });
        }

        self.machine.update_timers()
    }

    pub fn read_input(&mut self, input: &WinitInputHelper) {
        // keypress
        if input.key_pressed(VirtualKeyCode::Key1) {
            self.keyboard.press_key(Key::_1)
        }
        if input.key_pressed(VirtualKeyCode::Key2) {
            self.keyboard.press_key(Key::_2)
        }
        if input.key_pressed(VirtualKeyCode::Key3) {
            self.keyboard.press_key(Key::_3)
        }
        if input.key_pressed(VirtualKeyCode::Key4) {
            self.keyboard.press_key(Key::C)
        }
        if input.key_pressed(VirtualKeyCode::Q) {
            self.keyboard.press_key(Key::_4)
        }
        if input.key_pressed(VirtualKeyCode::W) {
            self.keyboard.press_key(Key::_5)
        }
        if input.key_pressed(VirtualKeyCode::E) {
            self.keyboard.press_key(Key::_6)
        }
        if input.key_pressed(VirtualKeyCode::R) {
            self.keyboard.press_key(Key::D)
        }
        if input.key_pressed(VirtualKeyCode::A) {
            self.keyboard.press_key(Key::_7)
        }
        if input.key_pressed(VirtualKeyCode::S) {
            self.keyboard.press_key(Key::_8)
        }
        if input.key_pressed(VirtualKeyCode::D) {
            self.keyboard.press_key(Key::_9)
        }
        if input.key_pressed(VirtualKeyCode::F) {
            self.keyboard.press_key(Key::E)
        }
        if input.key_pressed(VirtualKeyCode::Z) {
            self.keyboard.press_key(Key::A)
        }
        if input.key_pressed(VirtualKeyCode::X) {
            self.keyboard.press_key(Key::_0)
        }
        if input.key_pressed(VirtualKeyCode::C) {
            self.keyboard.press_key(Key::B)
        }
        if input.key_pressed(VirtualKeyCode::V) {
            self.keyboard.press_key(Key::F)
        }

        // keyrelease
        if input.key_released(VirtualKeyCode::Key1) {
            self.keyboard.release_key(Key::_1)
        }
        if input.key_released(VirtualKeyCode::Key2) {
            self.keyboard.release_key(Key::_2)
        }
        if input.key_released(VirtualKeyCode::Key3) {
            self.keyboard.release_key(Key::_3)
        }
        if input.key_released(VirtualKeyCode::Key4) {
            self.keyboard.release_key(Key::C)
        }
        if input.key_released(VirtualKeyCode::Q) {
            self.keyboard.release_key(Key::_4)
        }
        if input.key_released(VirtualKeyCode::W) {
            self.keyboard.release_key(Key::_5)
        }
        if input.key_released(VirtualKeyCode::E) {
            self.keyboard.release_key(Key::_6)
        }
        if input.key_released(VirtualKeyCode::R) {
            self.keyboard.release_key(Key::D)
        }
        if input.key_released(VirtualKeyCode::A) {
            self.keyboard.release_key(Key::_7)
        }
        if input.key_released(VirtualKeyCode::S) {
            self.keyboard.release_key(Key::_8)
        }
        if input.key_released(VirtualKeyCode::D) {
            self.keyboard.release_key(Key::_9)
        }
        if input.key_released(VirtualKeyCode::F) {
            self.keyboard.release_key(Key::E)
        }
        if input.key_released(VirtualKeyCode::Z) {
            self.keyboard.release_key(Key::A)
        }
        if input.key_released(VirtualKeyCode::X) {
            self.keyboard.release_key(Key::_0)
        }
        if input.key_released(VirtualKeyCode::C) {
            self.keyboard.release_key(Key::B)
        }
        if input.key_released(VirtualKeyCode::V) {
            self.keyboard.release_key(Key::F)
        }

        debug!("interpreter_read_input, keys={:?}", self.keyboard.keys_as_ref());
    }
}
