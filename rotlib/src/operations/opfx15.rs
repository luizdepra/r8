//! The implementation of the Fx15 (LD DT, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx15 (LD DT, Vx) operation. Set `delay timer = Vx`.
pub(crate) struct Opfx15 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx15 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx15 {
    /// Execute the operation Fx15 (LD DT, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.dt = machine.v[self.x as usize];

        OperationResult::Next
    }
}
