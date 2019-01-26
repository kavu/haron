use super::{op_code_modifier, OpCodeModifier};

#[test]
fn test_display() {
    assert_eq!(".BA", format!("{}", OpCodeModifier::BA));
}

#[test]
fn test_op_code_modifier() {
    let (_, result) = op_code_modifier(".AB".into()).unwrap();
    assert_eq!(OpCodeModifier::AB, result);
}
