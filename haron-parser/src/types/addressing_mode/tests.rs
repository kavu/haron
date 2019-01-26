use super::{possible_addressing_modes, addressing_mode};
use crate::types::addressing_mode::AddressingMode;

#[test]
fn test_possible_addressing_modes() {
    match possible_addressing_modes("#".into()) {
        Ok((_, result)) => assert_eq!('#', result),
        Err(err) => assert!(false, format!("{}", err)),
    }
}

#[test]
fn test_addressing_mode() {
    match addressing_mode("#".into()) {
        Ok((_, Some(result))) => assert_eq!(AddressingMode::IMMEDIATE, result),
        Ok((_, None)) => assert!(false, "no matches"),
        Err(err) => assert!(false, format!("{}", err)),
    }
}