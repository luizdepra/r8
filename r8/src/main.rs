mod args;
mod error;
mod interpreter;
mod sound;

use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use clap::Parser;
use interpreter::Interpreter;
use log::{debug, error};
use pixels::{Pixels, SurfaceTexture};
use sound::Beep;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::args::CmdArgs;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

const DEFAULT_CYCLE_FREQUENCY: f64 = 540.0;
const DEFAULT_TIMER_FREQUENCY: f64 = 60.0;

fn main() -> Result<()> {
    env_logger::init();

    let args = CmdArgs::parse();

    let mut interpreter = Interpreter::default();
    if let Err(err) = interpreter.load(&args.rom) {
        debug!("load_error, error={:?}", err);
        bail!("Missing or invalid ROM file provided: {}", args.rom.display())
    }

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let size = LogicalSize::new((SCREEN_WIDTH * args.scale) as f64, (SCREEN_HEIGHT * args.scale) as f64);
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

    let beep = Beep::new()?;

    let target_frame_time = Duration::from_micros((1_000_000.0 / DEFAULT_CYCLE_FREQUENCY / args.speed) as u64);
    let target_timer_time = Duration::from_micros((1_000_000.0 / DEFAULT_TIMER_FREQUENCY / args.speed) as u64);
    let mut frame_last_time = Instant::now();
    let mut timer_last_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                if frame_last_time.elapsed() >= target_frame_time {
                    interpreter.update();

                    if interpreter.should_draw() {
                        interpreter.draw(pixels.get_frame_mut());

                        if pixels.render().map_err(|e| error!("rendering failed: {}", e)).is_err() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    frame_last_time = Instant::now();
                }

                if timer_last_time.elapsed() >= target_timer_time {
                    interpreter.update_timers();

                    if interpreter.should_beep() {
                        beep.play();
                    } else {
                        beep.pause();
                    }

                    timer_last_time = Instant::now();
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

            interpreter.read_input(&input);

            window.request_redraw();
        }
    });
}
