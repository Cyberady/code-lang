#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),

    String(String),

    Boolean(bool),

    Array(Vec<Value>),

    Object(std::collections::HashMap<String, Value>),

    Null,
}

use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),

            Value::String(s) => write!(f, "{s}"),

            Value::Boolean(b) => write!(f, "{b}"),

            Value::Null => write!(f, "null"),

            Value::Array(values) => {
                write!(f, "[")?;

                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{value}")?;
                }

                write!(f, "]")
            }

            Value::Object(map) => {
                write!(f, "{{")?;

                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{key}: {value}")?;
                }

                write!(f, "}}")
            }
        }
    }
}
