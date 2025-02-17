#[derive(Debug)]
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
    pub fn get_operation_closure(&self) -> impl FnOnce(bool, bool) -> bool {
        match self {
            Operation::Equality => |first, second| first == second,
            Operation::Not => |first: bool, _| !first,
            Operation::And => |first, second| first & second,
            Operation::Or => |first, second| first | second,
            Operation::Xor => |first, second| first ^ second,
            Operation::IfThen => |first: bool, second| {
                if !first || (first && second) {
                    true
                } else {
                    false
                }
            },
        }
    }
}
