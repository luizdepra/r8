//! The implementation of the Fx65 (LD Vx, [I]) operation.

use log::debug;

use crate::Machine;

use super::{Operation, OperationResult};

/// Implements the Fx65 (LD Vx, [I]) operation. Read registers `V0` through `Vx` from memory starting at location `I`.
pub(crate) struct Opfx65 {
    /// The `x` operation parameter.
    x: u8,
}

impl Opfx65 {
    // Creates a new Opfx65.
    pub(crate) fn new(x: u8) -> Self {
        Self { x }
    }
}

impl Operation for Opfx65 {
    /// Execute the operation Fx65 (LD Vx, [I]).
    fn exec(&self, machine: &mut Machine) -> OperationResult {
        debug!("op_fx65, x={}", self.x);

        (0..=self.x).for_each(|n| machine.v[n as usize] = machine.ram[machine.i + n as usize]);

        OperationResult::Next
    }
}

#[cfg(test)]
mod test_opfx65 {
    use crate::CARRY;

    use super::*;

    #[test]
    fn test_opfx65_exec() {
        let mut machine = Machine::default();
        let x = 0x5;

        machine.i = 0xFF0;
        (0..=CARRY as usize).for_each(|n| machine.v[n] = 0);
        (0..=x as usize).for_each(|n| machine.ram[machine.i + n] = n as u8);

        let op = Opfx65::new(x);
        let result = op.exec(&mut machine);

        assert_eq!(result, OperationResult::Next, "should return Next");
        (0..=x as usize).for_each(|n| {
            assert_eq!(
                machine.v[n],
                machine.ram[machine.i + n] as u8,
                "machine v[{}] should be equal to ram value at i+{}",
                n,
                n
            )
        });
        ((x as usize + 1)..=CARRY).for_each(|n| assert_eq!(machine.v[n], 0, "machine v[{}] should be equal to 0", n));
    }
}
