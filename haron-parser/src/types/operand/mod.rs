use std::fmt;

use nom::types::CompleteStr as Input;
use nom::*;

use crate::types::addressing_mode::{addressing_mode, AddressingMode};

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operand {
    pub mode: Option<AddressingMode>,
    pub value: i64,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.mode.unwrap_or_default(), self.value)
    }
}

type OperandsTuple = (Option<Operand>, Option<Operand>);

fn to_operand(input: (Option<AddressingMode>, Input)) -> Option<Operand> {
    let mode = input.0;

    match i64::from_str_radix(&input.1, 10) {
        Ok(value) => Some(Operand { mode, value }),
        Err(err) => panic!(err),
    }
}

fn unwrap_operands(opt: Option<Vec<Option<Operand>>>) -> OperandsTuple {
    let vec = opt.unwrap_or_default();
    let operand_a: Option<Operand> = match vec.get(0) {
        Some(value) => *value,
        None => None,
    };

    let operand_b: Option<Operand> = match vec.get(1) {
        Some(value) => *value,
        None => None,
    };

    (operand_a, operand_b)
}

named!(maybe_negative_digit(Input) -> Input,
    recognize!(pair!(
        opt!(tag!("-")),
        digit1)));

named!(operand(Input) -> Option<Operand>,
    map!(ws!(pair!(addressing_mode, maybe_negative_digit)), to_operand));

named!(only_b_operand(Input) -> Vec<Option<Operand>>,
    map!(preceded!(ws!(tag!(",")), operand), |op| vec![None, op]));

named!(normal_operands_pair(Input) -> Vec<Option<Operand>>,
    separated_nonempty_list!(tag!(","), operand));

named!(pub operands(Input) -> OperandsTuple,
    map!(opt!(alt!(normal_operands_pair | only_b_operand)), unwrap_operands));
