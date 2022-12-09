//! The implementation of the 6xkk (LD Vx, byte) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 6xkk (LD Vx, byte) operation. Set `Vx = kk`.
pub(crate) struct Op6xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op6xkk {
    // Creates a new Op6xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op6xkk {
    /// Execute the operation 6xkk (LD Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_6xkk, x={}, kk={}", self.x, self.kk);

        machine.v[self.x as usize] = self.kk;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op6xkk {
    use super::*;

    #[test]
    fn test_op6xkk_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0xA;

        let op = Op6xkk::new(x, kk);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], kk,
            "machine v[{:#02x?}] value should be {}",
            x, kk
        );
    }
}
