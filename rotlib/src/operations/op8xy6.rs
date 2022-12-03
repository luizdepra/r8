//! The implementation of the 8xy6 (SHR Vx {, Vy}) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xy6 (SHR Vx {, Vy}) operation. Set `Vx = Vx SHR 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xy6 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy6 {
    // Creates a new Op8xy6.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy6 {
    /// Execute the operation 8xy6 (SHR Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy6, x={}", self.x);

        let ix = self.x as usize;
        let result = machine.v[ix].overflowing_shr(1);
        machine.v[ix] = result.0;
        machine.v[CARRY] = result.1 as u8;

        OperationResult::Next
    }
}
