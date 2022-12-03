//! The implementation of the 8xy4 (ADD Vx, Vy) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xy4 (ADD Vx, Vy) operation. Set `Vx = Vx + Vy`, set `VF = carry`.
pub(crate) struct Op8xy4 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy4 {
    // Creates a new Op8xy4.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy4 {
    /// Execute the operation 8xy4 (ADD Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy4, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[ix].overflowing_add(machine.v[iy]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}
