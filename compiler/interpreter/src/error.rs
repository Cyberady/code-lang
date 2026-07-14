//! Interpreter errors.

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable,

    InvalidBinaryOperation,
}
