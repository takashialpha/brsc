// src/core/calculator.rs
use crate::core::parse::parse;
use crate::error::error::CalculatorError;

#[derive(Debug)]
pub struct Calculator {
    pub expression: String, // mark this as pub if you want to run tests
}

impl Calculator {
    pub fn new(input: &str) -> Result<Self, CalculatorError> {
        let expr = Self {
            expression: input.to_string(),
        };
        Ok(expr)
    }


    pub fn evaluate(&mut self) -> Result<f64, CalculatorError> {
		match parse(&mut self.expression) {
	        Ok(_) => { 
	        meval::eval_str(&self.expression)
	            .map_err(|e| CalculatorError::EvaluationError(e.to_string()))?
	            .try_into()
	            .map_err(|_| CalculatorError::ConversionError)
	        }
	    	Err(e) => Err(e)
        }	
	}
}
