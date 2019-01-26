use std::fmt;

use crate::types::addressing_mode::AddressingMode;

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub mode: Option<AddressingMode>,
    pub value: i64,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.mode.clone().unwrap_or_default(), self.value)
    }
}
