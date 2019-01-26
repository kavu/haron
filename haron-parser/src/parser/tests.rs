use super::parse_string;
use crate::types::instruction::Instruction;
use crate::types::op_code::OpCode;
use crate::types::op_code_modifier::OpCodeModifier;
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
            modifier: Some(OpCodeModifier::A),
            operand_a: Some(operand),
            operand_b: Some(operand),
        },
        Instruction {
            opcode: OpCode::MOV,
            modifier: Some(OpCodeModifier::BA),
            operand_a: Some(operand),
            operand_b: None,
        },
        Instruction {
            opcode: OpCode::SPL,
            modifier: None,
            operand_a: None,
            operand_b: Some(operand),
        },
    ];

    let input = "\r\n\r\n    DAT.A 1, 1  \n\nMOV.BA   1\n\n\nSPL ,1\n".into();
    let (_, result) = parse_string(input).unwrap();

    assert_eq!(expected, result);
}
