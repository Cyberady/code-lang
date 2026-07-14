use lexer::{lexer::Lexer, source::SourceFile};

use parser::parser::Parser;

use crate::{interpreter::Interpreter, value::Value};

#[test]
fn executes_variable_declaration() {
    let source = SourceFile::new("main.code".into(), "const result = 10 + 20".into());

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();

    interpreter.execute(&program).unwrap();

    assert_eq!(
        interpreter.environment().get("result"),
        Some(&Value::Number(30.0))
    );
}
