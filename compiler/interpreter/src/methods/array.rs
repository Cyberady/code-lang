use crate::{error::InterpreterError, interpreter::Interpreter, value::Value};

use parser::ast::Expression;

use lexer::span::Span;

pub fn call(
    interpreter: &mut Interpreter,
    name: &str,
    array: &mut Vec<Value>,
    property: &str,
    arguments: &[Expression],
    span: Span,
) -> Result<Value, InterpreterError> {
    match property {
        "add" => {
            match arguments.len() {
                1 => {
                    let value = interpreter.evaluate(&arguments[0])?;

                    array.push(value);
                }

                2 => {
                    let index = interpreter.evaluate(&arguments[0])?;

                    let index = match index {
                        Value::Number(n) => n as usize,

                        _ => {
                            return Err(InterpreterError::RuntimeError {
                                message: "Insert index must be a number.".to_string(),
                                span,
                            });
                        }
                    };

                    if index > array.len() {
                        return Err(InterpreterError::RuntimeError {
                            message: "Array index out of bounds.".to_string(),
                            span,
                        });
                    }

                    let value = interpreter.evaluate(&arguments[1])?;

                    array.insert(index, value);
                }

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.add expects 1 or 2 arguments.".to_string(),
                        span,
                    });
                }
            }

            interpreter
                .environment
                .borrow_mut()
                .assign(name.to_string(), Value::Array(array.clone()))?;

            Ok(Value::Null)
        }

        "remove" => {
            match arguments.len() {
                0 => {
                    if array.is_empty() {
                        return Err(InterpreterError::RuntimeError {
                            message: "Array is empty.".to_string(),
                            span,
                        });
                    }

                    array.remove(array.len() - 1);
                }

                1 => {
                    let index = interpreter.evaluate(&arguments[0])?;

                    let index = match index {
                        Value::Number(n) => n as usize,

                        _ => {
                            return Err(InterpreterError::RuntimeError {
                                message: "Remove index must be a number.".to_string(),
                                span,
                            });
                        }
                    };

                    if index >= array.len() {
                        return Err(InterpreterError::RuntimeError {
                            message: "Array index must be a number.".to_string(),
                            span,
                        });
                    }

                    array.remove(index);
                }

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.remove expects 0 or 1 arguments.".to_string(),
                        span,
                    });
                }
            }

            interpreter
                .environment
                .borrow_mut()
                .assign(name.to_string(), Value::Array(array.clone()))?;

            Ok(Value::Null)
        }

        "contains" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "array.contains expects 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            Ok(Value::Boolean(array.contains(&value)))
        }

        "clear" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.clear expects 0 arguments.".to_string(),
                    span,
                });
            }

            array.clear();

            interpreter
                .environment
                .borrow_mut()
                .assign(name.to_string(), Value::Array(array.clone()))?;

            Ok(Value::Null)
        }

        "first" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.first expects 0 arguments.".to_string(),
                    span,
                });
            }

            match array.first() {
                Some(value) => Ok(value.clone()),

                None => Err(InterpreterError::RuntimeError {
                    message: "Array is empty.".to_string(),
                    span,
                }),
            }
        }

        "last" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.last expects 0 arguments.".to_string(),
                    span,
                });
            }

            match array.last() {
                Some(value) => Ok(value.clone()),

                None => Err(InterpreterError::RuntimeError {
                    message: "Array is empty.".to_string(),
                    span,
                }),
            }
        }

        "isEmpty" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.isEmpty expects 0 arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Boolean(array.is_empty()))
        }

        "reverse" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.reverse expects 0 arguments.".to_string(),
                    span,
                });
            }

            array.reverse();

            interpreter
                .environment
                .borrow_mut()
                .assign(name.to_string(), Value::Array(array.clone()))?;

            Ok(Value::Null)
        }

        "sort" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "array.sort expects 0 arguments.".to_string(),
                    span,
                });
            }

            if !array.iter().all(|value| matches!(value, Value::Number(_))) {
                return Err(InterpreterError::RuntimeError {
                    message: "array.sort only supports numeric arrays.".to_string(),
                    span,
                });
            }

            array.sort_by(|a, b| {
                let a = match a {
                    Value::Number(n) => *n,
                    _ => unreachable!(),
                };

                let b = match b {
                    Value::Number(n) => *n,
                    _ => unreachable!(),
                };

                a.partial_cmp(&b).unwrap()
            });

            interpreter
                .environment
                .borrow_mut()
                .assign(name.to_string(), Value::Array(array.clone()))?;

            Ok(Value::Null)
        }

        _ => Err(InterpreterError::RuntimeError {
            message: format!("Unknown array method '{}'.", property),
            span,
        }),
    }
}
