use super::instruction;
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