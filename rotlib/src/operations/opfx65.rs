//! The implementation of the Fx65 (LD Vx, [I]) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx65 (LD Vx, [I]) operation. Read registers `V0` through `Vx` from memory starting at location `I`.
pub(crate) struct Opfx65 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx65 {
    // Creates a new Opfx65.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx65 {
    /// Execute the operation Fx65 (LD Vx, [I]).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx65, x={}", self.x);

        (0..=self.x).for_each(|n| machine.v[n as usize] = machine.ram[machine.i + n as usize]);

        OperationResult::Next
    }
}
