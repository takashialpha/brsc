// src/error.rs

use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    EvaluationError(String),
    ConversionError,
    NameAlreadyInUse,
    InvalidVarFormat,
    InvalidVarName,
    InvalidVarValue,
    InvalidFunctionFormat,
    InvalidFunctionParameter,
    InvalidFunctionName,
}

impl std::error::Error for CalculatorError {}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            CalculatorError::ConversionError => write!(f, "Unable to convert result"),
            CalculatorError::NameAlreadyInUse => write!(f, "This name is already in use"),
            CalculatorError::InvalidVarFormat => write!(f, "Invalid format: expected 'var <name> <value>' with exactly one value. "),
            CalculatorError::InvalidVarName => write!(f, "Invalid format: <name> should be alphabetic/alphanumeric."),
            CalculatorError::InvalidVarValue => write!(f, "Invalid format: <value> should be numeric."),
            CalculatorError::InvalidFunctionFormat => write!(f, "Invalid format: func <func_name>(<func_signature>) = <func_body>, e.g double(x) = x*2"),
            CalculatorError::InvalidFunctionParameter => write!(f, "Invalid func parameter "),
            CalculatorError::InvalidFunctionName => write!(f, "Invalid function name"),
        }
    }
}
