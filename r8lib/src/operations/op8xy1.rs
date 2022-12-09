//! The implementation of the 8xy1 (OR Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 8xy1 (OR Vx, Vy) operation. Set `Vx = Vx OR Vy`.
pub(crate) struct Op8xy1 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy1 {
    // Creates a new Op8xy1.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy1 {
    /// Execute the operation 8xy1 (OR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy1, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] |= machine.v[self.y as usize];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xy1 {
    use super::*;

    #[test]
    fn test_op8xy1_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x3;
        machine.v[y as usize] = 0x5;

        let op = Op8xy1::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0x7,
            "machine v[{:#02x?}] value should be updated with a bitwise OR with v[{:#02x?}] value",
            x, y
        );
        assert_eq!(
            machine.v[y as usize], 0x5,
            "machine v[{:#02x?}] value should not change",
            y
        );
    }
}
