//! Interpreter errors.

use std::fmt;

use crate::value::Value;
use lexer::span::Span;

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable {
        name: String,
        span: Span,
    },

    CannotAssignConstant {
        name: String,
        span: Span,
    },

    InvalidBinaryOperation {
        operator: String,
        span: Span,
    },

    RuntimeError {
        message: String,
        span: Span,
    },

    NotCallable {
        name: String,
        span: Span,
    },

    InvalidArgumentCount {
        expected: usize,
        found: usize,
        span: Span,
    },

    Return(Value),

    Break,

    Continue,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable { name, .. } => {
                write!(f, "[E1001]: Undefined variable '{}'", name)
            }

            InterpreterError::CannotAssignConstant { name, .. } => {
                write!(f, "[E1002]: Cannot assign to constant '{}'", name)
            }

            InterpreterError::InvalidBinaryOperation { operator, .. } => {
                write!(f, "[E1003]: Invalid use of operator '{}'", operator)
            }

            InterpreterError::InvalidArgumentCount { expected, found, .. } => {
                write!(
                    f,
                    "[E1006]: Function expected {} argument(s) but received {}.",
                    expected,
                    found
                )
            }

            InterpreterError::RuntimeError { message, .. } => { write!(f, "[E1004]: {message}") }

            InterpreterError::Return(_) => { write!(f, "Internal interpreter return") }

            InterpreterError::Break => { write!(f, "Internal interpreter break") }

            InterpreterError::Continue => { write!(f, "Internal interpreter continue") }

            InterpreterError::NotCallable { name, .. } => {
                write!(f, "[E1005]: Value '{}' is not callable", name)
            }
        }
    }
}

impl std::error::Error for InterpreterError {}
