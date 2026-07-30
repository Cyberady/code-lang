use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

use lexer::span::Span;
use parser::ast::Expression;

pub fn call(
    interpreter: &mut Interpreter,
    name: &str,
    object: &mut std::collections::HashMap<String, Value>,
    property: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match property {
        "has" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "object.has expects 1 argument.".to_string(),
                    span,
                });
            }

            let key = interpreter.evaluate(&arguments[0])?;

            let key = match key {
                Value::String(key) => key,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "object.has expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            Ok(Value::Boolean(object.contains_key(&key)))
        }

        "keys" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "object.keys expects 0 arguments.".to_string(),
                    span,
                });
            }

            let keys = object
                .keys()
                .map(|key| Value::String(key.clone()))
                .collect();

            Ok(Value::Array(keys))
        }

        "values" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "object.values expects 0 arguments.".to_string(),
                    span,
                });
            }

            let values = object.values().cloned().collect();

            Ok(Value::Array(values))
        }

        "remove" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "object.remove expects 1 argument.".to_string(),
                    span,
                });
            }

            let key = interpreter.evaluate(&arguments[0])?;

            let key = match key {
                Value::String(key) => key,

                _ => {
                    return Err(InterpreterError::RuntimeError {
                        message: "object.remove expects a string argument.".to_string(),
                        span,
                    });
                }
            };

            if object.remove(&key).is_none() {
                return Err(InterpreterError::RuntimeError {
                    message: format!("Undefined property '{}'.", key),
                    span,
                });
            }

            interpreter.environment
                .borrow_mut()
                .assign(name.to_string(), Value::Object(object.clone()), Span::default())?;

            Ok(Value::Null)
        }

        "clear" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "object.clear expects 0 arguments.".to_string(),
                    span,
                });
            }

            object.clear();

            interpreter.environment
                .borrow_mut()
                .assign(name.to_string(), Value::Object(object.clone()), Span::default())?;

            Ok(Value::Null)
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown object method '{}'.", property),
                span,
            }),
    }
}
