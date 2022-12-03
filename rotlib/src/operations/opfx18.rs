//! The implementation of the Fx18 (LD ST, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx18 (LD ST, Vx) operation. Set `sound timer = Vx`.
pub(crate) struct Opfx18 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx18 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx18 {
    /// Execute the operation Fx18 (LD ST, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.st = machine.v[self.x as usize];

        OperationResult::Next
    }
}
