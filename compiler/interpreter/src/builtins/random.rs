use lexer::span::Span;
use parser::ast::Expression;
use rand::Rng;

use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    match property {
        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown random property '{}'.", property),
                span,
            }),
    }
}

pub fn call(
    interpreter: &mut Interpreter,
    method: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match method {
        "int" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "random.int() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let min = interpreter.evaluate(&arguments[0])?;
            let max = interpreter.evaluate(&arguments[1])?;

            match (min, max) {
                (Value::Number(min), Value::Number(max)) => {
                    let mut rng = rand::rng();

                    let value = rng.random_range(min as i64..=max as i64);

                    Ok(Value::Number(value as f64))
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "random.int() expects two numbers.".to_string(),
                        span,
                    }),
            }
        }

        "float" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "random.float() expects no arguments.".to_string(),
                    span,
                });
            }

            let mut rng = rand::rng();

            Ok(Value::Number(rng.random::<f64>()))
        }

        "bool" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "random.bool() expects no arguments.".to_string(),
                    span,
                });
            }

            let mut rng = rand::rng();

            Ok(Value::Boolean(rng.random()))
        }

        "choice" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "random.choice() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::Array(array) => {
                    if array.is_empty() {
                        return Err(InterpreterError::RuntimeError {
                            message: "Cannot choose from an empty array.".to_string(),
                            span,
                        });
                    }

                    let mut rng = rand::rng();
                    let index = rng.random_range(0..array.len());

                    Ok(array[index].clone())
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "random.choice() expects an array.".to_string(),
                        span,
                    }),
            }
        }
        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown random method '{}'.", method),
                span,
            }),
    }
}
