//! The implementation of the 4xkk (SNE Vx, byte) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 4xkk (SNE Vx, byte) operation. Skip next instruction if `Vx != kk`.
pub(crate) struct Op4xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op4xkk {
    // Creates a new Op4xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op4xkk {
    /// Execute the operation 4xkk (SNE Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_4xkk, x={}, kk={}", self.x, self.kk);

        if machine.v[self.x as usize] != self.kk {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op4xkk {
    use super::*;

    #[test]
    fn test_op4xkk_exec_should_not_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0xA;

        machine.v[x as usize] = 0xA;

        let op = Op4xkk::new(x, kk);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
    }

    #[test]
    fn test_op4xkk_exec_should_skip() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0xA;

        machine.v[x as usize] = 0xF;

        let op = Op4xkk::new(x, kk);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::SkipNext, "should return SkipNext");
    }
}
