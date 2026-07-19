//! Interpreter implementation.

use parser::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};

use crate::{environment::Environment, error::InterpreterError, value::Value};

use lexer::source::SourceFile;

use crate::diagnostic::Diagnostic;

use lexer::span::Span;

use std::{cell::RefCell, rc::Rc};

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
        statement: &Statement,
    ) -> Result<Option<Value>, InterpreterError> {
        match statement {
            Statement::PropertyAssignment {
                object,
                property,
                value,
                ..
            } => {
                let value = self.evaluate(value)?;

                self.assign_property(object, property, value)?;

                Ok(None)
            }

            Statement::ConstantDeclaration { name, value, .. } => {
                let value = self.evaluate(value)?;

                self.environment
                    .borrow_mut()
                    .define(name.clone(), value, true);

                Ok(None)
            }

            Statement::IndexAssignment {
                object,
                index,
                value,
                span,
            } => {
                let object_name = match object {
                    Expression::Identifier { name, .. } => name.clone(),

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "Value is not an array.".to_string(),
                            span: *span,
                        });
                    }
                };

                let mut array = match self.environment.borrow().get(&object_name) {
                    Some(Value::Array(values)) => values,

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "Value is not an array.".to_string(),
                            span: *span,
                        });
                    }
                };

                let index = self.evaluate(index)?;

                let value = self.evaluate(value)?;

                let index = match index {
                    Value::Number(n) => n as usize,

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "Array index must be a number.".to_string(),
                            span: *span,
                        });
                    }
                };

                if index >= array.len() {
                    return Err(InterpreterError::RuntimeError {
                        message: "Array index out of bounds.".to_string(),
                        span: *span,
                    });
                }

                array[index] = value;

                self.environment
                    .borrow_mut()
                    .assign(object_name, Value::Array(array))?;

                Ok(None)
            }

            Statement::Assignment {
                name,
                value,
                span: _,
            } => {
                let value = self.evaluate(value)?;

                self.environment.borrow_mut().assign(name.clone(), value)?;

                Ok(None)
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
                span,
            } => {
                let value = self.evaluate(condition)?;

                match value {
                    Value::Boolean(true) => {
                        let previous = self.environment.clone();

                        self.environment =
                            Rc::new(RefCell::new(Environment::child(previous.clone())));

                        for statement in then_branch {
                            self.execute_statement(statement)?;
                        }

                        self.environment = previous;
                    }

                    Value::Boolean(false) => {
                        if let Some(statements) = else_branch {
                            let previous = self.environment.clone();

                            self.environment =
                                Rc::new(RefCell::new(Environment::child(previous.clone())));

                            for statement in statements {
                                self.execute_statement(statement)?;
                            }

                            self.environment = previous;
                        }
                    }

                    _ => {
                        return Err(InterpreterError::InvalidBinaryOperation {
                            operator: "?".to_string(),
                            span: *span,
                        });
                    }
                }

                Ok(None)
            }

            Statement::While {
                condition,
                body,
                span,
            } => {
                'while_loop: loop {
                    let value = self.evaluate(condition)?;

                    match value {
                        Value::Boolean(true) => {
                            for statement in body {
                                match self.execute_statement(statement) {
                                    Ok(_) => {}

                                    Err(InterpreterError::Continue) => {
                                        continue 'while_loop;
                                    }

                                    Err(InterpreterError::Break) => {
                                        break 'while_loop;
                                    }

                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                        }

                        Value::Boolean(false) => {
                            break;
                        }

                        _ => {
                            return Err(InterpreterError::RuntimeError {
                                message: "While condition must be a boolean.".to_string(),
                                span: *span,
                            });
                        }
                    }
                }

                Ok(None)
            }

            Statement::FunctionDeclaration {
                name,
                parameters,
                body,
                ..
            } => {
                self.environment.borrow_mut().define_function(
                    name.clone(),
                    crate::environment::Function {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    },
                );

                Ok(None)
            }

            Statement::Expression(expression) => {
                let value = self.evaluate(expression)?;
                Ok(Some(value))
            }

            Statement::Break { .. } => Err(InterpreterError::Break),

            Statement::Continue { .. } => Err(InterpreterError::Continue),

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
                    .borrow()
                    .get(name)
                    .ok_or(InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    })
            }

            Expression::Binary {
                left,
                operator,
                right,
                span,
            } => {
                match operator {
                    BinaryOperator::And => {
                        let left = self.evaluate(left)?;

                        match left {
                            Value::Boolean(false) => {
                                // Short-circuit: don't evaluate right
                                Ok(Value::Boolean(false))
                            }

                            Value::Boolean(true) => {
                                let right = self.evaluate(right)?;

                                match right {
                                    Value::Boolean(value) => Ok(Value::Boolean(value)),

                                    _ => Err(InterpreterError::RuntimeError {
                                        message: "Operator 'and' requires boolean operands."
                                            .to_string(),
                                        span: *span,
                                    }),
                                }
                            }

                            _ => Err(InterpreterError::RuntimeError {
                                message: "Operator 'and' requires boolean operands.".to_string(),
                                span: *span,
                            }),
                        }
                    }

                    BinaryOperator::Or => {
                        let left = self.evaluate(left)?;

                        match left {
                            Value::Boolean(true) => {
                                // Short-circuit: don't evaluate right
                                Ok(Value::Boolean(true))
                            }

                            Value::Boolean(false) => {
                                let right = self.evaluate(right)?;

                                match right {
                                    Value::Boolean(value) => Ok(Value::Boolean(value)),

                                    _ => Err(InterpreterError::RuntimeError {
                                        message: "Operator 'or' requires boolean operands."
                                            .to_string(),
                                        span: *span,
                                    }),
                                }
                            }

                            _ => Err(InterpreterError::RuntimeError {
                                message: "Operator 'or' requires boolean operands.".to_string(),
                                span: *span,
                            }),
                        }
                    }

                    _ => {
                        let left = self.evaluate(left)?;
                        let right = self.evaluate(right)?;

                        self.evaluate_binary(left, operator, right, *span)
                    }
                }
            }

            Expression::Unary {
                operator,
                expression,
                span,
            } => {
                let value = self.evaluate(expression)?;

                match (operator, value) {
                    (UnaryOperator::Not, Value::Boolean(value)) => Ok(Value::Boolean(!value)),

                    _ => Err(InterpreterError::RuntimeError {
                        message: format!(
                            "Operator '{}' requires a boolean operand.",
                            operator.as_str()
                        ),
                        span: *span,
                    }),
                }
            }

            Expression::Call {
                callee, arguments, ..
            } => self.evaluate_call(callee, arguments),

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

            Expression::Index {
                object,
                index,
                span,
            } => {
                let object = self.evaluate(object)?;
                let index = self.evaluate(index)?;

                match (object, index) {
                    (Value::Array(values), Value::Number(i)) => {
                        let i = i as usize;

                        values
                            .get(i)
                            .cloned()
                            .ok_or(InterpreterError::RuntimeError {
                                message: "Array index out of bounds.".to_string(),
                                span: *span,
                            })
                    }

                    _ => Err(InterpreterError::RuntimeError {
                        message: "Invalid array index.".to_string(),
                        span: *span,
                    }),
                }
            }

            Expression::Property {
                object,
                property,
                span,
            } => {
                let object = self.evaluate(object)?;

                match object {
                    Value::Array(values) if property == "length" => {
                        Ok(Value::Number(values.len() as f64))
                    }

                    Value::Object(properties) => match properties.get(property) {
                        Some(value) => Ok(value.clone()),

                        None => Err(InterpreterError::RuntimeError {
                            message: format!("Undefined property '{}'.", property),
                            span: *span,
                        }),
                    },

                    _ => Err(InterpreterError::RuntimeError {
                        message: "Property access is only supported on objects.".to_string(),
                        span: *span,
                    }),
                }
            }
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<Value, InterpreterError> {
        match callee {
            Expression::Identifier { name, .. } if name == "print" => self.builtin_print(arguments),

            Expression::Property {
                object,
                property,
                span,
            } => self.evaluate_property_call(object, property, arguments, *span),

            Expression::Identifier { name, span } => {
                let function = self.environment.borrow().get_function(name).ok_or(
                    InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: *span,
                    },
                )?;

                if function.parameters.len() != arguments.len() {
                    return Err(InterpreterError::InvalidBinaryOperation {
                        operator: "?".to_string(),
                        span: *span,
                    });
                }
                let previous = self.environment.clone();

                let function_environment =
                    Rc::new(RefCell::new(Environment::child(previous.clone())));

                self.environment = function_environment;

                for (parameter, argument) in function.parameters.iter().zip(arguments.iter()) {
                    let value = self.evaluate(argument)?;

                    self.environment
                        .borrow_mut()
                        .define(parameter.clone(), value, false);
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
        right: Value,
        span: Span,
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
                operator: operator.as_str().to_string(),
                span,
            }),
        }
    }

    fn assign_property(
        &mut self,
        object: &Expression,
        property: &str,
        value: Value,
    ) -> Result<(), InterpreterError> {
        let (root_name, mut path) = self.property_path(object)?;

        let root_object = self.environment.borrow().get(&root_name).ok_or(
            InterpreterError::UndefinedVariable {
                name: root_name.clone(),
                span: Span::default(),
            },
        )?;

        path.push(property.to_string());

        let updated = self.update_object_property(root_object, &path, value)?;

        self.environment.borrow_mut().assign(root_name, updated)?;

        Ok(())
    }

    fn update_object_property(
        &mut self,
        object: Value,
        path: &[String],
        value: Value,
    ) -> Result<Value, InterpreterError> {
        if path.is_empty() {
            return Ok(value);
        }

        let mut map = match object {
            Value::Object(map) => map,

            _ => {
                return Err(InterpreterError::RuntimeError {
                    message: "Value is not an object.".to_string(),
                    span: Span::default(),
                });
            }
        };

        let key = &path[0];

        if path.len() == 1 {
            map.insert(key.clone(), value);
        } else {
            let child = map.remove(key).ok_or(InterpreterError::RuntimeError {
                message: format!("Undefined property '{}'.", key),
                span: Span::default(),
            })?;

            let updated_child = self.update_object_property(child, &path[1..], value)?;

            map.insert(key.clone(), updated_child);
        }

        Ok(Value::Object(map))
    }

    fn property_path(
        &self,
        expression: &Expression,
    ) -> Result<(String, Vec<String>), InterpreterError> {
        match expression {
            Expression::Identifier { name, .. } => Ok((name.clone(), Vec::new())),

            Expression::Property {
                object, property, ..
            } => {
                let (root, mut path) = self.property_path(object)?;

                path.push(property.clone());

                Ok((root, path))
            }

            _ => Err(InterpreterError::InvalidBinaryOperation {
                operator: "invalid property assignment".to_string(),
                span: Span::default(),
            }),
        }
    }

    pub fn environment(&self) -> std::cell::Ref<'_, Environment> {
        self.environment.borrow()
    }

    fn evaluate_property_call(
        &mut self,
        object: &Expression,
        property: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Result<Value, InterpreterError> {
        match object {
            Expression::Identifier { name, .. } => {
                let mut array = match self.environment.borrow().get(name) {
                    Some(Value::Array(values)) => values,

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "Value is not an array.".to_string(),
                            span,
                        });
                    }
                };

                self.array_method(name, &mut array, property, arguments, span)
            }

            _ => {
                let object = self.evaluate(object)?;

                match object {
                    Value::Array(_) => {
                        todo!("temporary non-identifier support");
                    }

                    _ => Err(InterpreterError::RuntimeError {
                        message: format!("Method '{}' is not supported on this value.", property),
                        span,
                    }),
                }
            }
        }
    }

    fn array_method(
        &mut self,
        name: &str,
        array: &mut Vec<Value>,
        property: &str,
        arguments: &[Expression],
        span: Span,
    ) -> Result<Value, InterpreterError> {
        match property {
            "add" => {
                match arguments.len() {
                    1 => {
                        let value = self.evaluate(&arguments[0])?;

                        array.push(value);
                    }

                    2 => {
                        let index = self.evaluate(&arguments[0])?;

                        let index = match index {
                            Value::Number(n) => n as usize,

                            _ => {
                                return Err(InterpreterError::RuntimeError {
                                    message: "Insert index must be a number.".to_string(),
                                    span,
                                });
                            }
                        };

                        if index > array.len() {
                            return Err(InterpreterError::RuntimeError {
                                message: "Array index out of bounds.".to_string(),
                                span,
                            });
                        }

                        let value = self.evaluate(&arguments[1])?;

                        array.insert(index, value);
                    }

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "array.add expects 1 or 2 arguments.".to_string(),
                            span,
                        });
                    }
                }

                self.environment
                    .borrow_mut()
                    .assign(name.to_string(), Value::Array(array.clone()))?;

                Ok(Value::Null)
            }

            "remove" => {
                match arguments.len() {
                    0 => {
                        if array.is_empty() {
                            return Err(InterpreterError::RuntimeError {
                                message: "Array is empty.".to_string(),
                                span,
                            });
                        }

                        array.remove(array.len() - 1);
                    }

                    1 => {
                        let index = self.evaluate(&arguments[0])?;

                        let index = match index {
                            Value::Number(n) => n as usize,

                            _ => {
                                return Err(InterpreterError::RuntimeError {
                                    message: "Remove index must be a number.".to_string(),
                                    span,
                                });
                            }
                        };

                        if index >= array.len() {
                            return Err(InterpreterError::RuntimeError {
                                message: "Array index must be a number.".to_string(),
                                span,
                            });
                        }

                        array.remove(index);
                    }

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "array.remove expects 0 or 1 arguments.".to_string(),
                            span,
                        });
                    }
                }

                self.environment
                    .borrow_mut()
                    .assign(name.to_string(), Value::Array(array.clone()))?;

                Ok(Value::Null)
            }

            "contains" => {
                if arguments.len() != 1 {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.contains expects 1 argument.".to_string(),
                        span,
                    });
                }

                let value = self.evaluate(&arguments[0])?;

                Ok(Value::Boolean(array.contains(&value)))
            }

            "clear" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.clear expects 0 arguments.".to_string(),
                        span,
                    });
                }

                array.clear();

                self.environment
                    .borrow_mut()
                    .assign(name.to_string(), Value::Array(array.clone()))?;

                Ok(Value::Null)
            }

            "first" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.first expects 0 arguments.".to_string(),
                        span,
                    });
                }

                match array.first() {
                    Some(value) => Ok(value.clone()),

                    None => Err(InterpreterError::RuntimeError {
                        message: "Array is empty.".to_string(),
                        span,
                    }),
                }
            }

            "last" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.last expects 0 arguments.".to_string(),
                        span,
                    });
                }

                match array.last() {
                    Some(value) => Ok(value.clone()),

                    None => Err(InterpreterError::RuntimeError {
                        message: "Array is empty.".to_string(),
                        span,
                    }),
                }
            }

            "isEmpty" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.isEmpty expects 0 arguments.".to_string(),
                        span,
                    });
                }

                Ok(Value::Boolean(array.is_empty()))
            }

            "reverse" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.reverse expects 0 arguments.".to_string(),
                        span,
                    });
                }

                array.reverse();

                self.environment
                    .borrow_mut()
                    .assign(name.to_string(), Value::Array(array.clone()))?;

                Ok(Value::Null)
            }

            "sort" => {
                if !arguments.is_empty() {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.sort expects 0 arguments.".to_string(),
                        span,
                    });
                }

                if !array.iter().all(|value| matches!(value, Value::Number(_))) {
                    return Err(InterpreterError::RuntimeError {
                        message: "array.sort only supports numeric arrays.".to_string(),
                        span,
                    });
                }

                array.sort_by(|a, b| {
                    let a = match a {
                        Value::Number(n) => *n,
                        _ => unreachable!(),
                    };

                    let b = match b {
                        Value::Number(n) => *n,
                        _ => unreachable!(),
                    };

                    a.partial_cmp(&b).unwrap()
                });

                self.environment
                    .borrow_mut()
                    .assign(name.to_string(), Value::Array(array.clone()))?;

                Ok(Value::Null)
            }

            _ => Err(InterpreterError::RuntimeError {
                message: format!("Unknown array method '{}'.", property),
                span,
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

            InterpreterError::RuntimeError { message, span } => Diagnostic {
                code: "E0004",

                title: "Runtime Error".to_string(),

                message: message.clone(),

                note: None,

                help: None,

                example: None,

                span: *span,

                source: self._source,
            },

            InterpreterError::Return(_) => unreachable!(),

            InterpreterError::Break => unreachable!(),

            InterpreterError::Continue => unreachable!(),
        }
    }
}
