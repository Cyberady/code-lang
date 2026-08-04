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
            message: "type() expects exactly 1 argument.".to_string(),
            span: Span::default(),
        });
    }

    let value = interpreter.evaluate(&arguments[0])?;

    let type_name = match value {
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Boolean(_) => "Boolean",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
        Value::Null => "Null",
    };

    Ok(Value::String(type_name.to_string()))
}
