//! The implementation of the Cxkk (RND Vx, byte) operation.

use log::debug;

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

        let value = rand::random::<u8>();
        machine.v[self.x as usize] = value & self.kk;

        OperationResult::Next
    }
}
