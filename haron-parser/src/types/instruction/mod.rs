use nom::types::CompleteStr as Input;
use nom::*;

use std::fmt;

use crate::types::op_code::{opcode, OpCode};
use crate::types::operand::{operands, Operand};

#[cfg(test)]
mod tests;

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

named!(pub instruction(Input) -> Instruction,
    ws!(do_parse!(
        opcode: opcode >>
        operands_tuple: operands >>
        (Instruction {
            opcode,
            operand_a: operands_tuple.0,
            operand_b: operands_tuple.1,
        })
    )));
