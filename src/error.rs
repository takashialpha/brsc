// src/error.rs

use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    InvalidCharacter(char),
    EvaluationError(String),
    ConversionError,
}

impl std::error::Error for CalculatorError {}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::InvalidCharacter(c) => {
                write!(f, "Invalid character '{}' in expression", c)
            }
            CalculatorError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            CalculatorError::ConversionError => write!(f, "Unable to convert result"),
        }
    }
}
