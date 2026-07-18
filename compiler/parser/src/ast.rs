//! Abstract Syntax Tree definitions for the Code programming language.

use lexer::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
        span: Span,
    },

    Assignment {
        name: String,
        value: Expression,
        span: Span,
    },

    IndexAssignment {
        object: Expression,
        index: Expression,
        value: Expression,
        span: Span,
    },

    PropertyAssignment {
        object: Expression,
        property: String,
        value: Expression,
        span: Span,
    },

    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
        span: Span,
    },

    Return {
        value: Expression,
        span: Span,
    },

    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
        span: Span,
    },

    Expression(Expression),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier {
        name: String,
        span: Span,
    },

    NumberLiteral {
        value: String,
        span: Span,
    },

    ArrayLiteral {
        elements: Vec<Expression>,
        span: Span,
    },

    ObjectLiteral {
        properties: Vec<(String, Expression)>,
        span: Span,
    },

    StringLiteral {
        value: String,
        span: Span,
    },

    BooleanLiteral {
        value: bool,
        span: Span,
    },

    NullLiteral {
        span: Span,
    },

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },

    Index {
        object: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },

    Property {
        object: Box<Expression>,
        property: String,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Identifier { span, .. } => span,

            Expression::NumberLiteral { span, .. } => span,

            Expression::ArrayLiteral { span, .. } => span,

            Expression::ObjectLiteral { span, .. } => span,

            Expression::StringLiteral { span, .. } => span,

            Expression::BooleanLiteral { span, .. } => span,

            Expression::NullLiteral { span } => span,

            Expression::Binary { span, .. } => span,

            Expression::Call { span, .. } => span,

            Expression::Index { span, .. } => span,

            Expression::Property { span, .. } => span,
        }
    }
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
