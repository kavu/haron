use super::{instruction,comment};

use crate::types::addressing_mode::AddressingMode;
use crate::types::instruction::Instruction;
use crate::types::op_code::OpCode;
use crate::types::op_code_modifier::OpCodeModifier;
use crate::types::operand::Operand;

#[test]
fn test_instruction_with_modifier_and_comment() {
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
        modifier: Some(OpCodeModifier::BA),
        operand_a: Some(operand_a),
        operand_b: Some(operand_b),
    };

    let input = "\r\n\r\n  NOP.BA   #1,  @-1   ; this is comment".into();
    let (_, result) = instruction(input).unwrap();

    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_comment() {
    let input = "\r\n\n\n   ;this is comment     \n".into();
    let expected = "this is comment";

    let (_, result) = comment(input).unwrap();

    assert_eq!(expected, result.0);
}

#[test]
fn test_instruction_wo_modifier() {
    let operand_a = Operand {
        mode: Some(AddressingMode::IMMEDIATE),
        value: 1,
    };

    let operand_b = Operand {
        mode: Some(AddressingMode::DIRECT),
        value: -1,
    };

    let expected = Some(Instruction {
        opcode: OpCode::NOP,
        modifier: None,
        operand_a: Some(operand_a),
        operand_b: Some(operand_b),
    });

    let input = "\r\n\r\nNOP   #1,  @-1".into();
    let (_, result) = instruction(input).unwrap();

    assert_eq!(expected, result);
}
