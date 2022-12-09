//! The implementation of the 8xy7 (SUBN Vx, Vy) operation.

use log::debug;

use crate::{Machine, CARRY};

use super::{Operation, OperationResult};

/// Implements the 8xy7 (SUBN Vx, Vy) operation. Set `Vx = Vy - Vx`, set `VF = NOT borrow`.
pub(crate) struct Op8xy7 {
    /// The `x` operation parameter.
    x: u8,
    /// The `y` operation parameter.
    y: u8,
}

impl Op8xy7 {
    // Creates a new Op8xy7.
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl Operation for Op8xy7 {
    /// Execute the operation 1nnn 8xy7 (SUBN Vx, Vy).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_8xy7, x={}, y={}", self.x, self.y);

        let ix = self.x as usize;
        let iy = self.y as usize;
        let result = machine.v[iy].overflowing_sub(machine.v[ix]);
        machine.v[ix] = result.0;
        machine.v[CARRY] = !result.1 as u8;

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_op8xy7 {
    use super::*;

    #[test]
    fn test_op8xy7_exec() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x1;
        machine.v[y as usize] = 0x2;

        let op = Op8xy7::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0x1,
            "machine v[{:#02x?}] value should be subtracted by v[{:#02x?}] value",
            x, y
        );
        assert_eq!(
            machine.v[y as usize], 0x2,
            "machine v[{:#02x?}] value should not change",
            y
        );
        assert_eq!(machine.v[CARRY], 0x1, "machine v[0xF] value should be one",);
    }

    #[test]
    fn test_op8xy7_exec_wrapping() {
        let mut machine = Machine::default();
        let x = 0x1;
        let y = 0x2;

        machine.v[x as usize] = 0x1;
        machine.v[y as usize] = 0x0;

        let op = Op8xy7::new(x, y);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        assert_eq!(
            machine.v[x as usize], 0xFF,
            "machine v[{:#02x?}] value should be subtracted by v[{:#02x?}] value wrapping",
            x, y
        );
        assert_eq!(
            machine.v[y as usize], 0x0,
            "machine v[{:#02x?}] value should not change",
            y
        );
        assert_eq!(machine.v[CARRY], 0x0, "machine v[0xF] value should be zero",);
    }
}
