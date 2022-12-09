//! The implementation of the Bnnn (JP V0, addr) operation.

use log::debug;

use crate::{Machine, ZERO};

use super::{Operation, OperationResult};

/// Implements the Bnnn (JP V0, addr) operation. Jump to location `nnn + V0`.
pub(crate) struct Opbnnn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opbnnn {
    // Creates a new Opbnnn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opbnnn {
    /// Execute the operation Bnnn (JP V0, addr) .
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_bnnn, nnn={}", self.nnn);

        OperationResult::JumpTo((self.nnn + machine.v[ZERO] as u16) as usize)
    }
}

#[cfg(test)]
mod test_opbnnn {
    use super::*;

    #[test]
    fn test_opannn_exec() {
        let mut machine = Machine::default();
        let nnn = 0x11;

        machine.v[ZERO] = 0x5;

        let op = Opbnnn::new(nnn);
        let result = op.exec(&mut machine);

        let expected = (nnn + machine.v[ZERO] as u16) as usize;
        assert_eq!(
            result,
            OperationResult::JumpTo(expected),
            "should return JumpTo(nnn + v[0x0])"
        );
    }
}
