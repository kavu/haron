use std::fmt;

use nom::types::CompleteStr as Input;
use nom::{alt, call, char, error_position, map, named, opt};

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    NONE,
    IMMEDIATE,
    DIRECT,
}

impl Copy for AddressingMode {}

impl Default for AddressingMode {
    fn default() -> Self {
        AddressingMode::NONE
    }
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            AddressingMode::NONE => "",
            AddressingMode::IMMEDIATE => "#",
            AddressingMode::DIRECT => "@",
        };

        write!(f, "{}", symbol)
    }
}

impl From<char> for AddressingMode {
    fn from(c: char) -> Self {
        match c {
            '#' => AddressingMode::IMMEDIATE,
            '@' => AddressingMode::DIRECT,
            _ => AddressingMode::NONE,
        }
    }
}

named!(possible_addressing_modes(Input) -> char,
    alt!(
        char!('#') |
        char!('@')));

named!(pub addressing_mode(Input) -> Option<AddressingMode>,
    opt!(map!(possible_addressing_modes, AddressingMode::from)));

#[cfg(test)]
mod tests {
    use super::*;

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
}
