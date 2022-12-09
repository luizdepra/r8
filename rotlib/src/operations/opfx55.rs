//! The implementation of the Fx55 (LD [I], Vx) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx55 (LD [I], Vx) operation. Store registers `V0` through `Vx` in memory starting at location `I`.
pub(crate) struct Opfx55 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx55 {
    // Creates a new Opfx55.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx55 {
    /// Execute the operation Fx55 (LD [I], Vx).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx55, x={}", self.x);

        (0..=self.x).for_each(|n| machine.ram[machine.i + n as usize] = machine.v[n as usize]);

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opfx55 {
    use crate::CARRY;

    use super::*;

    #[test]
    fn test_opfx55_exec() {
        let mut machine = Machine::default();
        let x = 0x5u8;

        machine.i = 0xFF0;
        (0..=x as usize).for_each(|n| machine.v[n] = n as u8);

        let op = Opfx55::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        (0..=x as usize).for_each(|n| {
            assert_eq!(
                machine.ram[machine.i + n],
                n as u8,
                "machine ram at i+{} should be {}",
                n,
                n
            )
        });
        ((x as usize + 1)..=CARRY).for_each(|n| {
            assert_eq!(
                machine.ram[machine.i + n],
                0,
                "machine ram at i+{} should be 0",
                n
            )
        });
    }
}
