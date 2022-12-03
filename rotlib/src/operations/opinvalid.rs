//! The implementation of the invalid operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements a dummy invalid operation.
pub(crate) struct OpInvalid;

impl OpInvalid {
    // Creates a new OpInvalid.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for OpInvalid {
    /// Skips the current invalid operation.
    fn exec(&self, _: &mut Machine) -> OperationResult {
        debug!("op_invalid");
        OperationResult::Next
    }
}
