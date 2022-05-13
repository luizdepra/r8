pub const DEFAULT_SCREEN_WIDTH: usize = 64;
pub const DEFAULT_SCREEN_HEIGHT: usize = 32;
pub const DEFAULT_SCREEN_SCALE: usize = 1;

pub trait Renderer {}

pub struct Screen {
    width: u32,
    height: u32,
    scale: u32,
    buffer: Vec<bool>,
    // renderer: Box<dyn Renderer>,
}

impl Screen {
    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        unimplemented!();
        false
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        unimplemented!();
    }

    pub fn render(&self) {
        unimplemented!();
    }
}

pub struct ScreenBuilder {
    width: u32,
    height: u32,
    scale: u32,
    renderer: Option<Box<dyn Renderer>>,
}

impl ScreenBuilder {
    pub fn new(width: u32, height: u32, scale: u32) -> Self {
        Self {
            width,
            height,
            scale,
            renderer: None,
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;

        self
    }

    pub fn scale(mut self, scale: u32) -> Self {
        self.scale = scale;

        self
    }

    pub fn renderer(mut self, renderer: Box<dyn Renderer>) -> Self {
        self.renderer = Some(renderer);

        self
    }

    pub fn build(self) -> Screen {
        Screen {
            width: self.width,
            height: self.height,
            scale: self.scale,
            buffer: vec![false; (self.width * self.height) as usize],
            // renderer: None,
        }
    }
}
