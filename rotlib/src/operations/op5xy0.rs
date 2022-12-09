//! The implementation of the 5xy0 (SE Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 5xy0 (SE Vx, Vy) operation. Skip next instruction if `Vx = Vy`.
pub(crate) struct Op5xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op5xy0 {
    // Creates a new Op5xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op5xy0 {
    /// Execute the operation 5xy0 (SE Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_5xy0, x={}, y={}", self.x, self.y);

        if machine.v[self.x as usize] == machine.v[self.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op5xy0 {
    use super::*;

    #[test]
    fn test_op5xy0_exec_should_not_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0xA;
        machine.v[y as usize] = 0xF;

        let op = Op5xy0::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
    }

    #[test]
    fn test_op5xy0_exec_should_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0xA;
        machine.v[y as usize] = 0xA;

        let op = Op5xy0::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::SkipNext, "should return SkipNext");
    }
}
