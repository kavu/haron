use nom::types::CompleteStr as Input;
use nom::*;

use std::fmt;

use crate::types::op_code::{opcode, OpCode};
use crate::types::op_code_modifier::{op_code_modifier, OpCodeModifier};
use crate::types::operand::{operands, Operand};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub modifier: Option<OpCodeModifier>,
    pub operand_a: Option<Operand>,
    pub operand_b: Option<Operand>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.opcode))?;

        if let Some(modifier) = self.modifier {
            f.write_fmt(format_args!("{}", modifier))?;
        }

        if self.operand_a.is_some() || self.operand_b.is_some() {
            f.write_str(" ")?;
        }

        if let Some(operand_a) = self.operand_a {
            f.write_fmt(format_args!("{}", operand_a))?;
        }

        if let Some(operand_b) = self.operand_b {
            f.write_fmt(format_args!(",{}", operand_b))?;
        }

        Ok(())
    }
}

named!(pub instruction(Input) -> Instruction,
    do_parse!(
        opt!(multispace) >>
        opcode: opcode >>
        modifier: opt!(op_code_modifier) >>
        multispace >>
        operands_tuple: operands >>
        opt!(multispace) >>
        (Instruction {
            opcode,
            modifier,
            operand_a: operands_tuple.0,
            operand_b: operands_tuple.1,
        })
    ));
