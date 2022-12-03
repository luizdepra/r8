//! The implementation of the 3xkk (SE Vx, byte) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 3xkk (SE Vx, byte) operation. Skip next instruction if `Vx = kk`.
pub(crate) struct Op3xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op3xkk {
    // Creates a new Op3xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op3xkk {
    /// Execute the operation 3xkk (SE Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_3xkk, x={}, kk={}", self.x, self.kk);

        if machine.v[self.x as usize] == self.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}
