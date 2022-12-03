//! The implementation of the 8xy1 (OR Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 8xy1 (OR Vx, Vy) operation. Set `Vx = Vx OR Vy`.
pub(crate) struct Op8xy1 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy1 {
    // Creates a new Op8xy1.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy1 {
    /// Execute the operation 8xy1 (OR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy1, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] |= machine.v[self.y as usize];

        OperationResult::Next
    }
}