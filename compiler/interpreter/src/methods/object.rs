use crate::{error::InterpreterError, interpreter::Interpreter, value::Value};

use parser::ast::Expression;
use lexer::span::Span;

pub fn call(
    _interpreter: &mut Interpreter,
    _text: String,
    _property: &str,
    _arguments: &[Expression],
    _span: Span,
) -> Result<Value, InterpreterError> {
    todo!("objects methods");
}
