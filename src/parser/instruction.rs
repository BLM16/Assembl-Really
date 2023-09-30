use strum::{EnumString, EnumIter, Display};

/// https://github.com/michaelsergio/nasm-instruction-set/blob/master/README.md
#[derive(Debug, PartialEq)]
#[derive(EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Instruction {
    Nop,
    
// Stack
    Push,
    Pop,

// Move
    Mov,
    Xchg,
    
// Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,
    Shl,
    Shr,
    
// Comparisons
    Cmp,
    And,
    Or,
    Not,
    Xor,
    Test,
    
// Jumps
    Jmp,
    Jz,
    Jnz,
    Jg,
    Jl,
    Jge,
    Jle,
    Je,
    Jne,
    
// Functions
    Call,
    Ret,
}
