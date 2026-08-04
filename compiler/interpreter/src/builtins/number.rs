use parser::ast::Expression;

use lexer::span::Span;

use crate::{
    error::InterpreterError,
    interpreter::Interpreter,
    value::Value,
};

pub fn call(
    interpreter: &mut Interpreter,
    arguments: &[Expression],
) -> Result<Value, InterpreterError> {
    if arguments.len() != 1 {
        return Err(InterpreterError::RuntimeError {
            message: "number() expects exactly 1 argument.".to_string(),
            span: Span::default(),
        });
    }

    let value = interpreter.evaluate(&arguments[0])?;

    match value {
        Value::Number(number) => Ok(Value::Number(number)),

        Value::Boolean(true) => Ok(Value::Number(1.0)),

        Value::Boolean(false) => Ok(Value::Number(0.0)),

        Value::String(text) => {
            match text.trim().parse::<f64>() {
                Ok(number) => Ok(Value::Number(number)),

                Err(_) => Err(InterpreterError::RuntimeError {
                    message: format!("Cannot convert '{}' to Number.", text),
                    span: Span::default(),
                }),
            }
        }

        _ => Err(InterpreterError::RuntimeError {
            message: "Value cannot be converted to Number.".to_string(),
            span: Span::default(),
        }),
    }
}
