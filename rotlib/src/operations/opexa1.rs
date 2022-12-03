//! The implementation of the ExA1 (SKNP Vx) operation.

use log::debug;

use crate::{Keys, Machine};

use super::{Operation, OperationResult};

/// Implements the ExA1 (SKNP Vx) operation. Skip next instruction if key with the value of `Vx` is not pressed.
pub(crate) struct Opexa1<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opexa1<'a> {
    // Creates a new Opexa1.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opexa1<'_> {
    /// Execute the operation ExA1 (SKNP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_exa1, x={}, keys={:?}", self.x, self.keys);

        if !self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}
