use strum::EnumString;

/// https://github.com/michaelsergio/nasm-instruction-set/blob/master/README.md
#[derive(Debug, PartialEq)]
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Instruction {
    Nop,
    
// Stack
    Push,
    Pop,

// Move
    Mov,
    Lea,
    Xchg,
    
// Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    
    Inc,
    Dec,
    Neg,
    
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
