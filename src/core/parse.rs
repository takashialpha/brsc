use crate::core::var::Variable;
use crate::error::error::CalculatorError;

pub fn parse(expr: &mut String) -> Result<(), CalculatorError> {
    if let Some(var_body) = expr.strip_prefix("var ") {
        let mut variable = Variable::init(); 
        match parse_variable(var_body, &mut variable) {
            Ok(_) => {
            	for (name, value) in &variable.variable {
            		*expr = expr.replace(name, value);
            	}
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}

fn parse_variable(var_body: &str, variable: &mut Variable) -> Result<(), CalculatorError> {
    let parts: Vec<&str> = var_body.split_whitespace().collect();
    if parts.len() == 2 {
        let name = parts[0];
        if !name.chars().all(|c| c.is_alphanumeric()) || name.contains('=') {
            return Err(CalculatorError::InvalidVarName);
        }

        let value = parts[1];
        if value.parse::<f64>().is_err() {
            return Err(CalculatorError::InvalidVarValue);
        }

        match variable.new(name.to_string(), value.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        Err(CalculatorError::InvalidVarFormat) 
    }
}

// future function implementation 
/*
fn parse_function(func_body: &str) -> Result<(), CalculatorError> {
    let parts: Vec<&str> = func_body.splitn(2, '=').collect();
    if parts.len() == 2 {
        let signature = parts[0].trim();
        let body = parts[1].trim();

        if let Some((name, param)) = parse_function_signature(signature)? {
            Function::new(name, param, body.to_string());
            return Ok(());
        } else {
            return Err(CalculatorError::InvalidFunctionFormat);
        }
    }

}

fn parse_function_signature(signature: &str) -> Result<(String, String), CalculatorError> {
    if let Some(open_paren) = signature.find('(') {
        if let Some(close_paren) = signature.find(')') {
            if close_paren > open_paren {
                let name = &signature[..open_paren];
                let param = &signature[(open_paren + 1)..close_paren];

                if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric()) || name.contains('=') {
                    return Err(CalculatorError::InvalidFunctionName);
                }

                if param.is_empty() || !param.chars().all(|c| c.is_alphanumeric()) {
                    return Err(CalculatorError::InvalidFunctionParameter);
                }

                return Ok((name.to_string(), param.to_string()));
            }
        }
    }
}
*/
