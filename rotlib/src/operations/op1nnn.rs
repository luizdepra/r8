//! The implementation of the 1nnn (JP addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 1nnn (JP addr) operation. Jump to location `nnn`.
pub(crate) struct Op1nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op1nnn {
    // Creates a new Op1nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op1nnn {
    /// Execute the operation 1nnn (JP addr).
    fn exec(&self, _: &mut Machine) -> OperationResult {
        debug!("op_1nnn, nnn={:#06x?}", self.nnn);

        OperationResult::JumpTo(self.nnn as usize)
    }
}
