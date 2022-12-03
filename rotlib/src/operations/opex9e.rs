//! The implementation of the Ex9E (SKP Vx) operation.

use log::debug;

use crate::{Keys, Machine};

use super::{Operation, OperationResult};

/// Implements the Ex9E (SKP Vx) operation. Skip next instruction if key with the value of `Vx` is pressed.
pub(crate) struct Opex9e<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opex9e<'a> {
    // Creates a new Opex9e.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opex9e<'_> {
    /// Execute the operation Ex9E (SKP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_ex9e, x={}, keys={:?}", self.x, self.keys);

        if self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}
