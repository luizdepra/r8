//! The implementation of the 8xy0 (LD Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 8xy0 (LD Vx, Vy) operation. Set `Vx = Vy`.
pub(crate) struct Op8xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy0 {
    // Creates a new Op8xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy0 {
    /// Execute the operation 8xy0 (LD Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy0, x={}, y={}", self.x, self.y);

        machine.v[self.x as usize] = machine.v[self.y as usize];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xy0 {
    use super::*;

    #[test]
    fn test_op8xy0_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[y as usize] = 0xF;

        let op = Op8xy0::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], machine.v[y as usize],
            "machine v[{:#02x?}] value should be same as v[{:#02x?}] value",
            x, y
        );
    }
}
