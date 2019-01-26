use std::fmt;

use crate::types::op_code::OpCode;
use crate::types::operand::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand_a: Option<Operand>,
    pub operand_b: Option<Operand>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.opcode))?;

        if self.operand_a.is_some() || self.operand_b.is_some() {
            f.write_str(" ")?;
        }

        if self.operand_a.is_some() {
            f.write_fmt(format_args!("{}", self.operand_a.clone().unwrap()))?;
        }

        if self.operand_b.is_some() {
            f.write_fmt(format_args!(",{}", self.operand_b.clone().unwrap()))?;
        }

        Ok(())
    }
}
