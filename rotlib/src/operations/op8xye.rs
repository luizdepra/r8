//! The implementation of the 8xyE (SHL Vx {, Vy}) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xyE (SHL Vx {, Vy}) operation. Set `Vx = Vx SHL 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xye {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xye {
    // Creates a new Op8xye.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xye {
    /// Execute the operation 8xyE (SHL Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xye, x={}", self.x);

        let ix = self.x as usize;

        let result = machine.v[ix].overflowing_shl(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}
