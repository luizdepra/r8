//! The implementation of the 7xkk (ADD Vx, byte) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 7xkk (ADD Vx, byte) operation. Set `Vx = Vx + kk`.
pub(crate) struct Op7xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op7xkk {
    // Creates a new Op7xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op7xkk {
    /// Execute the operation 7xkk (ADD Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_7xkk, x={}, kk={}", self.x, self.kk);

        let ix = self.x as usize;
        machine.v[ix] = machine.v[ix].wrapping_add(self.kk);

        OperationResult::Next
    }
}
