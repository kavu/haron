use std::str::FromStr;

pub use nom::types::CompleteStr as Input;
use nom::*;

use crate::types::addressing_mode::*;
use crate::types::instruction::*;
use crate::types::op_code::*;
use crate::types::operand::*;

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

// named!(opcode(Input) -> OpCode,
//     ws!(alt!(
//         map!(tag_no_case!("MOV"), |_| OpCode::MOV) |
//         map!(tag_no_case!("NOP"), |_| OpCode::NOP) |
//         map!(tag_no_case!("SPL"), |_| OpCode::SPL) |
//         map!(tag_no_case!("DAT"), |_| OpCode::DAT))));

named!(opcode(Input) -> OpCode,
    ws!(map_res!(alt!(
        tag_no_case!("MOV") |
        tag_no_case!("NOP") |
        tag_no_case!("SPL") |
        tag_no_case!("DAT")
    ), |s: Input| OpCode::from_str(s.0))));

named!(possible_addressing_modes(Input) -> char,
    alt!(
        char!('#') |
        char!('@')));

named!(addressing_mode(Input) -> Option<AddressingMode>,
    opt!(map!(possible_addressing_modes, AddressingMode::from)));

named!(operands(Input) -> OperandsTuple,
    map!(opt!(alt!(normal_operands_pair | only_b_operand)), unwrap_operands));

named!(maybe_negative_digit(Input) -> Input,
    recognize!(pair!(
        opt!(tag!("-")),
        digit1)));

named!(normal_operands_pair(Input) -> Vec<Option<Operand>>,
    separated_nonempty_list!(tag!(","), operand));

named!(only_b_operand(Input) -> Vec<Option<Operand>>,
    map!(preceded!(ws!(tag!(",")), operand), |op| vec![None, op]));

named!(operand(Input) -> Option<Operand>,
    map!(ws!(pair!(addressing_mode, maybe_negative_digit)), to_operand));

named!(instruction(Input) -> Instruction,
    ws!(do_parse!(
        opcode: opcode >>
        operands_tuple: operands >>
        (Instruction {
            opcode,
            operand_a: operands_tuple.0,
            operand_b: operands_tuple.1,
        })
    )));

named!(pub parse(Input) -> Vec<Instruction>, many0!(instruction));

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

        let (_, result) = operands("0, -1".into()).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_one_operand() {
        let operand_a = Some(Operand {
            mode: None,
            value: -1,
        });
        let operand_b = None;
        let expected = (operand_a, operand_b);

        let (_, result) = operands("-1".into()).unwrap();

        assert_eq!(result, expected);
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
