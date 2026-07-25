pub mod print;
pub mod range;
pub mod math;
pub mod random;

use parser::ast::Expression;

use crate::{
    error::InterpreterError,
    interpreter::Interpreter,
    value::Value,
};

pub fn call(
    interpreter: &mut Interpreter,
    name: &str,
    arguments: &[Expression],
) -> Result<Value, InterpreterError> {
    match name {
        "print" => print::call(interpreter, arguments),
        "range" => range::call(interpreter, arguments),

        _ => unreachable!(),
    }
}
