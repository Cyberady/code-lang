//! Interpreter for the Code programming language.

pub mod builtins;
pub mod diagnostic;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod value;
pub mod methods;

#[cfg(test)]
mod tests;
