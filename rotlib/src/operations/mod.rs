//! The implementation of all CHIP-8 operations.

mod op00e0;
mod op00ee;
mod op1nnn;
mod op2nnn;
mod op3xkk;
mod op4xkk;
mod op5xy0;
mod op6xkk;
mod op7xkk;
mod op8xy0;
mod op8xy1;
mod op8xy2;
mod op8xy3;
mod op8xy4;
mod op8xy5;
mod op8xy6;
mod op8xy7;
mod op8xye;
mod op9xy0;
mod opannn;
mod opbnnn;
mod opcxkk;
mod opdxyn;
mod opex9e;
mod opexa1;
mod opfx07;
mod opfx0a;
mod opfx15;
mod opfx18;
mod opfx1e;
mod opfx29;
mod opfx33;
mod opfx55;
mod opfx65;
mod opinvalid;

use crate::Machine;

pub(crate) use crate::operations::op00e0::Op00e0;
pub(crate) use crate::operations::op00ee::Op00ee;
pub(crate) use crate::operations::op1nnn::Op1nnn;
pub(crate) use crate::operations::op2nnn::Op2nnn;
pub(crate) use crate::operations::op3xkk::Op3xkk;
pub(crate) use crate::operations::op4xkk::Op4xkk;
pub(crate) use crate::operations::op5xy0::Op5xy0;
pub(crate) use crate::operations::op6xkk::Op6xkk;
pub(crate) use crate::operations::op7xkk::Op7xkk;
pub(crate) use crate::operations::op8xy0::Op8xy0;
pub(crate) use crate::operations::op8xy1::Op8xy1;
pub(crate) use crate::operations::op8xy2::Op8xy2;
pub(crate) use crate::operations::op8xy3::Op8xy3;
pub(crate) use crate::operations::op8xy4::Op8xy4;
pub(crate) use crate::operations::op8xy5::Op8xy5;
pub(crate) use crate::operations::op8xy6::Op8xy6;
pub(crate) use crate::operations::op8xy7::Op8xy7;
pub(crate) use crate::operations::op8xye::Op8xye;
pub(crate) use crate::operations::op9xy0::Op9xy0;
pub(crate) use crate::operations::opannn::Opannn;
pub(crate) use crate::operations::opbnnn::Opbnnn;
pub(crate) use crate::operations::opcxkk::Opcxkk;
pub(crate) use crate::operations::opdxyn::Opdxyn;
pub(crate) use crate::operations::opex9e::Opex9e;
pub(crate) use crate::operations::opexa1::Opexa1;
pub(crate) use crate::operations::opfx07::Opfx07;
pub(crate) use crate::operations::opfx0a::Opfx0a;
pub(crate) use crate::operations::opfx15::Opfx15;
pub(crate) use crate::operations::opfx18::Opfx18;
pub(crate) use crate::operations::opfx1e::Opfx1e;
pub(crate) use crate::operations::opfx29::Opfx29;
pub(crate) use crate::operations::opfx33::Opfx33;
pub(crate) use crate::operations::opfx55::Opfx55;
pub(crate) use crate::operations::opfx65::Opfx65;
pub(crate) use crate::operations::opinvalid::OpInvalid;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OperationResult {
    Next,
    NextAndRedraw,
    SkipNext,
    JumpTo(usize),
    WaitInput,
}

/// A trait for CHIP-8 operations.
pub(crate) trait Operation {
    /// Executes the operation.
    fn exec(&self, machine: &mut Machine) -> OperationResult;
}
