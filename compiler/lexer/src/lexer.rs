//! Lexer implementation for the Code programming language.

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

    /// Converts source code into a sequence of tokens.
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

            // Number
            if ch.is_ascii_digit() {
                tokens.push(self.lex_number()?);
                continue;
            }

            // Operator
            if let Some(token) = self.lex_operator()? {
                tokens.push(token);
                continue;
            }

            // Delimiter
            if let Some(token) = self.lex_delimiter() {
                tokens.push(token);
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

        // End of file
        tokens.push(Token::new(
            TokenKind::EOF,
            String::new(),
            Span::new(self.cursor.position(), self.cursor.position()),
        ));

        Ok(tokens)
    }

    /// Returns true if a character can start an identifier.
    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    /// Returns true if a character can appear inside an identifier.
    fn is_identifier_part(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    /// Lexes an identifier or keyword.
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
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => TokenKind::Identifier,
        };

        Token::new(kind, lexeme, Span::new(start, self.cursor.position()))
    }

    /// Lexes a numeric literal.
    fn lex_number(&mut self) -> Result<Token, LexerError> {
        let start = self.cursor.position();

        let mut lexeme = String::new();
        let mut has_decimal = false;

        while let Some(ch) = self.cursor.current() {
            if ch == '.' {
                if has_decimal {
                    return Err(LexerError::InvalidNumber {
                        span: Span::new(start, self.cursor.position()),
                    });
                }

                has_decimal = true;
                lexeme.push(ch);
                self.cursor.advance();
                continue;
            }

            if !ch.is_ascii_digit() {
                break;
            }

            lexeme.push(ch);
            self.cursor.advance();
        }

        Ok(Token::new(
            TokenKind::Number,
            lexeme,
            Span::new(start, self.cursor.position()),
        ))
    }

    /// Lexes single-character operators.
    fn lex_operator(&mut self) -> Result<Option<Token>, LexerError> {
        let start = self.cursor.position();

        let Some(ch) = self.cursor.current() else {
            return Ok(None);
        };

        let (kind, lexeme) = match ch {
            '=' => (TokenKind::Equal, "="),
            '+' => (TokenKind::Plus, "+"),
            '-' => (TokenKind::Minus, "-"),
            '*' => (TokenKind::Star, "*"),
            '/' => (TokenKind::Slash, "/"),
            '%' => (TokenKind::Percent, "%"),
            _ => {
                return Ok(None);
            }
        };

        self.cursor.advance();

        Ok(Some(Token::new(
            kind,
            lexeme.to_string(),
            Span::new(start, self.cursor.position()),
        )))
    }

    /// Lexes delimiters.
    fn lex_delimiter(&mut self) -> Option<Token> {
        let start = self.cursor.position();

        let ch = self.cursor.current()?;

        let (kind, lexeme) = match ch {
            '(' => (TokenKind::LeftParen, "("),
            ')' => (TokenKind::RightParen, ")"),

            '{' => (TokenKind::LeftBrace, "{"),
            '}' => (TokenKind::RightBrace, "}"),

            '[' => (TokenKind::LeftBracket, "["),
            ']' => (TokenKind::RightBracket, "]"),

            ',' => (TokenKind::Comma, ","),
            '.' => (TokenKind::Dot, "."),
            ':' => (TokenKind::Colon, ":"),

            _ => {
                return None;
            }
        };

        self.cursor.advance();

        Some(Token::new(
            kind,
            lexeme.to_string(),
            Span::new(start, self.cursor.position()),
        ))
    }
}
