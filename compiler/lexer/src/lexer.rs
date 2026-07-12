//! Lexer implementation for the Code programming language.
//!
//! The lexer converts source code into a sequence of tokens.

use crate::{
    cursor::Cursor,
    error::LexerError,
    source::SourceFile,
    span::Span,
    token::{Token, TokenKind},
};

/// Lexical analyzer for the Code programming language.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer.
    pub fn new(source: &'a SourceFile) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    /// Converts the source code into tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        // Version 1:
        // We only produce EOF.
        tokens.push(Token::new(
            TokenKind::EOF,
            String::new(),
            Span::new(self.cursor.position(), self.cursor.position()),
        ));

        Ok(tokens)
    }
}
