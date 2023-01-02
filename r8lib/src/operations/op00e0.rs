//! The implementation of the 00E0 (CLS) operation.

use log::debug;

use crate::{Machine, VRAM_HEIGHT, VRAM_WIDTH};

use super::{Operation, OperationResult};

/// Implements the 00E0 (CLS) operation. Clear the display.
pub(crate) struct Op00e0;

impl Op00e0 {
    // Creates a new Op00e0.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Operation for Op00e0 {
    /// Execute the operation 00E0 (CLS).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_00e0");

        machine.vram = [false; VRAM_WIDTH * VRAM_HEIGHT];

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op00e0 {
    use super::*;

    #[test]
    fn test_op00e0_exec() {
        let mut machine = Machine::default();

        // Make VRAM dirty.
        machine.vram.iter_mut().enumerate().for_each(|(i, v)| *v = i % 2 == 0);

        let op = Op00e0::new();
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        machine
            .vram
            .iter()
            .for_each(|v| assert_eq!(*v, false, "all VRAM values should be false"));
    }
}
