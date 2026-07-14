//! Variable storage.

use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
    variables: HashMap<String, Variable>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub is_const: bool,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Value, is_const: bool) {
        self.variables.insert(name, Variable { value, is_const });
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if let Some(variable) = self.variables.get_mut(&name) {
            variable.value = value;
        } else {
            self.define(name, value, false);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name).map(|variable| &variable.value)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}
