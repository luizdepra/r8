//! r8 commandline arguments.

use clap::Parser;
use std::path::PathBuf;

/// A simple CHIP-8 interpreter made with rust, winit and pixels.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// A ROM file to be loaded.
    #[arg(required = true, value_parser)]
    pub rom: PathBuf,
    /// A value to scale every pixel from the screen.
    #[arg(short, long, default_value_t = 8)]
    pub scale: u32,
    /// A desirable speed of the interpreter.
    #[arg(short('S'), long, default_value_t = 1.0)]
    pub speed: f64,
}
