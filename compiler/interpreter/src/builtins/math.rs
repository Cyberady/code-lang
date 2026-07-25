use lexer::span::Span;

use crate::{ error::InterpreterError, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    match property {
        "pi" => Ok(Value::Number(std::f64::consts::PI)),

        "e" => Ok(Value::Number(std::f64::consts::E)),

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

        "abs" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "math.abs() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::Number(number) => Ok(Value::Number(number.abs())),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "math.abs() expects a number.".to_string(),
                        span,
                    }),
            }
        }

        "pow" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "math.pow() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let base = interpreter.evaluate(&arguments[0])?;
            let exponent = interpreter.evaluate(&arguments[1])?;

            match (base, exponent) {
                (Value::Number(base), Value::Number(exponent)) => {
                    Ok(Value::Number(base.powf(exponent)))
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "math.pow() expects two numbers.".to_string(),
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
