use std::fmt;

use nom::types::CompleteStr as Input;
use nom::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCodeModifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

impl fmt::Display for OpCodeModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            OpCodeModifier::A => ".A",
            OpCodeModifier::B => ".B",
            OpCodeModifier::AB => ".AB",
            OpCodeModifier::BA => ".BA",
            OpCodeModifier::F => ".F",
            OpCodeModifier::X => ".X",
            OpCodeModifier::I => ".I",
        };

        write!(f, "{}", symbol)
    }
}

named!(pub op_code_modifier(Input) -> OpCodeModifier,
    alt!(
        map!(tag_no_case!(".AB"), |_| OpCodeModifier::AB) |
        map!(tag_no_case!(".BA"), |_| OpCodeModifier::BA) |
        map!(tag_no_case!(".A"), |_| OpCodeModifier::A) |
        map!(tag_no_case!(".B"), |_| OpCodeModifier::B) |
        map!(tag_no_case!(".F"), |_| OpCodeModifier::F) |
        map!(tag_no_case!(".X"), |_| OpCodeModifier::X) |
        map!(tag_no_case!(".I"), |_| OpCodeModifier::I)));
