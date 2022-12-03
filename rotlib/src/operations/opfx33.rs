//! The implementation of the Fx33 (LD B, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx33 (LD B, Vx) operation. Store BCD representation of `Vx` in memory locations `I`, `I+1`, and `I+2`.
pub(crate) struct Opfx33 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx33 {
    // Creates a new Opfx33.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx33 {
    /// Execute the operation Fx33 (LD B, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx33, x={}", self.x);

        let vx = machine.v[self.x as usize];

        machine.ram[machine.i] = vx / 100 % 10;
        machine.ram[machine.i + 1] = vx / 10 % 10;
        machine.ram[machine.i + 2] = vx % 10;

        OperationResult::Next
    }
}
