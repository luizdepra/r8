//! A simple implementation of a CHIP-8 interpreter.

//#![deny(clippy::all)]
#![allow(dead_code)]

mod keyboard;
mod machine;
mod operations;

pub use crate::keyboard::*;
pub use crate::machine::*;
