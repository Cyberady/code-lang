//! Interpreter for the Code programming language.

pub mod builtins;
pub mod diagnostic;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod methods;
pub mod value;

#[cfg(test)]
mod tests;
