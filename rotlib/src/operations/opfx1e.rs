//! The implementation of the Fx1E (ADD I, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx1E (ADD I, Vx) operation. Set `I = I + Vx`.
pub(crate) struct Opfx1e {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx1e {
    // Creates a new Opfx1e.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx1e {
    /// Execute the operation Fx1E (ADD I, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx1e, x={}", self.x);

        machine.i += machine.v[self.x as usize] as usize;

        OperationResult::Next
    }
}
