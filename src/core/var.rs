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
		// define your built in vars here before copilation.
	}
	
	fn check_override(&self, name:&String) -> Result<(), CalculatorError> {
		match self.variable.iter().any(|(first, _)| first == name) {
			true => Err(CalculatorError::NameAlreadyInUse),
			false => Ok(()),
		}
	}
	
	pub fn new(&mut self, name:String, value:String) -> Result<(), CalculatorError> {
		match self.check_override(&name) {
			Ok(_) => {
				self.variable.push((name, value));
				Ok(())
			},
			Err(e) => Err(e),
		}
	}
}
