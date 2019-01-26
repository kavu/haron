use super::OpCode;

#[test]
fn test_display() {
    assert_eq!("DAT", format!("{}", OpCode::DAT));
}