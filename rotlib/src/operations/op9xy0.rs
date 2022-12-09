//! The implementation of the 9xy0 (SNE Vx, Vy) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 9xy0 (SNE Vx, Vy) operation. Skip next instruction if `Vx != Vy`.
pub(crate) struct Op9xy0 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op9xy0 {
    // Creates a new Op9xy0.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op9xy0 {
    /// Execute the operation 9xy0 (SNE Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_9xy0, x={}, y={}", self.x, self.y);

        if machine.v[self.x as usize] != machine.v[self.y as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op9xy0 {
    use super::*;

    #[test]
    fn test_op9xy0_exec_should_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x2;
        machine.v[y as usize] = 0x1;

        let op = Op9xy0::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::SkipNext, "should return SkipNext");
    }

    #[test]
    fn test_op9xy0_exec_should_not_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x1;
        machine.v[y as usize] = 0x1;

        let op = Op9xy0::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
    }
}
