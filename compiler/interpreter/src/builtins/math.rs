use lexer::span::Span;

use crate::{ error::InterpreterError, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    match property {
        "pi" => Ok(Value::Number(std::f64::consts::PI)),

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown math property '{}'.", property),
                span,
            }),
    }
}

use parser::ast::Expression;

use crate::interpreter::Interpreter;

pub fn call(
    interpreter: &mut Interpreter,
    method: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match method {
        "sqrt" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "math.sqrt() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::Number(number) => { Ok(Value::Number(number.sqrt())) }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "math.sqrt() expects a number.".to_string(),
                        span,
                    }),
            }
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown math method '{}'.", method),
                span,
            }),
    }
}
