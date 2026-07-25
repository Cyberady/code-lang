use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

use parser::ast::Expression;
use lexer::span::Span;

pub fn call(
    _interpreter: &mut Interpreter,
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

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown string method '{}'.", property),
                span,
            }),
    }
}
