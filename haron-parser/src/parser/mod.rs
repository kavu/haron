use nom::types::CompleteStr as Input;
use nom::*;

use crate::types::instruction::{instruction, Instruction};

#[cfg(test)]
mod tests;

named!(pub parse_string(Input) -> Vec<Instruction>,
    many0!(instruction));
