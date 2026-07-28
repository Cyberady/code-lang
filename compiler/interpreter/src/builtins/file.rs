use std::fs::{ self, OpenOptions };
use std::io::Write;
use std::path::Path;

use lexer::span::Span;
use parser::ast::Expression;

use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

pub fn property(property: &str, span: Span) -> Result<Value, InterpreterError> {
    Err(InterpreterError::RuntimeError {
        message: format!("Unknown file property '{}'.", property),
        span,
    })
}

pub fn call(
    interpreter: &mut Interpreter,
    method: &str,
    arguments: &[Expression],
    span: Span
) -> Result<Value, InterpreterError> {
    match method {
        "exists" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.exists() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => Ok(Value::Boolean(Path::new(&path).exists())),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.exists() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "read" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.read() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::read_to_string(&path) {
                        Ok(contents) => Ok(Value::String(contents)),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.read() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "write" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.write() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let path = interpreter.evaluate(&arguments[0])?;
            let contents = interpreter.evaluate(&arguments[1])?;

            match (path, contents) {
                (Value::String(path), Value::String(contents)) => {
                    match fs::write(&path, contents) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.write() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.write() expects the first argument to be a string.".to_string(),
                        span,
                    }),
            }
        }

        "add" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.add() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let path = interpreter.evaluate(&arguments[0])?;
            let text = interpreter.evaluate(&arguments[1])?;

            match (path, text) {
                (Value::String(path), Value::String(text)) => {
                    let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
                        Ok(file) => file,

                        Err(error) => {
                            return Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            });
                        }
                    };

                    match file.write_all(text.as_bytes()) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.add() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.add() expects the first argument to be a string.".to_string(),
                        span,
                    }),
            }
        }

        "delete" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.delete() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::remove_file(&path) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.delete() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "list" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.list() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    let mut entries = Vec::new();

                    let directory = match fs::read_dir(&path) {
                        Ok(dir) => dir,

                        Err(error) => {
                            return Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            });
                        }
                    };

                    for entry in directory {
                        let entry = match entry {
                            Ok(entry) => entry,

                            Err(error) => {
                                return Err(InterpreterError::RuntimeError {
                                    message: error.to_string(),
                                    span,
                                });
                            }
                        };

                        let name = entry.file_name();

                        entries.push(Value::String(name.to_string_lossy().into_owned()));
                    }

                    entries.sort_by(|a, b| {
                        match (a, b) {
                            (Value::String(a), Value::String(b)) => a.cmp(b),
                            _ => std::cmp::Ordering::Equal,
                        }
                    });

                    Ok(Value::Array(entries))
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.list() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "size" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.size() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::metadata(&path) {
                        Ok(metadata) => Ok(Value::Number(metadata.len() as f64)),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.size() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "isFile" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.isFile() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::metadata(&path) {
                        Ok(metadata) => Ok(Value::Boolean(metadata.is_file())),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.isFile() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "isDirectory" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.isDirectory() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::metadata(&path) {
                        Ok(metadata) => Ok(Value::Boolean(metadata.is_dir())),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.isDirectory() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "name" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.name() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    let path = Path::new(&path);

                    match path.file_name() {
                        Some(name) => { Ok(Value::String(name.to_string_lossy().into_owned())) }

                        None => Ok(Value::Null),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.name() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "extension" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.extension() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    let path = Path::new(&path);

                    match path.extension() {
                        Some(extension) => {
                            Ok(Value::String(extension.to_string_lossy().into_owned()))
                        }

                        None => Ok(Value::Null),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.extension() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "parent" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.parent() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    let path = Path::new(&path);

                    match path.parent() {
                        Some(parent) => {
                            let parent = parent.to_string_lossy().into_owned();

                            if parent.is_empty() {
                                Ok(Value::Null)
                            } else {
                                Ok(Value::String(parent))
                            }
                        }

                        None => Ok(Value::Null),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.parent() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "mkdir" => {
            if arguments.len() != 1 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.mkdir() expects exactly 1 argument.".to_string(),
                    span,
                });
            }

            let value = interpreter.evaluate(&arguments[0])?;

            match value {
                Value::String(path) => {
                    match fs::create_dir(&path) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.mkdir() expects a string.".to_string(),
                        span,
                    }),
            }
        }

        "copy" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.copy() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let source = interpreter.evaluate(&arguments[0])?;
            let destination = interpreter.evaluate(&arguments[1])?;

            match (source, destination) {
                (Value::String(source), Value::String(destination)) => {
                    match fs::copy(&source, &destination) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.copy() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.copy() expects the first argument to be a string.".to_string(),
                        span,
                    }),
            }
        }

        "move" => {
            if arguments.len() != 2 {
                return Err(InterpreterError::RuntimeError {
                    message: "file.move() expects exactly 2 arguments.".to_string(),
                    span,
                });
            }

            let source = interpreter.evaluate(&arguments[0])?;
            let destination = interpreter.evaluate(&arguments[1])?;

            match (source, destination) {
                (Value::String(source), Value::String(destination)) => {
                    match fs::rename(&source, &destination) {
                        Ok(_) => Ok(Value::Null),

                        Err(error) =>
                            Err(InterpreterError::RuntimeError {
                                message: error.to_string(),
                                span,
                            }),
                    }
                }

                (Value::String(_), _) =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.move() expects the second argument to be a string.".to_string(),
                        span,
                    }),

                _ =>
                    Err(InterpreterError::RuntimeError {
                        message: "file.move() expects the first argument to be a string.".to_string(),
                        span,
                    }),
            }
        }

        _ =>
            Err(InterpreterError::RuntimeError {
                message: format!("Unknown file method '{}'.", method),
                span,
            }),
    }
}
