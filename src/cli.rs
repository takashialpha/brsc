// src/cli.rs
use std::io::{self, Write};
use clap::Parser;
use crate::calculator::Calculator;
use crate::error::CalculatorError;

#[derive(Parser, Debug)]
#[command(
    name = "brsc", 
    version = "0.9.0", 
    about = "brsc (basic rust calculator): A powerful mathematical expression calculator, made by takashialpha",
    long_about = "brsc (basic rust calculator): A flexible CLI calculator written in rust inspired in bc, made by takashialpha"
)]
pub struct Cli {
    /// Mathematical expression to evaluate
    #[arg(short, long)]
    expr: Option<String>,

    /// Precision of floating point results
    #[arg(short, long, default_value_t = 2)]
    precision: usize,

    /// Suppress all output except results
    #[arg(short, long)]
    quiet: bool,
}

impl Cli {
    pub fn run(&self) -> Result<(), CalculatorError> {
        match &self.expr {
            Some(expression) => self.evaluate_expression(expression),
            None => {
            	if self.quiet {
            		self.interactive_mode(self.quiet)
            	} else {
            		println!("\nLicensed under the Apache License 2.0. You may use, copy, modify, and distribute this software under the terms of the License.\nSee http://www.apache.org/licenses/LICENSE-2.0 for details.\n  
            		\nbrsc (basic rust calculator): A powerful mathematical expression calculator, made by takashialpha\n");
            		self.interactive_mode(self.quiet)
            	}
            },
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<(), CalculatorError> {
        let calculator = Calculator::new(expr)?;
        let result = calculator.evaluate()?;
        
        if !self.quiet {
            println!("Result: {:.precision$}", result, precision = self.precision);
        } else {
            println!("{:.precision$}", result, precision = self.precision);
        }

        Ok(())
    }

    fn interactive_mode(&self, quiet: bool) -> Result<(), CalculatorError> {
    	if quiet {
	        loop {
	            io::stdout().flush().map_err(|_| CalculatorError::ConversionError)?;

	            let mut input = String::new();
	            io::stdin().read_line(&mut input)
	                .map_err(|_| CalculatorError::ConversionError)?;
	            
	            let input = input.trim();

	            match input {
	                "exit" => break Ok(()),
	                "" => continue,
	                _ => {
	                    match Calculator::new(input).and_then(|calc| calc.evaluate()) {
	                        Ok(result) => {
	                        println!("{:.precision$}", result, precision = self.precision);
	                        },
	                        Err(e) => eprintln!("Error: {}", e),
	                    }
	                }
	            }
	        }
		} else {
			loop {
				print!("Enter expression (or 'exit'): ");
				io::stdout().flush().map_err(|_| CalculatorError::ConversionError)?;
				
				let mut input = String::new();
				io::stdin().read_line(&mut input)
					.map_err(|_| CalculatorError::ConversionError)?;
					            
				let input = input.trim();
				
				match input {
					"exit" => break Ok(()),
					"" => continue,
					_ => {
						match Calculator::new(input).and_then(|calc| calc.evaluate()) {
					    	Ok(result) => {
					        	if !self.quiet {
					            	println!("Result: {:.precision$}", result, precision = self.precision);
					            } else {
					            	println!("{:.precision$}", result, precision = self.precision);
					            }
					        },
					        Err(e) => eprintln!("Error: {}", e),
					    }
				    }
		        }
	        }
		}  
    }
}
