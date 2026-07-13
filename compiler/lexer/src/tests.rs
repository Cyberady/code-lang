use crate::{lexer::Lexer, source::SourceFile, token::TokenKind};

#[test]
fn empty_source_returns_eof() {
    let source = SourceFile::new("main.code".to_string(), String::new());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::EOF);
}

#[test]
fn lexes_identifier() {
    let source = SourceFile::new("main.code".to_string(), "hello".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "hello");
}

#[test]
fn lexes_keyword_const() {
    let source = SourceFile::new("main.code".to_string(), "const".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Const);
}

#[test]
fn lexes_const_identifier() {
    let source = SourceFile::new("main.code".to_string(), "const PI".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0].kind, TokenKind::Const);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "PI");
    assert_eq!(tokens[2].kind, TokenKind::EOF);
}
