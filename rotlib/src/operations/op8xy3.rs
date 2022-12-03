//! The implementation of the 8xy3 (XOR Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 8xy3 (XOR Vx, Vy) operation. Set `Vx = Vx XOR Vy`.
pub(crate) struct Op8xy3 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy3 {
    // Creates a new Op8xy3.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy3 {
    /// Execute the operation 8xy3 (XOR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy3, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] ^= machine.v[self.y as usize];

        OperationResult::Next
    }
}
