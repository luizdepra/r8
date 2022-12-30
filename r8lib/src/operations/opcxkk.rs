//! The implementation of the Cxkk (RND Vx, byte) operation.

use log::debug;
use rand::Rng;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Cxkk (RND Vx, byte) operation. Set `Vx = random byte AND kk`.
pub(crate) struct Opcxkk {
    /// The `x` operation parameter.
    x: u8,
    /// The `kk` operation parameter.
    kk: u8,
}

impl Opcxkk {
    // Creates a new Opcxkk.
    pub(crate) fn new(x: u8, kk: u8) -> Self {
        Self { x, kk }
    }
}

impl Operation for Opcxkk {
    /// Execute the operation Cxkk (RND Vx, byte).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_cxkk, x={}, kk={}", self.x, self.kk);

        let value = machine.rng.gen::<u8>();
        machine.v[self.x as usize] = value & self.kk;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opcxkk {
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_opcxkk_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let kk = 0xA;

        machine.rng = Box::new(StepRng::new(4, 2));

        let op = Opcxkk::new(x, kk);

        let expected = [0u8, 2, 8, 10, 8];
        for expc in expected {
            let result = op.exec(&mut machine);

            assert_eq!(result, OperationResult::Next, "should return Next");
            assert_eq!(machine.v[x as usize], expc, "machine v[{}] should be {}", x, expc);
        }
    }
}
