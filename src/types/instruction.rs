use nom::types::CompleteStr as Input;
use nom::*;

use std::fmt;

use crate::types::op_code::{opcode, OpCode};
use crate::types::operand::{operands, Operand};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::addressing_mode::AddressingMode;
    use crate::types::instruction::Instruction;
    use crate::types::op_code::OpCode;
    use crate::types::operand::Operand;

    #[test]
    fn test_instruction() {
        let operand_a = Operand {
            mode: Some(AddressingMode::IMMEDIATE),
            value: 1,
        };

        let operand_b = Operand {
            mode: Some(AddressingMode::DIRECT),
            value: -1,
        };

        let expected = Instruction {
            opcode: OpCode::NOP,
            operand_a: Some(operand_a),
            operand_b: Some(operand_b),
        };

        let input = "\r\n\r\nNOP   #1,  @-1".into();
        let (_, result) = instruction(input).unwrap();

        assert_eq!(expected, result);
    }
}
