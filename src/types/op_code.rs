use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    MOV,
    NOP,
    SPL,
    DAT,
}

#[derive(Debug, PartialEq)]
pub struct ParseOpCodeError;

impl std::error::Error for ParseOpCodeError {
    fn description(&self) -> &str {
        "Can't parse str as OpCode"
    }
}

impl fmt::Display for ParseOpCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseOpCodeError()")
    }
}

impl FromStr for OpCode {
    type Err = ParseOpCodeError;

    fn from_str(s: &str) -> Result<OpCode, Self::Err> {
        match s.to_uppercase().as_str() {
            "MOV" => Ok(OpCode::MOV),
            "NOP" => Ok(OpCode::NOP),
            "SPL" => Ok(OpCode::SPL),
            "DAT" => Ok(OpCode::DAT),
            _ => Err(ParseOpCodeError),
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(OpCode::MOV, "MOV".parse().unwrap());
        assert_eq!(OpCode::NOP, "NOP".parse().unwrap());
        assert_eq!(OpCode::SPL, "SPL".parse().unwrap());
        assert_eq!(OpCode::DAT, "DAT".parse().unwrap());
    }

    #[test]
    fn test_wrong_from_str() {
        match "SOMETHING".parse::<OpCode>() {
            Ok(_) => assert!(false, "must be a ParseOpCodeError error"),
            Err(ParseOpCodeError) => assert!(true),
        };
    }
}
