use std::fs;
use std::path::Path;

use lexer::span::Span;
use parser::ast::Expression;

use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    Err(InterpreterError::RuntimeError {
        message: format!("Unknown file property '{}'.", property),
        span,
    })
}

pub fn call(
    interpreter: &mut Interpreter,
    method: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match method {
        "exists" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.exists() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => Ok(Value::Boolean(Path::new(&path).exists())),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.exists() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "read" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.read() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::read_to_string(&path) {
                        Ok(contents) => Ok(Value::String(contents)),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.read() expects a string.".to_string(),
                        span,
                    }),
            }
        }
        
        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown file method '{}'.", method),
                span,
            }),
    }
}
