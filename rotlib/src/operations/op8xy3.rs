//! The implementation of the 8xy3 (XOR Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 8xy3 (XOR Vx, Vy) operation. Set `Vx = Vx XOR Vy`.
pub(crate) struct Op8xy3 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy3 {
    // Creates a new Op8xy3.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy3 {
    /// Execute the operation 8xy3 (XOR Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy3, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] ^= machine.v[self.y as usize];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xy3 {
    use super::*;

    #[test]
    fn test_op8xy3_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x3;
        machine.v[y as usize] = 0x9;

        let op = Op8xy3::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0xA,
            "machine v[{:#02x?}] value should be updated with a bitwise XOR with v[{:#02x?}] value",
            x, y
        );
        assert_eq!(
            machine.v[y as usize], 0x9,
            "machine v[{:#02x?}] value should not change",
            y
        );
    }
}
