//! The implementation of the 00EE (RET) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 00EE (RET) operation. Return from a subroutine.
pub(crate) struct Op00ee;

impl Op00ee {
    // Creates a new Op00ee.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00ee {
    /// Execute the operation 00EE (RET).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00ee");

        machine.pc = machine.stack[machine.sp as usize] as usize;
        machine.sp -= 1;

        OperationResult::Next
    }
}
