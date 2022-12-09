//! The implementation of the Fx18 (LD ST, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx18 (LD ST, Vx) operation. Set `sound timer = Vx`.
pub(crate) struct Opfx18 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx18 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx18 {
    /// Execute the operation Fx18 (LD ST, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.st = machine.v[self.x as usize];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opfx18 {
    use super::*;

    #[test]
    fn test_opfx18_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.v[x as usize] = 0xC;
        machine.st = 0x0;

        let op = Opfx18::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.st, machine.v[x as usize],
            "machine sound timer value should be same as v[{:#02x?}]",
            x
        );
    }
}
