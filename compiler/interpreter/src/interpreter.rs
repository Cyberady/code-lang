//! Interpreter implementation.

use parser::ast::{BinaryOperator, Expression, Program, Statement};

use crate::{environment::Environment, error::InterpreterError, value::Value};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    /// Executes an entire program.
    pub fn execute(&mut self, program: &Program) -> Result<(), InterpreterError> {
        for statement in &program.statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), InterpreterError> {
        match statement {
            Statement::VariableDeclaration { name, value } => {
                let value = self.evaluate(value)?;

                self.environment.define(name.clone(), value);

                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Value, InterpreterError> {
        match expression {
            Expression::NumberLiteral(value) => Ok(Value::Number(value.parse().unwrap())),

            Expression::Identifier(name) => self
                .environment
                .get(name)
                .cloned()
                .ok_or(InterpreterError::UndefinedVariable),

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                self.evaluate_binary(left, operator, right)
            }
        }
    }

    fn evaluate_binary(
        &self,
        left: Value,
        operator: &BinaryOperator,
        right: Value,
    ) -> Result<Value, InterpreterError> {
        match (left, operator, right) {
            (Value::Number(a), BinaryOperator::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),

            (Value::Number(a), BinaryOperator::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),

            (Value::Number(a), BinaryOperator::Multiply, Value::Number(b)) => {
                Ok(Value::Number(a * b))
            }

            (Value::Number(a), BinaryOperator::Divide, Value::Number(b)) => {
                Ok(Value::Number(a / b))
            }

            (Value::Number(a), BinaryOperator::Modulo, Value::Number(b)) => {
                Ok(Value::Number(a % b))
            }

            _ => Err(InterpreterError::InvalidBinaryOperation),
        }
    }

    /// Returns the runtime environment.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}
