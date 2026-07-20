use parser::ast::Expression;

use lexer::span::Span;

use crate::{error::InterpreterError, interpreter::Interpreter, value::Value};

pub fn builtin_print(
    interpreter: &mut Interpreter,
    arguments: &[Expression],
) -> Result<Value, InterpreterError> {
    if arguments.len() != 1 {
        return Err(InterpreterError::InvalidBinaryOperation {
            operator: "?".to_string(),
            span: Span::default(),
        });
    }

    let value = interpreter.evaluate(&arguments[0])?;

    match &value {
        Value::Number(number) => println!("{number}"),

        Value::String(text) => println!("{text}"),

        Value::Boolean(boolean) => println!("{boolean}"),

        Value::Null => println!("null"),

        Value::Array(values) => {
            print!("[");

            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    print!(", ");
                }

                match value {
                    Value::Number(n) => print!("{n}"),
                    Value::String(s) => print!("{s}"),
                    Value::Boolean(b) => print!("{b}"),
                    Value::Null => print!("null"),
                    Value::Array(_) => print!("[...]"),
                    Value::Object(_) => print!("{{...}}"),
                }
            }

            println!("]");
        }

        Value::Object(properties) => {
            print!("{{");

            for (index, (key, value)) in properties.iter().enumerate() {
                if index > 0 {
                    print!(", ");
                }

                print!("{key}: ");

                match value {
                    Value::Number(n) => print!("{n}"),
                    Value::String(s) => print!("{s}"),
                    Value::Boolean(b) => print!("{b}"),
                    Value::Null => print!("null"),
                    Value::Array(_) => print!("[...]"),
                    Value::Object(_) => print!("{{...}}"),
                }
            }

            println!("}}");
        }
    }

    Ok(Value::Null)
}

pub fn builtin_range(
    interpreter: &mut Interpreter,
    arguments: &[Expression],
) -> Result<Value, InterpreterError> {
    // Helper function
    fn as_number(value: &Value) -> Option<f64> {
        match value {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    // Evaluate arguments
    let mut values = Vec::new();

    for argument in arguments {
        values.push(interpreter.evaluate(argument)?);
    }

    // Validate argument count
    if values.is_empty() || values.len() > 3 {
        return Err(InterpreterError::RuntimeError {
            message: "range() expects 1 to 3 arguments.".to_string(),
            span: Span::default(),
        });
    }

    // Extract start, end and step
    let (start, end, step) = match values.len() {
        1 => (
            0.0,
            as_number(&values[0]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
            1.0,
        ),

        2 => (
            as_number(&values[0]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
            as_number(&values[1]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
            1.0,
        ),

        3 => (
            as_number(&values[0]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
            as_number(&values[1]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
            as_number(&values[2]).ok_or(InterpreterError::RuntimeError {
                message: "range() arguments must be numbers.".to_string(),
                span: Span::default(),
            })?,
        ),

        _ => unreachable!(),
    };

    if step == 0.0 {
        return Err(InterpreterError::RuntimeError {
            message: "range() step cannot be zero.".to_string(),
            span: Span::default(),
        });
    }

    let mut result = Vec::new();

    let mut current = start;

    if step > 0.0 {
        while current < end {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > end {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Array(result))
}
