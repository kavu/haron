use std::fmt;

use nom::types::CompleteStr as Input;
use nom::*;

use crate::types::addressing_mode::{addressing_mode, AddressingMode};

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub mode: Option<AddressingMode>,
    pub value: i64,
}

impl Copy for Operand {}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.mode.clone().unwrap_or_default(), self.value)
    }
}

type OperandsTuple = (Option<Operand>, Option<Operand>);

fn to_operand(input: (Option<AddressingMode>, Input)) -> Option<Operand> {
    let mode = input.0;
    let value = i64::from_str_radix(&input.1, 10).unwrap();

    Some(Operand { mode, value })
}

fn unwrap_operands(opt: Option<Vec<Option<Operand>>>) -> OperandsTuple {
    let vec = opt.unwrap_or_default();
    let operand_a: Option<Operand> = match vec.get(0) {
        Some(value) => value.clone(),
        None => None,
    };

    let operand_b: Option<Operand> = match vec.get(1) {
        Some(value) => value.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_two_operands() {
        let operand_a = Some(Operand {
            mode: None,
            value: 0,
        });

        let operand_b = Some(Operand {
            mode: None,
            value: -1,
        });

        let expected = (operand_a, operand_b);

        match operands("0, -1".into()) {
            Ok((_, result)) => assert_eq!(expected, result),
            Err(err) => assert!(false, err),
        }
    }

    #[test]
    fn parse_one_operand() {
        let operand_a = Some(Operand {
            mode: None,
            value: 1,
        });
        let operand_b = None;
        let expected = (operand_a, operand_b);

        match operands("1".into()) {
            Ok((_, result)) => assert_eq!(expected, result),
            Err(err) => assert!(false, err),
        }
    }

    #[test]
    fn parse_invalid_number_of_operands() {
        let operand_a = Some(Operand {
            mode: None,
            value: -1,
        });
        let operand_b = Some(Operand {
            mode: None,
            value: 1222,
        });
        let expected = (operand_a, operand_b);

        let (_, result) = operands("-1, 1222, 2".into()).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_only_b_operand() {
        let operand_b = Some(Operand {
            mode: None,
            value: 1222,
        });
        let expected = (None, operand_b);

        let (_, result) = operands(", 1222".into()).unwrap();

        assert_eq!(result, expected);
    }
}
