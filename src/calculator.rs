// src/calculator.rs
use crate::error::CalculatorError;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Calculator {
    expression: String,
}

impl Calculator {
    pub fn new(input: &str) -> Result<Self, CalculatorError> {
        let expr = Self {
            expression: input.to_string(),
        };
        expr.validate()?;
        Ok(expr)
    }

    fn validate(&self) -> Result<(), CalculatorError> {
        let allowed_chars: HashSet<char> = [
            '+', '-', '*', '/', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', ' ', '^', '(',
            ')', 'e', 'p', 'i', '.',
        ]
        .iter()
        .cloned()
        .collect();

        for c in self.expression.chars() {
            if !allowed_chars.contains(&c) {
                return Err(CalculatorError::InvalidCharacter(c));
            }
        }
        Ok(())
    }

    pub fn evaluate(&self) -> Result<f64, CalculatorError> {
        meval::eval_str(&self.expression)
            .map_err(|e| CalculatorError::EvaluationError(e.to_string()))?
            .try_into()
            .map_err(|_| CalculatorError::ConversionError)
    }
}
