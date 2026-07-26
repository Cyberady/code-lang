use parser::ast::Expression;

use lexer::span::Span;

use crate::{error::InterpreterError, interpreter::Interpreter, value::Value};

pub fn call(
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
