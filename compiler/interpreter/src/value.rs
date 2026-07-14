//! Runtime values.

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),

    Boolean(bool),

    Null,
}
