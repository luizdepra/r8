//! The implementation of the 2nnn (CALL addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 2nnn (CALL addr) operation. Call subroutine at `nnn`.
pub(crate) struct Op2nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op2nnn {
    // Creates a new Op2nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op2nnn {
    /// Execute the operation 2nnn (CALL addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_2nnn, nnn={:#06x?}", self.nnn);

        machine.sp += 1;
        machine.stack[machine.sp] = machine.pc as u16;

        OperationResult::JumpTo(self.nnn as usize)
    }
}

#[cfg(test)]
mod test_op2nnn {
    use super::*;

    #[test]
    fn test_op2nnn_exec() {
        let mut machine = Machine::default();
        let nnn = 0xF;

        machine.sp = 0x5;
        machine.pc = 0xA;

        let op = Op2nnn::new(nnn);
        let result = op.exec(&mut machine);

        assert_eq!(
            result,
            OperationResult::JumpTo(nnn as usize),
            "should return JumpTo(nnn)"
        );
        assert_eq!(machine.sp, 0x6, "stack pointer should be incremented by one");
        assert_eq!(
            machine.stack[0x6], 0xA,
            "new stack position should points to old program counter value"
        );
    }
}
