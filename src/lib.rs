pub mod core;
pub mod parser;
pub mod interpreter;

pub use parser::{parse_lines, is_valid_identifier};
pub use interpreter::execute;
pub use interpreter::error::AsmrRuntimeError;
