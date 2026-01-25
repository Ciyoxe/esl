use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TokenizeError {
    #[error("String literal should be closed (\"example\")")]
    UnterminatedString,
    #[error("Attribute name cannot be empty (@example)")]
    InvalidAttributeName,
    #[error("Invalid sequence ({0})")]
    UnexpectedChar(String),
    #[error("Unexpected closing brace ({0})")]
    UnexpectedClosingDelimiter(String),
    #[error("Unexpected end of file, brace pair was not closed")]
    UnclosedDelimiter,
}