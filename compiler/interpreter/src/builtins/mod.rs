pub mod file;
pub mod math;
pub mod print;
pub mod random;
pub mod range;
pub mod time;
pub mod input;
pub mod number;

use parser::ast::Expression;

use crate::{ error::InterpreterError, interpreter::Interpreter, value::Value };

pub fn call(
    interpreter: &mut Interpreter,
    name: &str,
    arguments: &[Expression]
) -> Result<Value, InterpreterError> {
    match name {
        "print" => print::call(interpreter, arguments),
        "range" => range::call(interpreter, arguments),
        "input" => input::call(interpreter, arguments),
        "number" => number::call(interpreter, arguments),

        _ => unreachable!(),
    }
}
