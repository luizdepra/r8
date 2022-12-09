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

#[cfg(test)]
mod test_opfx33 {
    use super::*;

    #[test]
    fn test_opfx33_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.i = 0xFF0;
        machine.v[x as usize] = 0xFF;

        let op = Opfx33::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.ram[machine.i], 0x2,
            "machine ram value at i should be the hundred part of v[{:#02x?}]",
            x
        );
        assert_eq!(
            machine.ram[machine.i + 1],
            0x5,
            "machine ram value at i+1 should be the hundred part of v[{:#02x?}]",
            x
        );
        assert_eq!(
            machine.ram[machine.i + 2],
            0x5,
            "machine ram value at i+2 should be the hundred part of v[{:#02x?}]",
            x
        );
    }
}
