//! The implementation of the Fx07 (LD Vx, DT) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx07 (LD Vx, DT) operation. Set `Vx = delay timer value`.
pub(crate) struct Opfx07 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx07 {
    // Creates a new Opexa1.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx07 {
    /// Execute the operation Fx07 (LD Vx, DT).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx07, x={}, dt={}", self.x, machine.dt);

        machine.v[self.x as usize] = machine.dt;

        OperationResult::Next
    }
}
