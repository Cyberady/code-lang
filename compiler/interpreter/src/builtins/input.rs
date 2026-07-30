use std::io::{self, Write};

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

    if arguments.len() > 1 {
        return Err(InterpreterError::RuntimeError {
            message: "input() expects 0 or 1 arguments.".to_string(),
            span: Span::default(),
        });
    }

    if arguments.len() == 1 {
        let value = interpreter.evaluate(&arguments[0])?;

        match value {
            Value::String(text) => {
                print!("{text}");
            }

            _ => {
                return Err(InterpreterError::RuntimeError {
                    message: "input() prompt must be a string.".to_string(),
                    span: Span::default(),
                });
            }
        }

        io::stdout().flush().unwrap();
    }

    let mut text = String::new();

    io::stdin().read_line(&mut text).unwrap();

    if text.ends_with('\n') {
        text.pop();

        if text.ends_with('\r') {
            text.pop();
        }
    }

    Ok(Value::String(text))
}
