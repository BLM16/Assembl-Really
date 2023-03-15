use std::{fmt::{self, Display, Formatter}, error::Error};

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub line_number: i32,
    pub cause: String,
}

impl Error for ParserError { }

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error on line {}: {}", self.line_number, self.cause)
    }
}
