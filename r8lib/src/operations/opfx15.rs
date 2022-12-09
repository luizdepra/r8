//! The implementation of the Fx15 (LD DT, Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx15 (LD DT, Vx) operation. Set `delay timer = Vx`.
pub(crate) struct Opfx15 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx15 {
    // Creates a new Opfx15.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx15 {
    /// Execute the operation Fx15 (LD DT, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx15, x={}", self.x);

        machine.dt = machine.v[self.x as usize];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opfx15 {
    use super::*;

    #[test]
    fn test_opfx15_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.v[x as usize] = 0xC;
        machine.dt = 0x0;

        let op = Opfx15::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.dt, machine.v[x as usize],
            "machine delay timer value should be same as v[{:#02x?}]",
            x
        );
    }
}
