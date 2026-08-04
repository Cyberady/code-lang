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
            message: "boolean() expects exactly 1 argument.".to_string(),
            span: Span::default(),
        });
    }

    let value = interpreter.evaluate(&arguments[0])?;

    let result = match value {
        Value::Boolean(value) => value,

        Value::Null => false,

        Value::Number(number) => number != 0.0,

        Value::String(text) => !text.is_empty(),

        Value::Array(values) => !values.is_empty(),

        Value::Object(properties) => !properties.is_empty(),
    };

    Ok(Value::Boolean(result))
}
