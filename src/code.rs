use thiserror;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Left,
    Right,
    LoopStart,
    LoopEnd,
    Input,
    Output,
    Nop,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum ErrType {
    UnknownOp(char),
}
impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrType::UnknownOp(c) => write!(f, "Unknown operation: {}", c),
        }
    }
}

pub type BfRes = Result<Operation, ErrType>;

impl Operation {
    pub fn from_char(c: char) -> BfRes {
        match c {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Sub),
            '<' => Ok(Operation::Left),
            '>' => Ok(Operation::Right),
            '[' => Ok(Operation::LoopStart),
            ']' => Ok(Operation::LoopEnd),
            ',' => Ok(Operation::Input),
            '.' => Ok(Operation::Output),
            _ => Err(ErrType::UnknownOp(c)),
        }
    }
}

pub fn lex(st: &str) -> Vec<Operation> {
    st.chars()
        .map(|c| Operation::from_char(c).unwrap_or(Operation::Nop))
        .filter(|op| *op != Operation::Nop)
        .collect()
}
