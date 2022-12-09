//! The implementation of the 8xyE (SHL Vx {, Vy}) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xyE (SHL Vx {, Vy}) operation. Set `Vx = Vx SHL 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xye {
    /// The `x` operation parameter.
    x: u8,
}

impl Op8xye {
    // Creates a new Op8xye.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Op8xye {
    /// Execute the operation 8xyE (SHL Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xye, x={}", self.x);

        let ix = self.x as usize;
        machine.v[CARRY] = machine.v[ix] & 0x80;
        machine.v[ix] <<= 0x01;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xye {
    use super::*;

    #[test]
    fn test_op8xye_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.v[x as usize] = 0x79;

        let op = Op8xye::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0xF2,
            "machine v[{:#02x?}] value should be updated by right-shifting by one",
            x
        );
        assert_eq!(
            machine.v[CARRY], 0x0,
            "machine v[0xF] value should be the least-significant bit of v[{:#02x?}]",
            x
        );
    }
}
