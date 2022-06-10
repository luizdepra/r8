use anyhow::{bail, Result};
use clap::Parser;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rotlib::{Keyboard, Machine};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 8;

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
}

impl Interpreter {
    fn new() -> Self {
        Self {
            machine: Machine::default(),
            keyboard: Keyboard::default(),
            redraw: false,
        }
    }

    fn load(&mut self, path: PathBuf) -> Result<()> {
        let rom = fs::read(path)?;

        self.machine.load_rom(&rom);

        Ok(())
    }

    fn update(&mut self) {
        //let result = self.machine.step(self.keyboard.keys_as_ref());

        //self.redraw = result.redraw;
    }

    fn draw(&mut self, _frame: &mut [u8]) {
        self.redraw = false;
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

    let width = SCREEN_WIDTH * SCREEN_SCALE;
    let height = SCREEN_HEIGHT * SCREEN_SCALE;
    let size = LogicalSize::new(width as f64, height as f64);
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
    let mut pixels = Pixels::new(width, height, surface_texture)?;

    let mut interpreter = Interpreter::new();
    interpreter.load(args.rom).expect("failed to read rom");

    /*event_loop.run(move |event, _, control_flow| {`
        if let Event::RedrawRequested(_) = event {
            interpreter.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("rendering failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            interpreter.update();
            window.request_redraw();
        }
    });*/

    let mut timer = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                //let elapsed

                // interpreter.update();
                // interpreter.draw(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("rendering failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        };

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}
