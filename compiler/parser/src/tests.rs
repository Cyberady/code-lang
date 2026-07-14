use lexer::{lexer::Lexer, source::SourceFile};

use crate::{
    ast::{BinaryOperator, Expression, Statement},
    parser::Parser,
};
#[test]
fn parses_binary_expression() {
    let source = SourceFile::new(
        "main.code".to_string(),
        "const result = 10 + 20".to_string(),
    );

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);

    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::VariableDeclaration { name, value } => {
            assert_eq!(name, "result");

            match value {
                Expression::Binary {
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(*operator, BinaryOperator::Plus);

                    assert_eq!(**left, Expression::NumberLiteral("10".into()));
                    assert_eq!(**right, Expression::NumberLiteral("20".into()));
                }

                _ => panic!("Expected binary expression"),
            }
        }

        Statement::If { .. } => {
            panic!("Expected variable declaration");
        }

        Statement::Expression(_) => {
            panic!("Expected variable declaration");
        }
    }
}
