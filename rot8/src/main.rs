use anyhow::{bail, Result};
use clap::Parser;
use log::{debug, error};
use pixels::{Pixels, SurfaceTexture};
use rotlib::{Keyboard, Machine};
use std::fs;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 8;

const WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

const TARGET_CPS: u64 = 500;

/// A simple CHIP-8 interpreter made with rust, winit and pixels.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(required = true, parse(from_os_str))]
    rom: PathBuf,
}

struct Interpreter {
    machine: Machine,
    keyboard: Keyboard,
    redraw: bool,
    should_update: bool,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            machine: Machine::default(),
            keyboard: Keyboard::default(),
            redraw: false,
            should_update: false,
        }
    }

    fn load(&mut self, path: PathBuf) -> Result<()> {
        debug!("interpreter_load, path={path:?}");

        let rom = fs::read(path)?;

        self.machine.load_rom(&rom);

        Ok(())
    }

    fn update(&mut self) {
        debug!("interpreter_update");

        let result = self.machine.step(self.keyboard.keys_as_ref());
        self.redraw = result.redraw;
    }

    fn draw(&mut self, frame: &mut [u8]) {
        debug!("interpreter_draw, redraw={}", self.redraw);

        if !self.redraw {
            return;
        }

        for (dst, &src) in frame
            .chunks_exact_mut(4)
            .zip(self.machine.vram_as_ref().iter())
        {
            dst.copy_from_slice(if src != 0 {
                WHITE.as_slice()
            } else {
                BLACK.as_slice()
            });
        }

        self.redraw = false;
        self.should_update = false;
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    if !args.rom.is_file() {
        bail!(
            "Missing or invalid ROM file provided: {}",
            args.rom.display()
        );
    }

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let size = LogicalSize::new(
        (SCREEN_WIDTH * SCREEN_SCALE) as f64,
        (SCREEN_HEIGHT * SCREEN_SCALE) as f64,
    );
    let window = WindowBuilder::new()
        .with_title("rot8")
        .with_resizable(false)
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    let mut interpreter = Interpreter::new();
    interpreter.load(args.rom).expect("failed to read rom");

    let target_frametime = Duration::from_micros(1_000_000 / TARGET_CPS);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let frame_time = Instant::now();

                interpreter.update();
                interpreter.draw(pixels.get_frame());

                if pixels
                    .render()
                    .map_err(|e| error!("rendering failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(wait_for) = target_frametime.checked_sub(frame_time.elapsed()) {
                    debug!("frame_sleep, wait_for={}", wait_for.as_micros());
                    sleep(wait_for);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        };

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Space) {
                interpreter.should_update = true;
            }
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}
