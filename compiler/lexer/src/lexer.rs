//! Lexer implementation for the Code programming language.

use crate::{
    cursor::Cursor,
    error::LexerError,
    source::SourceFile,
    span::Span,
    token::{Token, TokenKind},
};

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

    /// Converts source code into tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.cursor.is_eof() {
            let ch = self.cursor.current().unwrap();

            // Skip whitespace
            if ch.is_whitespace() {
                self.cursor.advance();
                continue;
            }

            // Identifier / Keyword
            if Self::is_identifier_start(ch) {
                tokens.push(self.lex_identifier());
                continue;
            }

            // Unknown character
            return Err(LexerError::UnexpectedCharacter {
                character: ch,
                span: Span::new(
                    self.cursor.position(),
                    self.cursor.position() + ch.len_utf8(),
                ),
            });
        }

        tokens.push(Token::new(
            TokenKind::EOF,
            String::new(),
            Span::new(self.cursor.position(), self.cursor.position()),
        ));

        Ok(tokens)
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_identifier_part(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.cursor.position();

        let mut lexeme = String::new();

        while let Some(ch) = self.cursor.current() {
            if !Self::is_identifier_part(ch) {
                break;
            }

            lexeme.push(ch);
            self.cursor.advance();
        }

        let kind = match lexeme.as_str() {
            "const" => TokenKind::Const,
            "func" => TokenKind::Func,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => TokenKind::Identifier,
        };

        Token::new(kind, lexeme, Span::new(start, self.cursor.position()))
    }
}
