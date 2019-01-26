use std::fmt;

use nom::types::CompleteStr as Input;
use nom::*;

#[cfg(test)]
mod tests;

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
