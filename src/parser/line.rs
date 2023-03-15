use super::{Instruction, Token};

#[derive(Debug, PartialEq)]
pub enum Line {
    Instruction {
        instruction: Instruction,
        params: Vec<Token>,
    },
    Label(String),
    Variable {
        identifier: String,
        mem_type: MemType,
        params: Vec<Token>,
    },
    Blank,
}

#[derive(Debug, PartialEq)]
pub enum MemType {
    Db, Resb
}
