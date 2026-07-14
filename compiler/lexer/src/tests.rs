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

#[test]
fn lexes_integer() {
    let source = SourceFile::new("main.code".to_string(), "123".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].lexeme, "123");
}

#[test]
fn lexes_decimal() {
    let source = SourceFile::new("main.code".to_string(), "3.14159".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].lexeme, "3.14159");
}

#[test]
fn lexes_const_number() {
    let source = SourceFile::new("main.code".to_string(), "const PI 3.14".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Const);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].kind, TokenKind::Number);
}

#[test]
fn lexes_equal() {
    let source = SourceFile::new("main.code".to_string(), "=".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Equal);
}

#[test]
fn lexes_plus() {
    let source = SourceFile::new("main.code".to_string(), "+".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Plus);
}

#[test]
fn lexes_variable_declaration() {
    let source = SourceFile::new("main.code".to_string(), "const PI = 3.14".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 5);

    assert_eq!(tokens[0].kind, TokenKind::Const);

    assert_eq!(tokens[1].kind, TokenKind::Identifier);

    assert_eq!(tokens[2].kind, TokenKind::Equal);

    assert_eq!(tokens[3].kind, TokenKind::Number);

    assert_eq!(tokens[4].kind, TokenKind::EOF);
}

#[test]
fn lexes_parentheses() {
    let source = SourceFile::new("main.code".to_string(), "()".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftParen);
    assert_eq!(tokens[1].kind, TokenKind::RightParen);
}

#[test]
fn lexes_braces() {
    let source = SourceFile::new("main.code".to_string(), "{}".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[1].kind, TokenKind::RightBrace);
}

#[test]
fn lexes_function_declaration() {
    let source = SourceFile::new("main.code".to_string(), "func add() {}".to_string());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Func);

    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "add");

    assert_eq!(tokens[2].kind, TokenKind::LeftParen);
    assert_eq!(tokens[3].kind, TokenKind::RightParen);

    assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[5].kind, TokenKind::RightBrace);

    assert_eq!(tokens[6].kind, TokenKind::EOF);
}
