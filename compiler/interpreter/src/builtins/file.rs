use std::fs::{ self, OpenOptions };
use std::io::Write;
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

        "write" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.write() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let path = interpreter.evaluate(&arguments[0])?;
            let contents = interpreter.evaluate(&arguments[1])?;

            match (path, contents) {
                (Value::String(path), Value::String(contents)) => {
                    match fs::write(&path, contents) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.write() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.write() expects the first argument to be a string.".to_string(),
                        span,
                    }),
            }
        }

        "add" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.add() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let path = interpreter.evaluate(&arguments[0])?;
            let text = interpreter.evaluate(&arguments[1])?;

            match (path, text) {
                (Value::String(path), Value::String(text)) => {
                    let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
                        Ok(file) => file,

                        Err(error) => {
                            return Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            });
                        }
                    };

                    match file.write_all(text.as_bytes()) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.add() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.add() expects the first argument to be a string.".to_string(),
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
