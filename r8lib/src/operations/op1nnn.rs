//! The implementation of the 1nnn (JP addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 1nnn (JP addr) operation. Jump to location `nnn`.
pub(crate) struct Op1nnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Op1nnn {
    // Creates a new Op1nnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Op1nnn {
    /// Execute the operation 1nnn (JP addr).
    fn exec(&self, _: &mut Machine) -> OperationResult {
        debug!("op_1nnn, nnn={:#06x?}", self.nnn);

        OperationResult::JumpTo(self.nnn as usize)
    }
}

#[cfg(test)]
mod test_op1nnn {
    use super::*;

    #[test]
    fn test_op1nnn_exec() {
        let mut machine = Machine::default();
        let nnn = 0xA;

        let op = Op1nnn::new(nnn);
        let result = op.exec(&mut machine);

        assert_eq!(
            result,
            OperationResult::JumpTo(nnn as usize),
            "should return JumpTo(nnn)"
        );
    }
}
