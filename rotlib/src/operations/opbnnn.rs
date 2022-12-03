//! The implementation of the Bnnn (JP V0, addr) operation.

use log::debug;

use crate::{Machine, ZERO};

use super::{Operation, OperationResult};

/// Implements the Bnnn (JP V0, addr) operation. Jump to location `nnn + V0`.
pub(crate) struct Opbnnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opbnnn {
    // Creates a new Opbnnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opbnnn {
    /// Execute the operation Bnnn (JP V0, addr) .
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_bnnn, nnn={}", self.nnn);

        OperationResult::JumpTo((self.nnn + machine.v[ZERO] as u16) as usize)
    }
}
