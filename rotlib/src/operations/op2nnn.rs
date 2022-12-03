//! The implementation of the 2nnn (CALL addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 2nnn (CALL addr) operation. Call subroutine at `nnn`.
pub(crate) struct Op2nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op2nnn {
    // Creates a new Op2nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op2nnn {
    /// Execute the operation 2nnn (CALL addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_2nnn, nnn={:#06x?}", self.nnn);

        machine.sp += 1;
        machine.stack[machine.sp as usize] = machine.pc as u16;

        OperationResult::JumpTo(self.nnn as usize)
    }
}
