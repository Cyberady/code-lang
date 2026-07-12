//! Token definitions for the Code programming language.
//!
//! A token is the smallest meaningful unit produced by the lexer.
//! The parser consumes tokens to build the syntax tree.

use crate::span::Span;

/// Represents every token recognized by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // -------------------------
    // Literals
    // -------------------------
    Identifier,
    Number,
    String,

    // -------------------------
    // Keywords
    // -------------------------
    Const,
    Func,

    If,
    Else,

    For,
    While,

    Return,
    Break,
    Continue,

    True,
    False,
    Null,

    // -------------------------
    // Operators
    // -------------------------
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Equal,
    EqualEqual,

    Bang,
    BangEqual,

    Greater,
    GreaterEqual,

    Less,
    LessEqual,

    // -------------------------
    // Delimiters
    // -------------------------
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    Comma,
    Dot,
    Colon,

    // -------------------------
    // End of File
    // -------------------------
    EOF,
}

/// Represents a single token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of token.
    pub kind: TokenKind,

    /// The exact text from the source code.
    pub lexeme: String,

    /// The position of the token inside the source file.
    pub span: Span,
}

impl Token {
    /// Creates a new token.
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self {
            kind,
            lexeme,
            span,
        }
    }
}
