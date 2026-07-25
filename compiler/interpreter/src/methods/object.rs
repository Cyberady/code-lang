use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

use parser::ast::Expression;
use lexer::span::Span;

pub fn call(
    interpreter: &mut Interpreter,
    object: std::collections::HashMap<String, Value>,
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

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown object method '{}'.", property),
                span,
            }),
    }
}
