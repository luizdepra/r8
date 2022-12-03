//! The implementation of the Fx55 (LD [I], Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx55 (LD [I], Vx) operation. Store registers `V0` through `Vx` in memory starting at location `I`.
pub(crate) struct Opfx55 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx55 {
    // Creates a new Opfx55.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx55 {
    /// Execute the operation Fx55 (LD [I], Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx55, x={}", self.x);

        (0..=self.x).for_each(|n| machine.ram[machine.i + n as usize] = machine.v[n as usize]);

        OperationResult::Next
    }
}
