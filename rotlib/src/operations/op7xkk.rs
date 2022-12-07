//! The implementation of the 7xkk (ADD Vx, byte) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the 7xkk (ADD Vx, byte) operation. Set `Vx = Vx + kk`.
pub(crate) struct Op7xkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Op7xkk {
    // Creates a new Op7xkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Op7xkk {
    /// Execute the operation 7xkk (ADD Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_7xkk, x={}, kk={}", self.x, self.kk);

        let ix = self.x as usize;
        machine.v[ix] = machine.v[ix].wrapping_add(self.kk);

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op7xkk {
    use super::*;

    #[test]
    fn test_op7xkk_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0xA;

        machine.v[x as usize] = 0x2;

        let op = Op7xkk::new(x, kk);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0xC,
            "should add kk value into machine v[{:#02x?}]",
            x
        );
    }

    #[test]
    fn test_op7xkk_exec_wrapping() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0x1;

        machine.v[x as usize] = 0xFF;

        let op = Op7xkk::new(x, kk);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0x0,
            "should add kk value into machine v[{:#02x?}] by wrapping",
            x
        );
    }
}
