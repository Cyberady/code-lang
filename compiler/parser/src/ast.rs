//! Abstract Syntax Tree definitions for the Code programming language.

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
    },

    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    Expression(Expression),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),

    NumberLiteral(String),

    StringLiteral(String),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    EqualEqual,
    BangEqual,

    Less,
    LessEqual,

    Greater,
    GreaterEqual,
}
