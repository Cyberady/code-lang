//! Interpreter errors.

use std::fmt;

use crate::value::Value;
use lexer::span::Span;

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable { name: String, span: Span },

    CannotAssignConstant { name: String, span: Span },

    InvalidBinaryOperation { operator: String, span: Span },

    RuntimeError { message: String, span: Span },

    Return(Value),

    Break,

    Continue,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable { name, .. } => {
                write!(f, "Undefined variable '{}'", name)
            }

            InterpreterError::CannotAssignConstant { name, .. } => {
                write!(f, "Cannot assign to constant '{}'", name)
            }

            InterpreterError::InvalidBinaryOperation { operator, .. } => {
                write!(f, "Invalid use of operator '{}'", operator)
            }

            InterpreterError::RuntimeError { message, .. } => {
                write!(f, "{message}")
            }

            InterpreterError::Return(_) => {
                write!(f, "Internal interpreter return")
            }

            InterpreterError::Break => {
                write!(f, "Internal interpreter break")
            }

            InterpreterError::Continue => {
                write!(f, "Internal interpreter continue")
            }
        }
    }
}

impl std::error::Error for InterpreterError {}
