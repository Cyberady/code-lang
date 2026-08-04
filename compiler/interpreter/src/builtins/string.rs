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
            message: "string() expects exactly 1 argument.".to_string(),
            span: Span::default(),
        });
    }

    let value = interpreter.evaluate(&arguments[0])?;

    Ok(Value::String(value.to_string()))
}
