use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

use parser::ast::Expression;
use lexer::span::Span;

pub fn call(
    interpreter: &mut Interpreter,
    text: String,
    property: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match property {
        "upper" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "string.upper expects 0 arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::String(text.to_uppercase()))
        }

        "lower" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "string.lower expects 0 arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::String(text.to_lowercase()))
        }

        "trim" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "string.trim expects 0 arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::String(text.trim().to_string()))
        }

        "contains" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "string.contains expects 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            let search = match value {
                Value::String(text) => text,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.contains expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            Ok(Value::Boolean(text.contains(&search)))
        }

        "startsWith" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "string.startsWith expects 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            let prefix = match value {
                Value::String(text) => text,
                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.startsWith expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            Ok(Value::Boolean(text.starts_with(&prefix)))
        }

        "endsWith" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "string.endsWith expects 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            let suffix = match value {
                Value::String(text) => text,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.endsWith expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            Ok(Value::Boolean(text.ends_with(&suffix)))
        }

        "replace" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "string.replace expects 2 arguments.".to_string(),
                    span,
                });
            }

            let old = interpreter.evaluate(&arguments[0])?;
            let new = interpreter.evaluate(&arguments[1])?;

            let old = match old {
                Value::String(text) => text,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.replace expects string arguments.".to_string(),
                        span,
                    });
                }
            };

            let new = match new {
                Value::String(text) => text,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.replace expects string arguments.".to_string(),
                        span,
                    });
                }
            };

            Ok(Value::String(text.replace(&old, &new)))
        }

        "split" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "string.split expects 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            let separator = match value {
                Value::String(text) => text,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "string.split expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            let values = text
                .split(&separator)
                .map(|part| Value::String(part.to_string()))
                .collect();

            Ok(Value::Array(values))
        }

        "reverse" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "string.reverse expects 0 arguments.".to_string(),
                    span,
                });
            }

            let reversed: String = text.chars().rev().collect();

            Ok(Value::String(reversed))
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown string method '{}'.", property),
                span,
            }),
    }
}
