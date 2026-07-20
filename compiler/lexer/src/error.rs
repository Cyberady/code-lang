//! Error types for the Code lexer.
//!
//! The lexer returns these errors whenever it encounters invalid source code.

use crate::span::Span;

/// Represents every error that can occur while lexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    /// An unexpected character was encountered.
    UnexpectedCharacter { character: char, span: Span },

    /// A string literal was not properly closed.
    UnterminatedString { span: Span },

    /// A block comment was not properly closed.
    UnterminatedComment { span: Span },

    /// An invalid numeric literal was encountered.
    InvalidNumber { span: Span },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedCharacter { character, .. } => {
                write!(f, "Unexpected character '{}'.", character)
            }

            LexerError::UnterminatedString { .. } => {
                write!(f, "Unterminated string literal.")
            }

            LexerError::UnterminatedComment { .. } => {
                write!(f, "Unterminated block comment.")
            }

            LexerError::InvalidNumber { .. } => {
                write!(f, "Invalid numeric literal.")
            }
        }
    }
}

impl std::error::Error for LexerError {}
