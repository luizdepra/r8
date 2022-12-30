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
    /// A desirable value of Cycles Per Second.
    #[arg(short, long, default_value_t = 500)]
    pub cps: u64,
}
