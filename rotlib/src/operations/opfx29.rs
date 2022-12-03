//! The implementation of the Fx29 (LD F, Vx) operation.

use log::debug;

use crate::{Machine, FONT_CHAR_SIZE};

use super::{Operation, OperationResult};

/// Implements the Fx29 (LD F, Vx) operation. Set `I = location of sprite for digit Vx`.
pub(crate) struct Opfx29 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx29 {
    // Creates a new Opfx29.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx29 {
    /// Execute the operation Fx29 (LD F, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx29, x={}", self.x);

        machine.i = machine.v[self.x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }
}
