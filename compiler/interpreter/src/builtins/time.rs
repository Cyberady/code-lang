use chrono::{ Datelike, Local, Timelike, SecondsFormat };
use lexer::span::Span;
use parser::ast::Expression;

use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    match property {
        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown time property '{}'.", property),
                span,
            }),
    }
}

pub fn call(
    _interpreter: &mut Interpreter,
    method: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match method {
        "now" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.now() expects no arguments.".to_string(),
                    span,
                });
            }

            let timestamp = Local::now().timestamp();

            Ok(Value::Number(timestamp as f64))
        }

        "nowMs" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.nowMs() expects no arguments.".to_string(),
                    span,
                });
            }

            let timestamp = Local::now().timestamp_millis();

            Ok(Value::Number(timestamp as f64))
        }

        "year" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.year() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().year() as f64))
        }

        "month" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.month() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().month() as f64))
        }

        "day" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.day() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().day() as f64))
        }

        "hour" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.hour() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().hour() as f64))
        }

        "minute" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.minute() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().minute() as f64))
        }

        "second" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.second() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().second() as f64))
        }

        "date" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.date() expects no arguments.".to_string(),
                    span,
                });
            }

            let date = Local::now().format("%Y-%m-%d").to_string();

            Ok(Value::String(date))
        }

        "clock" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.clock() expects no arguments.".to_string(),
                    span,
                });
            }

            let clock = Local::now().format("%H:%M:%S").to_string();

            Ok(Value::String(clock))
        }

        "datetime" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.datetime() expects no arguments.".to_string(),
                    span,
                });
            }

            let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            Ok(Value::String(datetime))
        }

        "iso" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.iso() expects no arguments.".to_string(),
                    span,
                });
            }

            let iso = Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);

            Ok(Value::String(iso))
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown time method '{}'.", method),
                span,
            }),
    }
}
