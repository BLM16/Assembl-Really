pub mod core;
pub mod parser;
pub mod interpreter;

pub use parser::parse_lines;
pub use interpreter::execute;
pub use interpreter::error::AsmrRuntimeError;
