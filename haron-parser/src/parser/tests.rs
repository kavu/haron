use super::parse_string;
use crate::types::instruction::Instruction;
use crate::types::op_code::OpCode;
use crate::types::operand::Operand;

#[test]
fn test_parse_string() {
    let operand = Operand {
        mode: None,
        value: 1,
    };

    let expected = vec![
        Instruction {
            opcode: OpCode::DAT,
            operand_a: Some(operand),
            operand_b: Some(operand),
        },
        Instruction {
            opcode: OpCode::MOV,
            operand_a: Some(operand),
            operand_b: None,
        },
        Instruction {
            opcode: OpCode::SPL,
            operand_a: None,
            operand_b: Some(operand),
        },
    ];

    let input = "\r\n\r\nDAT 1, 1  \n\nMOV 1\n\n\nSPL ,1\n".into();
    let (_, result) = parse_string(input).unwrap();

    assert_eq!(expected, result);
}
