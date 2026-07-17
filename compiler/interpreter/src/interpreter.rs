//! Interpreter implementation.

use parser::ast::{BinaryOperator, Expression, Program, Statement};

use crate::{environment::Environment, error::InterpreterError, value::Value};

use lexer::source::SourceFile;

use crate::diagnostic::Diagnostic;

use lexer::span::Span;

pub struct Interpreter<'a> {
    environment: Environment,
    _source: &'a SourceFile,
}

impl<'a> Interpreter<'a> {
    /// Creates a new interpreter.
    pub fn new(source: &'a SourceFile) -> Self {
        Self {
            environment: Environment::new(),
            _source: source,
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
            Statement::VariableDeclaration {
                name,
                value,
                span: _,
            } => {
                let value = self.evaluate(value)?;

                self.environment.define(name.clone(), value, true);

                Ok(())
            }

            Statement::Assignment {
                name,
                value,
                span: _,
            } => {
                let value = self.evaluate(value)?;

                self.environment.assign(name.clone(), value)?;

                Ok(())
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let value = self.evaluate(condition)?;

                match value {
                    Value::Boolean(true) => {
                        for statement in then_branch {
                            self.execute_statement(statement)?;
                        }
                    }

                    Value::Boolean(false) => {
                        if let Some(statements) = else_branch {
                            for statement in statements {
                                self.execute_statement(statement)?;
                            }
                        }
                    }

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "?".to_string(),
                            span: Span::default(),
                        });
                    }
                }

                Ok(())
            }

            Statement::FunctionDeclaration {
                name,
                parameters,
                body,
                ..
            } => {
                self.environment.define_function(
                    name.clone(),
                    crate::environment::Function {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    },
                );

                Ok(())
            }

            Statement::Expression(expression) => {
                self.evaluate(expression)?;
                Ok(())
            }

            Statement::Return { value, .. } => {
                let value = self.evaluate(value)?;

                Err(InterpreterError::Return(value))
            }
        }
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Value, InterpreterError> {
        match expression {
            Expression::NumberLiteral { value, .. } => Ok(Value::Number(value.parse().unwrap())),

            Expression::StringLiteral { value, .. } => Ok(Value::String(value.clone())),

            Expression::BooleanLiteral { value, .. } => Ok(Value::Boolean(*value)),

            Expression::NullLiteral { .. } => Ok(Value::Null),

            Expression::Identifier { name, span } => {
                self.environment
                    .get(name)
                    .cloned()
                    .ok_or(InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    })
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                self.evaluate_binary(left, operator, right)
            }

            Expression::Call {
                callee, arguments, ..
            } => self.evaluate_call(callee, arguments),
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<Value, InterpreterError> {
        match callee {
            Expression::Identifier { name, .. } if name == "print" => self.builtin_print(arguments),

            Expression::Identifier { name, span } => {
                let function = self.environment.get_function(name).cloned().ok_or(
                    InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    },
                )?;

                if function.parameters.len() != arguments.len() {
                    return Err(InterpreterError::InvalidBinaryOperation {
                        operator: "?".to_string(),
                        span: Span::default(),
                    });
                }

                for (parameter, argument) in function.parameters.iter().zip(arguments.iter()) {
                    let value = self.evaluate(argument)?;

                    self.environment.define(parameter.clone(), value, false);
                }

                let last_index = function.body.len().saturating_sub(1);

                for (index, statement) in function.body.iter().enumerate() {
                    match self.execute_statement(statement) {
                        Ok(()) => {
                            if index == last_index {
                                if let Statement::Expression(expression) = statement {
                                    let value = self.evaluate(expression)?;
                                    return Ok(value);
                                }
                            }
                        }

                        Err(InterpreterError::Return(value)) => {
                            return Ok(value);
                        }

                        Err(error) => {
                            return Err(error);
                        }
                    }
                }

                Ok(Value::Null)
            }

            _ => Err(InterpreterError::UndefinedVariable {
                name: match callee {
                    Expression::Identifier { name, .. } => name.clone(),
                    _ => "<unknown>".to_string(),
                },
                span: *callee.span(),
            }),
        }
    }

    fn builtin_print(&mut self, arguments: &[Expression]) -> Result<Value, InterpreterError> {
        if arguments.len() != 1 {
            return Err(InterpreterError::InvalidBinaryOperation {
                operator: "?".to_string(),
                span: Span::default(),
            });
        }

        let value = self.evaluate(&arguments[0])?;

        match &value {
            Value::Number(number) => println!("{number}"),

            Value::String(text) => println!("{text}"),

            Value::Boolean(boolean) => println!("{boolean}"),

            Value::Null => println!("null"),
        }

        Ok(Value::Null)
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

            (Value::Number(a), BinaryOperator::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),

            (Value::Number(a), BinaryOperator::LessEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a <= b))
            }

            (Value::Number(a), BinaryOperator::Greater, Value::Number(b)) => {
                Ok(Value::Boolean(a > b))
            }

            (Value::Number(a), BinaryOperator::GreaterEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a >= b))
            }

            (Value::Number(a), BinaryOperator::EqualEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a == b))
            }

            (Value::Number(a), BinaryOperator::BangEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a != b))
            }

            _ => Err(InterpreterError::InvalidBinaryOperation {
                operator: "?".to_string(),
                span: Span::default(),
            }),
        }
    }

    pub fn diagnostic<'b>(&'b self, error: &'b InterpreterError) -> Diagnostic<'b> {
        match error {
            InterpreterError::UndefinedVariable { name, span } => Diagnostic {
                code: "E0001",

                title: "Undefined Variable".to_string(),

                message: format!("Cannot find variable '{}'.", name),

                note: Some("The variable doesn't exist in the current scope.".to_string()),

                help: Some("Declare the variable before using it.".to_string()),

                example: Some(format!("{} = 0\nprint({})", name, name)),

                span: *span,

                source: self._source,
            },

            InterpreterError::CannotAssignConstant { name, span } => Diagnostic {
                code: "E0002",

                title: "Cannot Assign to Constant".to_string(),

                message: format!("Cannot modify constant '{}'.", name),

                note: Some("Constants are immutable after they are declared.".to_string()),

                help: Some("Use a normal variable if the value needs to change.".to_string()),

                example: Some("value = 10\nvalue = 20".to_string()),

                span: *span,

                source: self._source,
            },

            InterpreterError::InvalidBinaryOperation { operator, span } => Diagnostic {
                code: "E0003",

                title: "Invalid Operation".to_string(),

                message: format!("Operator '{}' cannot be applied to these values.", operator),

                note: Some("Both operands must support the selected operator.".to_string()),

                help: Some("Check the value types before using this operator.".to_string()),

                example: Some("10 + 20\ntrue and false".to_string()),

                span: *span,

                source: self._source,
            },

            InterpreterError::Return(_) => unreachable!(),
        }
    }

    /// Returns the runtime environment.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}
