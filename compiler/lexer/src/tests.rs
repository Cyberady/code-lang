use crate::{lexer::Lexer, source::SourceFile, token::TokenKind};

#[test]
fn empty_source_returns_eof() {
    let source = SourceFile::new("main.code".to_string(), String::new());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::EOF);
}
