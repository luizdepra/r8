//! The implementation of the Fx0A (LD Vx, K) operation.

use log::debug;

use crate::{Keys, Machine};

use super::{Operation, OperationResult};

/// Implements the Fx0A (LD Vx, K) operation. Wait for a key press, store the value of the key in `Vx`.
pub(crate) struct Opfx0a<'a> {
    /// The `x` operation parameter.
    x: u8,
    /// The `keys` operation parameter.
    keys: &'a Keys,
}

impl<'a> Opfx0a<'a> {
    // Creates a new Opfx0a.
    pub(crate) fn new(x: u8, keys: &'a Keys) -> Self {
        Self { x, keys }
    }
}

impl Operation for Opfx0a<'_> {
    /// Execute the operation Fx0A (LD Vx, K).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx0a, x={}, keys={:?}", self.x, self.keys);

        if let Some(pos) = self.keys.iter().position(|&v| v) {
            machine.v[self.x as usize] = pos as u8;
            return OperationResult::Next;
        }

        OperationResult::WaitInput
    }
}

#[cfg(test)]
mod test_opfx0a {
    use crate::keyboard::Key;

    use super::*;

    #[test]
    fn test_opfx0a_exec() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;
        let key = Key::A;

        keys[key as usize] = true;
        machine.v[x as usize] = 0x1;

        let op = Opfx0a::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], key as u8,
            "machine v[{:#02x?}] value should be the same as the pressed key",
            x
        );
    }

    #[test]
    fn test_opfx0a_exec_should_wait_input() {
        let mut machine = Machine::default();
        let mut keys = Keys::default();
        let x = 0x1;
        let key = Key::A;

        keys[key as usize] = false;
        machine.v[x as usize] = 0x1;

        let op = Opfx0a::new(x, &keys);
        let result = op.exec(&mut machine);

        assert_eq!(
            result,
            OperationResult::WaitInput,
            "should return WaitInput"
        );
        assert_eq!(
            machine.v[x as usize], 0x1,
            "machine v[{:#02x?}] value should not change",
            x
        );
    }
}
