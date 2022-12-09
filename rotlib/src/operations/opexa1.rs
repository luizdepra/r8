//! The implementation of the ExA1 (SKNP Vx) operation.

use log::debug;

use crate::{Keys, Machine};

use super::{Operation, OperationResult};

/// Implements the ExA1 (SKNP Vx) operation. Skip next instruction if key with the value of `Vx` is not pressed.
pub(crate) struct Opexa1<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opexa1<'a> {
    // Creates a new Opexa1.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opexa1<'_> {
    /// Execute the operation ExA1 (SKNP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_exa1, x={}, keys={:?}", self.x, self.keys);

        if !self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opexa1 {
    use crate::keyboard::Key;

    use super::*;

    #[test]
    fn test_opexa1_exec_should_skip() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;

        machine.v[x as usize] = Key::_3 as u8;
        keys[Key::_3 as usize] = false;

        let op = Opexa1::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::SkipNext, "should return SkipNext");
    }

    #[test]
    fn test_opexa1_exec_should_not_skip() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;

        machine.v[x as usize] = Key::_3 as u8;
        keys[Key::_3 as usize] = true;

        let op = Opexa1::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
    }
}
