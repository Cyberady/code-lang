//! Interpreter implementation.

use parser::ast::{ BinaryOperator, Expression, Program, Statement };

use crate::{ environment::Environment, error::InterpreterError, value::Value };

use lexer::source::SourceFile;

use crate::diagnostic::Diagnostic;

use lexer::span::Span;

use std::{ cell::RefCell, rc::Rc };

pub struct Interpreter<'a> {
    environment: Rc<RefCell<Environment>>,
    _source: &'a SourceFile,
}

impl<'a> Interpreter<'a> {
    /// Creates a new interpreter.
    pub fn new(source: &'a SourceFile) -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
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

    fn execute_statement(
        &mut self,
        statement: &Statement
    ) -> Result<Option<Value>, InterpreterError> {
        match statement {
            Statement::PropertyAssignment { object, property, value, .. } => {
                let value = self.evaluate(value)?;

                self.assign_property(object, property, value)?;

                Ok(None)
            }

            Statement::VariableDeclaration { name, value, span: _ } => {
                let value = self.evaluate(value)?;

                self.environment.borrow_mut().define(name.clone(), value, true);

                Ok(None)
            }

            Statement::IndexAssignment { object, index, value, .. } => {
                let object_name = match object {
                    Expression::Identifier { name, .. } => name.clone(),

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "invalid assignment".to_string(),
                            span: lexer::span::Span::default(),
                        });
                    }
                };

                let mut array = match self.environment.borrow().get(&object_name) {
                    Some(Value::Array(values)) => values,

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "not an array".to_string(),
                            span: lexer::span::Span::default(),
                        });
                    }
                };

                let index = self.evaluate(index)?;

                let value = self.evaluate(value)?;

                let index = match index {
                    Value::Number(n) => n as usize,

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "invalid index".to_string(),
                            span: lexer::span::Span::default(),
                        });
                    }
                };

                if index >= array.len() {
                    return Err(InterpreterError::InvalidBinaryOperation {
                        operator: "index out of bounds".to_string(),
                        span: lexer::span::Span::default(),
                    });
                }

                array[index] = value;

                self.environment.borrow_mut().assign(object_name, Value::Array(array))?;

                Ok(None)
            }

            Statement::Assignment { name, value, span: _ } => {
                let value = self.evaluate(value)?;

                self.environment.borrow_mut().assign(name.clone(), value)?;

                Ok(None)
            }

            Statement::If { condition, then_branch, else_branch, .. } => {
                let value = self.evaluate(condition)?;

                match value {
                    Value::Boolean(true) => {
                        let previous = self.environment.clone();

                        self.environment = Rc::new(
                            RefCell::new(Environment::child(previous.clone()))
                        );

                        for statement in then_branch {
                            self.execute_statement(statement)?;
                        }

                        self.environment = previous;
                    }

                    Value::Boolean(false) => {
                        if let Some(statements) = else_branch {
                            let previous = self.environment.clone();

                            self.environment = Rc::new(
                                RefCell::new(Environment::child(previous.clone()))
                            );

                            for statement in statements {
                                self.execute_statement(statement)?;
                            }

                            self.environment = previous;
                        }
                    }

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "?".to_string(),
                            span: Span::default(),
                        });
                    }
                }

                Ok(None)
            }

            Statement::FunctionDeclaration { name, parameters, body, .. } => {
                self.environment
                    .borrow_mut()
                    .define_function(name.clone(), crate::environment::Function {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    });

                Ok(None)
            }

            Statement::Expression(expression) => {
                let value = self.evaluate(expression)?;
                Ok(Some(value))
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
                self.environment.borrow().get(name).ok_or(InterpreterError::UndefinedVariable {
                    name: name.clone(),
                    span: *span,
                })
            }

            Expression::Binary { left, operator, right, .. } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                self.evaluate_binary(left, operator, right)
            }

            Expression::Call { callee, arguments, .. } => self.evaluate_call(callee, arguments),

            Expression::ArrayLiteral { elements, .. } => {
                let mut values = Vec::new();

                for element in elements {
                    values.push(self.evaluate(element)?);
                }

                Ok(Value::Array(values))
            }

            Expression::ObjectLiteral { properties, .. } => {
                let mut object = std::collections::HashMap::new();

                for (key, value) in properties {
                    object.insert(key.clone(), self.evaluate(value)?);
                }

                Ok(Value::Object(object))
            }

            Expression::Index { object, index, .. } => {
                let object = self.evaluate(object)?;
                let index = self.evaluate(index)?;

                match (object, index) {
                    (Value::Array(values), Value::Number(i)) => {
                        let i = i as usize;

                        values.get(i).cloned().ok_or(InterpreterError::InvalidBinaryOperation {
                            operator: "array index out of bounds".to_string(),
                            span: lexer::span::Span::default(),
                        })
                    }

                    _ =>
                        Err(InterpreterError::InvalidBinaryOperation {
                            operator: "invalid array index".to_string(),
                            span: lexer::span::Span::default(),
                        }),
                }
            }

            Expression::Property { object, property, .. } => {
                let object = self.evaluate(object)?;

                match object {
                    Value::Object(properties) =>
                        match properties.get(property) {
                            Some(value) => Ok(value.clone()),

                            None =>
                                Err(InterpreterError::InvalidBinaryOperation {
                                    operator: format!("undefined property '{}'", property),
                                    span: Span::default(),
                                }),
                        }

                    _ =>
                        Err(InterpreterError::InvalidBinaryOperation {
                            operator: "property access".to_string(),
                            span: Span::default(),
                        }),
                }
            }
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression]
    ) -> Result<Value, InterpreterError> {
        match callee {
            Expression::Identifier { name, .. } if name == "print" => self.builtin_print(arguments),

            Expression::Identifier { name, span } => {
                let function = self.environment
                    .borrow()
                    .get_function(name)
                    .ok_or(InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    })?;

                if function.parameters.len() != arguments.len() {
                    return Err(InterpreterError::InvalidBinaryOperation {
                        operator: "?".to_string(),
                        span: Span::default(),
                    });
                }
                let previous = self.environment.clone();

                let function_environment = Rc::new(
                    RefCell::new(Environment::child(previous.clone()))
                );

                self.environment = function_environment;

                for (parameter, argument) in function.parameters.iter().zip(arguments.iter()) {
                    let value = self.evaluate(argument)?;

                    self.environment.borrow_mut().define(parameter.clone(), value, false);
                }

                let result = {
                    let mut value = Value::Null;

                    for statement in &function.body {
                        match self.execute_statement(statement) {
                            Ok(Some(return_value)) => {
                                value = return_value;
                            }

                            Ok(None) => {}

                            Err(InterpreterError::Return(return_value)) => {
                                value = return_value;
                                break;
                            }

                            Err(error) => {
                                self.environment = previous;
                                return Err(error);
                            }
                        }
                    }

                    value
                };

                self.environment = previous;

                Ok(result)
            }

            _ =>
                Err(InterpreterError::UndefinedVariable {
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

            Value::Array(values) => {
                print!("[");

                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        print!(", ");
                    }

                    match value {
                        Value::Number(n) => print!("{n}"),
                        Value::String(s) => print!("{s}"),
                        Value::Boolean(b) => print!("{b}"),
                        Value::Null => print!("null"),
                        Value::Array(_) => print!("[...]"),
                        Value::Object(_) => print!("{{...}}"),
                    }
                }

                println!("]");
            }

            Value::Object(properties) => {
                print!("{{");

                for (index, (key, value)) in properties.iter().enumerate() {
                    if index > 0 {
                        print!(", ");
                    }

                    print!("{key}: ");

                    match value {
                        Value::Number(n) => print!("{n}"),
                        Value::String(s) => print!("{s}"),
                        Value::Boolean(b) => print!("{b}"),
                        Value::Null => print!("null"),
                        Value::Array(_) => print!("[...]"),
                        Value::Object(_) => print!("{{...}}"),
                    }
                }

                println!("}}");
            }
        }

        Ok(Value::Null)
    }

    fn evaluate_binary(
        &self,
        left: Value,
        operator: &BinaryOperator,
        right: Value
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

            _ =>
                Err(InterpreterError::InvalidBinaryOperation {
                    operator: "?".to_string(),
                    span: Span::default(),
                }),
        }
    }

    fn assign_property(
        &mut self,
        object: &Expression,
        property: &str,
        value: Value
    ) -> Result<(), InterpreterError> {
        let (root_name, mut path) = self.property_path(object)?;

        let root_object = self.environment
            .borrow()
            .get(&root_name)
            .ok_or(InterpreterError::UndefinedVariable {
                name: root_name.clone(),
                span: Span::default(),
            })?;

        path.push(property.to_string());

        let updated = self.update_object_property(root_object, &path, value)?;

        self.environment.borrow_mut().assign(root_name, updated)?;

        Ok(())
    }

    fn update_object_property(
        &mut self,
        object: Value,
        path: &[String],
        value: Value
    ) -> Result<Value, InterpreterError> {
        if path.is_empty() {
            return Ok(value);
        }

        let mut map = match object {
            Value::Object(map) => map,

            _ => {
                return Err(InterpreterError::InvalidBinaryOperation {
                    operator: "not an object".to_string(),
                    span: Span::default(),
                });
            }
        };

        let key = &path[0];

        if path.len() == 1 {
            map.insert(key.clone(), value);
        } else {
            let child = map.remove(key).ok_or(InterpreterError::InvalidBinaryOperation {
                operator: format!("undefined property '{}'", key),
                span: Span::default(),
            })?;

            let updated_child = self.update_object_property(child, &path[1..], value)?;

            map.insert(key.clone(), updated_child);
        }

        Ok(Value::Object(map))
    }

    fn property_path(
        &self,
        expression: &Expression
    ) -> Result<(String, Vec<String>), InterpreterError> {
        match expression {
            Expression::Identifier { name, .. } => Ok((name.clone(), Vec::new())),

            Expression::Property { object, property, .. } => {
                let (root, mut path) = self.property_path(object)?;

                path.push(property.clone());

                Ok((root, path))
            }

            _ =>
                Err(InterpreterError::InvalidBinaryOperation {
                    operator: "invalid property assignment".to_string(),
                    span: Span::default(),
                }),
        }
    }

    pub fn environment(&self) -> std::cell::Ref<'_, Environment> {
        self.environment.borrow()
    }

    pub fn diagnostic<'b>(&'b self, error: &'b InterpreterError) -> Diagnostic<'b> {
        match error {
            InterpreterError::UndefinedVariable { name, span } =>
                Diagnostic {
                    code: "E0001",

                    title: "Undefined Variable".to_string(),

                    message: format!("Cannot find variable '{}'.", name),

                    note: Some("The variable doesn't exist in the current scope.".to_string()),

                    help: Some("Declare the variable before using it.".to_string()),

                    example: Some(format!("{} = 0\nprint({})", name, name)),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::CannotAssignConstant { name, span } =>
                Diagnostic {
                    code: "E0002",

                    title: "Cannot Assign to Constant".to_string(),

                    message: format!("Cannot modify constant '{}'.", name),

                    note: Some("Constants are immutable after they are declared.".to_string()),

                    help: Some("Use a normal variable if the value needs to change.".to_string()),

                    example: Some("value = 10\nvalue = 20".to_string()),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::InvalidBinaryOperation { operator, span } =>
                Diagnostic {
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
}
