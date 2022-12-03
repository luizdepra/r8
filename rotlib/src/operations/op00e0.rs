//! The implementation of the 00E0 (CLS) operation.

use log::debug;

use crate::{Machine, VRAM_HEIGHT, VRAM_WIDTH};

use super::{Operation, OperationResult};

/// Implements the 00E0 (CLS) operation. Clear the display.
pub(crate) struct Op00e0;

impl Op00e0 {
    // Creates a new Op00e0.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00e0 {
    /// Execute the operation 00E0 (CLS).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00e0");

        machine.vram = [false; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }
}
