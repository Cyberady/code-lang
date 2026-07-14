//! Interpreter errors.

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable,
    InvalidBinaryOperation,
    CannotAssignConstant,
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::UndefinedVariable => {
                write!(f, "Undefined variable.")
            }

            InterpreterError::InvalidBinaryOperation => {
                write!(f, "Invalid binary operation.")
            }

            InterpreterError::CannotAssignConstant => {
                write!(f, "Cannot modify constant.")
            }
        }
    }
}

impl std::error::Error for InterpreterError {}
