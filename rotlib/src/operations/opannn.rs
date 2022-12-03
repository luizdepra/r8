//! The implementation of the Annn (LD I, addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Annn (LD I, addr) operation. Set `I = nnn`.
pub(crate) struct Opannn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opannn {
    // Creates a new Opannn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opannn {
    /// Execute the operation Annn (LD I, addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_annn, nnn={}", self.nnn);

        machine.i = self.nnn as usize;

        OperationResult::Next
    }
}
