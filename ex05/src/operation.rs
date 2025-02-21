use std::fmt::{write, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    And,
    Or,
    Xor,
    Not,
    IfThen,
    Equality,
}

impl Operation {
    pub fn new(symbol: char) -> (Self, u8) {
        match symbol {
            '!' => (Self::Not, 1),
            '&' => (Self::And, 2),
            '|' => (Self::Or, 2),
            '^' => (Self::Xor, 2),
            '>' => (Self::IfThen, 2),
            '=' => (Self::Equality, 2),
            _ => panic!("Unknown Symbol should have been seen earlier"),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation_character = match self {
            Operation::Equality => '=',
            Operation::Not => '!',
            Operation::And => '&',
            Operation::Or => '|',
            Operation::Xor => '^',
            Operation::IfThen => '>',
        };
        write!(f, "{operation_character}")
    }
}
