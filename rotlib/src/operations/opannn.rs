//! The implementation of the Annn (LD I, addr) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Annn (LD I, addr) operation. Set `I = nnn`.
pub(crate) struct Opannn {
    /// The `nnn` operation parameter.
    nnn: u16,
}

impl Opannn {
    // Creates a new Opannn.
    pub(crate) fn new(nnn: u16) -> Self {
        Self { nnn }
    }
}

impl Operation for Opannn {
    /// Execute the operation Annn (LD I, addr).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_annn, nnn={}", self.nnn);

        machine.i = self.nnn as usize;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opannn {
    use super::*;

    #[test]
    fn test_opannn_exec() {
        let mut machine = Machine::default();
        let nnn = 0x11;

        let op = Opannn::new(nnn);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.i, nnn as usize,
            "machine I register value should be equal to nnn"
        );
    }
}
