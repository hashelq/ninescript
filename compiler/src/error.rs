use crate::location::Location;

/// Represents an error during lexical scanning.
#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    NumberError,
    StringError,
    IndentationError,
    HashColorError,
    Eof,
    OtherError(String),
}
