// src/core/function.rs


use crate::error::error::CalculatorError;

pub struct Function {
	pub function: Vec<(String, String, String)>,
} 

impl Function {
	pub fn init() -> Self {
		Function {
			function: Vec::new(),
		}
		// define your built in functions here before compilation.
	}
	
	fn check_override(&self, name:&String) -> Result<(), CalculatorError> {
		match self.function.iter().any(|(first, _, _)| first == name) {
			true => Err(CalculatorError::NameAlreadyInUse),
			false => Ok(()),
		}
	}
	
	pub fn new(&mut self, name:String, signature:String, body:String) -> Result<(), CalculatorError> {
		match self.check_override(&name) {
			Ok(_) => {
				self.function.push((name, signature, body));
				Ok(())
			},
			Err(e) => Err(e),
		}
	}
}
