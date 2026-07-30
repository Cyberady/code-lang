//! Interpreter implementation.

use parser::ast::{ BinaryOperator, Expression, Program, Statement, UnaryOperator };

use crate::{ environment::Environment, error::InterpreterError, methods, value::Value };

use lexer::source::SourceFile;

use crate::diagnostic::Diagnostic;

use lexer::span::Span;

use std::{ cell::RefCell, rc::Rc };

use crate::builtins;

use lexer::lexer::Lexer;

use parser::parser::Parser;

pub struct Interpreter<'a> {
    pub(crate) environment: Rc<RefCell<Environment>>,
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

            Statement::ConstantDeclaration { name, value, span } => {
                if self.environment.borrow().contains_variable(name) {
                    return Err(InterpreterError::DuplicateDeclaration {
                        name: name.clone(),
                        span: *span,
                    });
                }

                let value = self.evaluate(value)?;

                self.environment.borrow_mut().define(name.clone(), value, true);

                Ok(None)
            }

            Statement::IndexAssignment { object, index, value, span } => {
                let index_span = *index.span();
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
                    Value::Number(n) => {
                        if n < 0.0 {
                            return Err(InterpreterError::RuntimeError {
                                message: "Array index cannot be negative.".to_string(),
                                span: index_span,
                            });
                        }

                        n as usize
                    }

                    _ => {
                        return Err(InterpreterError::RuntimeError {
                            message: "Array index must be a number.".to_string(),
                            span: index_span,
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

                self.environment.borrow_mut().assign(object_name, Value::Array(array), *span)?;

                Ok(None)
            }

            Statement::Assignment { name, value, span } => {
                let value = self.evaluate(value)?;

                self.environment.borrow_mut().assign(name.clone(), value, *span)?;

                Ok(None)
            }

            Statement::If { condition, then_branch, else_branch, span } => {
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
                            span: *span,
                        });
                    }
                }

                Ok(None)
            }

            Statement::While { condition, body, span } => {
                'while_loop: loop {
                    let value = self.evaluate(condition)?;

                    match value {
                        Value::Boolean(true) => {
                            let previous = self.environment.clone();

                            self.environment = Rc::new(
                                RefCell::new(Environment::child(previous.clone()))
                            );

                            for statement in body {
                                match self.execute_statement(statement) {
                                    Ok(_) => {}

                                    Err(InterpreterError::Continue) => {
                                        self.environment = previous;
                                        continue 'while_loop;
                                    }

                                    Err(InterpreterError::Break) => {
                                        self.environment = previous;
                                        break 'while_loop;
                                    }

                                    Err(error) => {
                                        self.environment = previous;
                                        return Err(error);
                                    }
                                }
                            }

                            self.environment = previous;
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

            Statement::For { variable, iterable, body, span } => {
                let iterable = self.evaluate(iterable)?;

                match iterable {
                    Value::Array(values) => {
                        'for_loop: for value in values {
                            let previous = self.environment.clone();

                            self.environment = Rc::new(
                                RefCell::new(Environment::child(previous.clone()))
                            );

                            self.environment.borrow_mut().assign(variable.clone(), value, *span)?;

                            for statement in body {
                                match self.execute_statement(statement) {
                                    Ok(_) => {}

                                    Err(InterpreterError::Continue) => {
                                        self.environment = previous;
                                        continue 'for_loop;
                                    }

                                    Err(InterpreterError::Break) => {
                                        self.environment = previous;
                                        break 'for_loop;
                                    }

                                    Err(error) => {
                                        self.environment = previous;
                                        return Err(error);
                                    }
                                }
                            }

                            self.environment = previous;
                        }

                        Ok(None)
                    }

                    _ =>
                        Err(InterpreterError::RuntimeError {
                            message: "For loop expects an array.".to_string(),
                            span: *span,
                        }),
                }
            }

            Statement::FunctionDeclaration { name, parameters, body, span } => {
                if self.environment.borrow().contains_function(name) {
                    return Err(InterpreterError::DuplicateDeclaration {
                        name: name.clone(),
                        span: *span,
                    });
                }

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

            Statement::Break { .. } => Err(InterpreterError::Break),

            Statement::Continue { .. } => Err(InterpreterError::Continue),

            Statement::Return { value, .. } => {
                let value = self.evaluate(value)?;

                Err(InterpreterError::Return(value))
            }
        }
    }

    pub(super) fn evaluate(&mut self, expression: &Expression) -> Result<Value, InterpreterError> {
        match expression {
            Expression::NumberLiteral { value, .. } => Ok(Value::Number(value.parse().unwrap())),

            Expression::StringLiteral { value, .. } => {
                let text = self.interpolate_string(value)?;

                Ok(Value::String(text))
            }

            Expression::BooleanLiteral { value, .. } => Ok(Value::Boolean(*value)),

            Expression::NullLiteral { .. } => Ok(Value::Null),

            Expression::Identifier { name, span } => {
                self.environment.borrow().get(name).ok_or(InterpreterError::UndefinedVariable {
                    name: name.clone(),
                    span: *span,
                })
            }

            Expression::Binary { left, operator, right, span } => {
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

                                    _ =>
                                        Err(InterpreterError::RuntimeError {
                                            message: "Operator 'and' requires boolean operands.".to_string(),
                                            span: *span,
                                        }),
                                }
                            }

                            _ =>
                                Err(InterpreterError::RuntimeError {
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

                                    _ =>
                                        Err(InterpreterError::RuntimeError {
                                            message: "Operator 'or' requires boolean operands.".to_string(),
                                            span: *span,
                                        }),
                                }
                            }

                            _ =>
                                Err(InterpreterError::RuntimeError {
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

            Expression::Unary { operator, expression, span } => {
                let value = self.evaluate(expression)?;

                match (operator, value) {
                    (UnaryOperator::Plus, Value::Number(value)) => Ok(Value::Number(value)),

                    (UnaryOperator::Minus, Value::Number(value)) => Ok(Value::Number(-value)),

                    (UnaryOperator::Not, Value::Boolean(value)) => Ok(Value::Boolean(!value)),

                    (UnaryOperator::Plus, _) | (UnaryOperator::Minus, _) => {
                        Err(InterpreterError::RuntimeError {
                            message: format!(
                                "Operator '{}' requires a number operand.",
                                operator.as_str()
                            ),
                            span: *span,
                        })
                    }

                    (UnaryOperator::Not, _) =>
                        Err(InterpreterError::RuntimeError {
                            message: "Operator 'not' requires a boolean operand.".to_string(),
                            span: *span,
                        }),
                }
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

            Expression::Index { object, index, span } => {
                let index_span = *index.span();
                let object = self.evaluate(object)?;
                let index = self.evaluate(index)?;

                match (object, index) {
                    (Value::Array(values), Value::Number(i)) => {
                        if i < 0.0 {
                            return Err(InterpreterError::RuntimeError {
                                message: "Array index cannot be negative.".to_string(),
                                span: index_span,
                            });
                        }

                        let i = i as usize;

                        values.get(i).cloned().ok_or(InterpreterError::RuntimeError {
                            message: "Array index out of bounds.".to_string(),
                            span: *span,
                        })
                    }
                    _ =>
                        Err(InterpreterError::RuntimeError {
                            message: "Invalid array index.".to_string(),
                            span: index_span,
                        }),
                }
            }

            Expression::Property { object, property, span } => {
                // Builtin namespaces
                if let Expression::Identifier { name, .. } = object.as_ref() {
                    if name == "math" {
                        return builtins::math::property(property, *span);
                    }

                    if name == "random" {
                        return builtins::random::property(property, *span);
                    }

                    if name == "file" {
                        return builtins::file::property(property, *span);
                    }
                }

                let object = self.evaluate(object)?;

                match object {
                    Value::Array(values) if property == "length" => {
                        Ok(Value::Number(values.len() as f64))
                    }

                    Value::String(text) if property == "length" => {
                        Ok(Value::Number(text.chars().count() as f64))
                    }

                    Value::Object(properties) if property == "length" => {
                        Ok(Value::Number(properties.len() as f64))
                    }

                    Value::Object(properties) =>
                        match properties.get(property) {
                            Some(value) => Ok(value.clone()),

                            None =>
                                Err(InterpreterError::RuntimeError {
                                    message: format!("Undefined property '{}'.", property),
                                    span: *span,
                                }),
                        }

                    _ =>
                        Err(InterpreterError::RuntimeError {
                            message: "Property access is only supported on objects.".to_string(),
                            span: *span,
                        }),
                }
            }
        }
    }

    fn evaluate_interpolation(&mut self, expression: &str) -> Result<Value, InterpreterError> {
        let source = SourceFile::new("<interpolation>".to_string(), expression.to_string());

        let mut lexer = Lexer::new(&source);

        let tokens = lexer.tokenize().map_err(|error| InterpreterError::RuntimeError {
            message: format!("Interpolation lexer error: {:?}", error),
            span: Span::default(),
        })?;

        let mut parser = Parser::new(tokens);

        let expression = parser
            .parse_expression_only()
            .map_err(|error| InterpreterError::RuntimeError {
                message: format!("Interpolation parser error: {:?}", error),
                span: Span::default(),
            })?;

        self.evaluate(&expression)
    }

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression]
    ) -> Result<Value, InterpreterError> {
        match callee {
            Expression::Identifier { name, .. } if matches!(name.as_str(), "print" | "range") => {
                builtins::call(self, name, arguments)
            }

            Expression::Property { object, property, span } =>
                self.evaluate_property_call(object, property, arguments, *span),

            Expression::Identifier { name, span } => {
                let function = match self.environment.borrow().get_function(name) {
                    Some(function) => function,

                    None => {
                        if self.environment.borrow().contains(name) {
                            return Err(InterpreterError::NotCallable {
                                name: name.clone(),
                                span: *span,
                            });
                        }

                        return Err(InterpreterError::UndefinedVariable {
                            name: name.clone(),
                            span: *span,
                        });
                    }
                };

                if function.parameters.len() != arguments.len() {
                    return Err(InterpreterError::InvalidArgumentCount {
                        expected: function.parameters.len(),
                        found: arguments.len(),
                        span: *span,
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

    fn evaluate_binary(
        &self,
        left: Value,
        operator: &BinaryOperator,
        right: Value,
        span: Span
    ) -> Result<Value, InterpreterError> {
        match (left, operator, right) {
            (Value::Number(a), BinaryOperator::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),

            (Value::Number(a), BinaryOperator::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),

            (Value::Number(a), BinaryOperator::Multiply, Value::Number(b)) => {
                Ok(Value::Number(a * b))
            }

            (Value::Number(a), BinaryOperator::Divide, Value::Number(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::RuntimeError {
                        message: "Division by zero.".to_string(),
                        span,
                    });
                }

                Ok(Value::Number(a / b))
            }

            (Value::Number(a), BinaryOperator::Modulo, Value::Number(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::RuntimeError {
                        message: "Modulo by zero.".to_string(),
                        span,
                    });
                }

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
                    operator: operator.as_str().to_string(),
                    span,
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

        self.environment.borrow_mut().assign(root_name, updated, Span::default())?;

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

    fn interpolate_string(&mut self, text: &str) -> Result<String, InterpreterError> {
        if !text.contains('{') {
            return Ok(text.to_string());
        }

        let chars: Vec<char> = text.chars().collect();

        let mut result = String::new();

        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '{' {
                let start = i + 1;

                i += 1;

                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }

                if i >= chars.len() {
                    return Err(InterpreterError::RuntimeError {
                        message: "Missing closing '}' in interpolation.".to_string(),
                        span: Span::default(),
                    });
                }

                let expression: String = chars[start..i].iter().collect();

                let value = self.evaluate_interpolation(&expression)?;

                result.push_str(&value.to_string());
            } else {
                result.push(chars[i]);
            }

            i += 1;
        }

        Ok(result)
    }

    fn evaluate_property_call(
        &mut self,
        object: &Expression,
        property: &str,
        arguments: &[Expression],
        span: Span
    ) -> Result<Value, InterpreterError> {
        match object {
            Expression::Identifier { name, .. } => {
                if name == "math" {
                    return builtins::math::call(self, property, arguments, span);
                }

                if name == "random" {
                    return builtins::random::call(self, property, arguments, span);
                }

                if name == "time" {
                    return builtins::time::call(self, property, arguments, span);
                }

                if name == "file" {
                    return builtins::file::call(self, property, arguments, span);
                }

                let value = self.environment
                    .borrow()
                    .get(name)
                    .ok_or(InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span,
                    })?;

                match value {
                    Value::Array(mut array) => {
                        methods::array::call(self, name, &mut array, property, arguments, span)
                    }

                    Value::String(text) => {
                        methods::string::call(self, text, property, arguments, span)
                    }

                    Value::Object(mut object) => {
                        methods::object::call(self, name, &mut object, property, arguments, span)
                    }

                    _ =>
                        Err(InterpreterError::RuntimeError {
                            message: format!("Method '{}' is not supported on this value.", property),
                            span,
                        }),
                }
            }

            _ => {
                let value = self.evaluate(object)?;

                match value {
                    Value::Array(_) => {
                        todo!("temporary non-identifier support");
                    }

                    Value::String(text) => {
                        methods::string::call(self, text, property, arguments, span)
                    }

                    Value::Object(_) => {
                        todo!("temporary object support");
                    }

                    _ =>
                        Err(InterpreterError::RuntimeError {
                            message: format!("Method '{}' is not supported on this value.", property),
                            span,
                        }),
                }
            }
        }
    }

    pub fn diagnostic<'b>(&'b self, error: &'b InterpreterError) -> Diagnostic<'b> {
        match error {
            InterpreterError::UndefinedVariable { name, span } =>
                Diagnostic {
                    code: "E1001",

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
                    code: "E1002",

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
                    code: "E1003",

                    title: "Invalid Operation".to_string(),

                    message: format!("Operator '{}' cannot be applied to these values.", operator),

                    note: Some("Both operands must support the selected operator.".to_string()),

                    help: Some("Check the value types before using this operator.".to_string()),

                    example: Some("10 + 20\ntrue and false".to_string()),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::RuntimeError { message, span } =>
                Diagnostic {
                    code: "E1004",

                    title: "Runtime Error".to_string(),

                    message: message.clone(),

                    note: None,

                    help: None,

                    example: None,

                    span: *span,

                    source: self._source,
                },

            InterpreterError::NotCallable { name, span } =>
                Diagnostic {
                    code: "E1005",

                    title: "Value is Not Callable".to_string(),

                    message: format!("'{}' is not a function and cannot be called.", name),

                    note: Some(
                        "Only functions and built-in functions can be called using '()'.".to_string()
                    ),

                    help: Some("Remove '()' or assign a function to this variable.".to_string()),

                    example: Some("func hello() {}\nhello()".to_string()),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::InvalidArgumentCount { expected, found, span } =>
                Diagnostic {
                    code: "E1006",

                    title: "Invalid Argument Count".to_string(),

                    message: format!(
                        "Function expected {} argument(s) but received {}.",
                        expected,
                        found
                    ),

                    note: Some(
                        "Every function parameter must receive a corresponding argument.".to_string()
                    ),

                    help: Some(
                        "Call the function with the correct number of arguments.".to_string()
                    ),

                    example: Some(
                        "func add(a, b) {\n    return a + b\n}\n\nadd(10, 20)".to_string()
                    ),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::DuplicateDeclaration { name, span } =>
                Diagnostic {
                    code: "E1007",

                    title: "Duplicate Declaration".to_string(),

                    message: format!("'{}' is already declared.", name),

                    note: Some("Names must be unique within the same scope.".to_string()),

                    help: Some(
                        "Choose a different name or remove the previous declaration.".to_string()
                    ),

                    example: Some("const value = 10".to_string()),

                    span: *span,

                    source: self._source,
                },

            InterpreterError::Return(_) => unreachable!(),

            InterpreterError::Break => unreachable!(),

            InterpreterError::Continue => unreachable!(),
        }
    }
}
