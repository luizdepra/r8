//! The implementation of the 8xy5 (SUB Vx, Vy) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xy5 (SUB Vx, Vy) operation. Set `Vx = Vx - Vy`, set `VF = NOT borrow`.
pub(crate) struct Op8xy5 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy5 {
    // Creates a new Op8xy5.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy5 {
    /// Execute the operation 8xy5 (SUB Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy5, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[ix].overflowing_sub(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}
