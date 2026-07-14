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

            if ch == '"' {
                tokens.push(self.lex_string()?);
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

    fn lex_string(&mut self) -> Result<Token, LexerError> {
        let start = self.cursor.position();

        // Skip opening quote
        self.cursor.advance();

        let mut value = String::new();

        while let Some(ch) = self.cursor.current() {
            if ch == '"' {
                break;
            }

            value.push(ch);
            self.cursor.advance();
        }

        // Reached EOF before closing quote
        if self.cursor.current().is_none() {
            return Err(LexerError::UnterminatedString {
                span: Span::new(start, self.cursor.position()),
            });
        }

        // Skip closing quote
        self.cursor.advance();

        Ok(Token::new(
            TokenKind::String,
            value,
            Span::new(start, self.cursor.position()),
        ))
    }

    /// Lexes single-character operators.
    fn lex_operator(&mut self) -> Result<Option<Token>, LexerError> {
        let start = self.cursor.position();

        let Some(ch) = self.cursor.current() else {
            return Ok(None);
        };

        let token = match ch {
            '=' => {
                self.cursor.advance();

                if self.cursor.current() == Some('=') {
                    self.cursor.advance();

                    Token::new(
                        TokenKind::EqualEqual,
                        "==".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                } else {
                    Token::new(
                        TokenKind::Equal,
                        "=".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                }
            }

            '!' => {
                self.cursor.advance();

                if self.cursor.current() == Some('=') {
                    self.cursor.advance();

                    Token::new(
                        TokenKind::BangEqual,
                        "!=".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                } else {
                    Token::new(
                        TokenKind::Bang,
                        "!".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                }
            }

            '<' => {
                self.cursor.advance();

                if self.cursor.current() == Some('=') {
                    self.cursor.advance();

                    Token::new(
                        TokenKind::LessEqual,
                        "<=".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                } else {
                    Token::new(
                        TokenKind::Less,
                        "<".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                }
            }

            '>' => {
                self.cursor.advance();

                if self.cursor.current() == Some('=') {
                    self.cursor.advance();

                    Token::new(
                        TokenKind::GreaterEqual,
                        ">=".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                } else {
                    Token::new(
                        TokenKind::Greater,
                        ">".to_string(),
                        Span::new(start, self.cursor.position()),
                    )
                }
            }

            '+' => {
                self.cursor.advance();

                Token::new(
                    TokenKind::Plus,
                    "+".to_string(),
                    Span::new(start, self.cursor.position()),
                )
            }

            '-' => {
                self.cursor.advance();

                Token::new(
                    TokenKind::Minus,
                    "-".to_string(),
                    Span::new(start, self.cursor.position()),
                )
            }

            '*' => {
                self.cursor.advance();

                Token::new(
                    TokenKind::Star,
                    "*".to_string(),
                    Span::new(start, self.cursor.position()),
                )
            }

            '/' => {
                self.cursor.advance();

                Token::new(
                    TokenKind::Slash,
                    "/".to_string(),
                    Span::new(start, self.cursor.position()),
                )
            }

            '%' => {
                self.cursor.advance();

                Token::new(
                    TokenKind::Percent,
                    "%".to_string(),
                    Span::new(start, self.cursor.position()),
                )
            }

            _ => {
                return Ok(None);
            }
        };

        Ok(Some(token))
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
