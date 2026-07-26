use chrono::{ Datelike, Local, Timelike, SecondsFormat };
use lexer::span::Span;
use parser::ast::Expression;
use std::thread;
use std::time::Duration;

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

        "weekday" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.weekday() expects no arguments.".to_string(),
                    span,
                });
            }

            let weekday = Local::now().weekday().to_string();

            Ok(Value::String(weekday))
        }

        "weekdayNumber" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.weekdayNumber() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().weekday().number_from_monday() as f64))
        }

        "dayOfYear" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.dayOfYear() expects no arguments.".to_string(),
                    span,
                });
            }

            Ok(Value::Number(Local::now().ordinal() as f64))
        }

        "isLeapYear" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.isLeapYear() expects no arguments.".to_string(),
                    span,
                });
            }

            let year = Local::now().year();

            let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;

            Ok(Value::Boolean(leap))
        }

        "daysInYear" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.daysInYear() expects no arguments.".to_string(),
                    span,
                });
            }

            let year = Local::now().year();

            let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;

            Ok(Value::Number(if leap { 366.0 } else { 365.0 }))
        }

        "timezone" => {
            if !arguments.is_empty() {
                return Err(InterpreterError::RuntimeError {
                    message: "time.timezone() expects no arguments.".to_string(),
                    span,
                });
            }

            let timezone = Local::now().format("%:z").to_string();

            Ok(Value::String(timezone))
        }

        "sleep" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "time.sleep() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = _interpreter.evaluate(&arguments[0])?;

            match value {
                Value::Number(ms) => {
                    if ms < 0.0 {
                        return Err(InterpreterError::RuntimeError {
                            message: "time.sleep() expects a non-negative number.".to_string(),
                            span,
                        });
                    }

                    thread::sleep(Duration::from_millis(ms as u64));
                    Ok(Value::Null)
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "time.sleep() expects a number.".to_string(),
                        span,
                    }),
            }
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown time method '{}'.", method),
                span,
            }),
    }
}
