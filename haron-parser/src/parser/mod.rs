use nom::types::CompleteStr as Input;
use nom::*;

use crate::types::instruction::{comment, instruction, Instruction};

#[cfg(test)]
mod tests;

fn instruction_or_comment(input: Input) -> IResult<Input, Option<Instruction>> {
    if let Ok((rest, _)) = comment(input) {
        return Ok((rest, None));
    }

    instruction(input)
}

pub fn parse_string(input: Input<'_>) -> IResult<Input, Vec<Instruction>> {
    let (rest, instructions) = many1!(input, instruction_or_comment)?;

    let filtered_instructions = instructions
        .into_iter()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    Ok((rest, filtered_instructions))
}
