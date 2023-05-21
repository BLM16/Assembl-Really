use std::{fmt::{self, Display, Formatter}, error::Error};

#[derive(Debug, PartialEq)]
pub struct AsmrRuntimeError {
    pub line_number: usize,
    pub cause: String,
}

impl AsmrRuntimeError {
    pub fn from(line_number: usize, cause: impl Into<String>) -> Self {
        AsmrRuntimeError { line_number, cause: cause.into() }
    }
}

impl Error for AsmrRuntimeError { }

impl Display for AsmrRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error on line {}: {}", self.line_number + 1, self.cause)
    }
}
