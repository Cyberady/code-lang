//! Parser errors.
use lexer::span::Span;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEOF,

    BreakOutsideLoop,

    ContinueOutsideLoop,

    ReturnOutsideFunction,

    DuplicateParameter {
        name: String,
        span: Span,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken => { write!(f, "[E3001]: Unexpected token.") }

            ParserError::UnexpectedEOF => { write!(f, "[E3002]: Unexpected end of file.") }

            ParserError::BreakOutsideLoop => {
                write!(f, "[E3003]: 'break' can only be used inside a loop.")
            }

            ParserError::ContinueOutsideLoop => {
                write!(f, "[E3004]: 'continue' can only be used inside a loop.")
            }

            ParserError::ReturnOutsideFunction => {
                write!(f, "[E3005]: 'return' can only be used inside a function.")
            }
            
            ParserError::DuplicateParameter { name, .. } => {
                write!(f, "[E3006]: Duplicate parameter '{}'.", name)
            }
        }
    }
}

impl std::error::Error for ParserError {}
