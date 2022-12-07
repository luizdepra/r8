//! The implementation of the Ex9E (SKP Vx) operation.

use log::debug;

use crate::{Keys, Machine};

use super::{Operation, OperationResult};

/// Implements the Ex9E (SKP Vx) operation. Skip next instruction if key with the value of `Vx` is pressed.
pub(crate) struct Opex9e<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opex9e<'a> {
    // Creates a new Opex9e.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opex9e<'_> {
    /// Execute the operation Ex9E (SKP Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_ex9e, x={}, keys={:?}", self.x, self.keys);

        if self.keys[machine.v[self.x as usize] as usize] {
            return OperationResult::SkipNext;
        }

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opex9e {
    use crate::keyboard::Key;

    use super::*;

    #[test]
    fn test_opex9e_exec_should_skip() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;

        machine.v[x as usize] = Key::_3 as u8;
        keys[Key::_3 as usize] = true;

        let op = Opex9e::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::SkipNext, "should return SkipNext");
    }

    #[test]
    fn test_opex9e_exec_should_not_skip() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;

        machine.v[x as usize] = Key::_3 as u8;
        keys[Key::_3 as usize] = false;

        let op = Opex9e::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
    }
}
