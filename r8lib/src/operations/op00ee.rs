//! The implementation of the 00EE (RET) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 00EE (RET) operation. Return from a subroutine.
pub(crate) struct Op00ee;

impl Op00ee {
    // Creates a new Op00ee.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00ee {
    /// Execute the operation 00EE (RET).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00ee");

        machine.pc = machine.stack[machine.sp as usize] as usize;
        machine.sp -= 1;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op00ee {
    use super::*;

    #[test]
    fn test_op00ee_exec() {
        let mut machine = Machine::default();

        machine.sp = 0x2;
        machine.pc = 0x1;
        machine.stack[0x2] = 0xF;

        let op = Op00ee::new();
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(machine.sp, 0x1, "should decrement stack point in 1");
        assert_eq!(
            machine.pc, 0xF,
            "program counter should point to the values stored in the stack"
        );
    }
}
