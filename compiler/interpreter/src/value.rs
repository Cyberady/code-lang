#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),

    String(String),

    Boolean(bool),

    Array(Vec<Value>),

    Object(std::collections::HashMap<String, Value>),

    Null,
}
