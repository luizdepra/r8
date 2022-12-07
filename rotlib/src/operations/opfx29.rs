//! The implementation of the Fx29 (LD F, Vx) operation.

use log::debug;

use crate::{Machine, FONT_CHAR_SIZE};

use super::{Operation, OperationResult};

/// Implements the Fx29 (LD F, Vx) operation. Set `I = location of sprite for digit Vx`.
pub(crate) struct Opfx29 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx29 {
    // Creates a new Opfx29.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx29 {
    /// Execute the operation Fx29 (LD F, Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx29, x={}", self.x);

        machine.i = machine.v[self.x as usize] as usize * FONT_CHAR_SIZE;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opfx29 {
    use super::*;

    #[test]
    fn test_opfx29_exec() {
        let mut machine = Machine::default();
        let x = 0x1;

        machine.v[x as usize] = 0xA;

        let op = Opfx29::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.i,
            (machine.v[x as usize] as usize) * FONT_CHAR_SIZE,
            "machine i register value should be same as v[{:#02x?}] multiplied to font char size",
            x
        );
    }
}
