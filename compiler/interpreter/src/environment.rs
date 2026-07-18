//! Variable storage.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

use lexer::span::Span;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
    parent: Option<Rc<RefCell<Environment>>>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<parser::ast::Statement>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    pub fn child(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value, is_const: bool) {
        self.variables.insert(name, Variable { value, is_const });
    }

    pub fn assign(
        &mut self,
        name: String,
        value: Value,
    ) -> Result<(), crate::error::InterpreterError> {
        if let Some(variable) = self.variables.get_mut(&name) {
            if variable.is_const {
                return Err(crate::error::InterpreterError::CannotAssignConstant {
                    name,
                    span: Span::default(),
                });
            }

            variable.value = value;
        } else {
            self.define(name, value, false);
        }

        Ok(())
    }

    pub fn assign_array_element(
        &mut self,
        name: &str,
        index: usize,
        value: Value,
    ) -> Result<(), crate::error::InterpreterError> {
        if let Some(variable) = self.variables.get_mut(name) {
            if variable.is_const {
                return Err(crate::error::InterpreterError::CannotAssignConstant {
                    name: name.to_string(),
                    span: Span::default(),
                });
            }

            match &mut variable.value {
                Value::Array(values) => {
                    if index >= values.len() {
                        return Err(crate::error::InterpreterError::InvalidBinaryOperation {
                            operator: "index out of bounds".to_string(),
                            span: Span::default(),
                        });
                    }

                    values[index] = value;

                    return Ok(());
                }

                _ => {
                    return Err(crate::error::InterpreterError::InvalidBinaryOperation {
                        operator: "not an array".to_string(),
                        span: Span::default(),
                    });
                }
            }
        }

        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign_array_element(name, index, value);
        }

        Err(crate::error::InterpreterError::UndefinedVariable {
            name: name.to_string(),
            span: Span::default(),
        })
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(variable) = self.variables.get(name) {
            return Some(variable.value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        None
    }

    pub fn contains(&self, name: &str) -> bool {
        if self.variables.contains_key(name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().contains(name);
        }

        false
    }

    pub fn define_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        if let Some(function) = self.functions.get(name) {
            return Some(function.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_function(name);
        }

        None
    }
}
