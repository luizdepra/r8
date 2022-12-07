//! The implementation of the 8xy6 (SHR Vx {, Vy}) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xy6 (SHR Vx {, Vy}) operation. Set `Vx = Vx SHR 1`.
///
/// Ignoring Vy value following modern interpreters implementation.
pub(crate) struct Op8xy6 {
    /// The `x` operation parameter.
    x: u8,
}

impl Op8xy6 {
    // Creates a new Op8xy6.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Op8xy6 {
    /// Execute the operation 8xy6 (SHR Vx {, Vy}).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy6, x={}", self.x);

        let ix = self.x as usize;
        machine.v[CARRY] = machine.v[ix] & 0x01;
        machine.v[ix] >>= 0x01;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xy6 {
    use super::*;

    #[test]
    fn test_op8xy6_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.v[x as usize] = 0x2;

        let op = Op8xy6::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0x1,
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
