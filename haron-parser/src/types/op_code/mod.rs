use std::fmt;

use nom::types::CompleteStr as Input;
use nom::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    MOV,
    NOP,
    SPL,
    DAT,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            OpCode::MOV => "MOV",
            OpCode::NOP => "NOP",
            OpCode::SPL => "SPL",
            OpCode::DAT => "DAT",
        };

        write!(f, "{}", symbol)
    }
}

named!(pub opcode(Input) -> OpCode,
    ws!(alt!(
        map!(tag_no_case!("MOV"), |_| OpCode::MOV) |
        map!(tag_no_case!("NOP"), |_| OpCode::NOP) |
        map!(tag_no_case!("SPL"), |_| OpCode::SPL) |
        map!(tag_no_case!("DAT"), |_| OpCode::DAT))));
