//! Parser errors.

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEOF,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken => {
                write!(f, "Unexpected token.")
            }

            ParserError::UnexpectedEOF => {
                write!(f, "Unexpected end of file.")
            }
        }
    }
}

impl std::error::Error for ParserError {}
