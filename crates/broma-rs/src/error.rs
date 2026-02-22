use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(
        "Unexpected token at line {line}, column {column}: expected {expected}, found {found}"
    )]
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },

    #[error("Unexpected end of file at line {line}, column {column}")]
    UnexpectedEof { line: usize, column: usize },

    #[error("Invalid hex literal '{value}' at line {line}, column {column}")]
    InvalidHexLiteral {
        value: String,
        line: usize,
        column: usize,
    },

    #[error("Class '{name}' inherits from itself at line {line}, column {column}")]
    SelfInheritance {
        name: String,
        line: usize,
        column: usize,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    PestError(String),
}

impl From<pest::error::Error<crate::parser::Rule>> for ParseError {
    fn from(err: pest::error::Error<crate::parser::Rule>) -> Self {
        ParseError::PestError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
