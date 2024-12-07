// src/core/var.rs


use crate::error::error::CalculatorError;

pub struct Variable {
    pub variable: Vec<(String, String)>,
}

impl Variable {
    pub fn init() -> Self {
        Variable {
            variable: Vec::new(),
        }
    }

    fn check_override(&self, name: &String) -> Result<(), CalculatorError> {
        if self.variable.iter().any(|(first, _)| first == name) {
            Err(CalculatorError::NameAlreadyInUse)
        } else {
            Ok(())
        }
    }

    pub fn new(&mut self, name: String, value: String) -> Result<(), CalculatorError> {
        match self.check_override(&name) {
            Ok(_) => {
                self.variable.push((name, value));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
