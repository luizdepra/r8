use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rotlib::{Keyboard, Machine};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 1;

#[allow(dead_code)]
struct Interpreter {
    machine: Machine,
    keyboard: Keyboard,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            machine: Machine::default(),
            keyboard: Keyboard::default(),
        }
    }

    fn update(&mut self) {}

    fn draw(&self, _frame: &mut [u8]) {}
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let width = SCREEN_WIDTH * SCREEN_SCALE;
    let height = SCREEN_HEIGHT * SCREEN_SCALE;
    let size = LogicalSize::new(width as f64, height as f64);
    let window = WindowBuilder::new()
        .with_title("rot8")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;

    let mut interpreter = Interpreter::new();

    event_loop.run(move |event, _, control_flow| {
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
    });
}
