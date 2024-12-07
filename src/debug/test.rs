// src/debug/test.rs

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::core::calculator::Calculator;
    use crate::cli::cli::Cli;
    use crate::error::error::CalculatorError;

    #[test]
    fn test_calculator_valid_expressions() {
        let test_cases = vec![
            ("2 + 2", 4.0),
            ("3 * 5", 15.0),
            ("10 / 2", 5.0),
            ("2^3", 8.0),
            ("(2 + 3) * 4", 20.0),
        ];

        for (expr, expected) in test_cases {
            let calculator = Calculator::new(expr).unwrap();
            let result = calculator.evaluate().unwrap();
            assert!((result - expected).abs() < f64::EPSILON, 
                    "Failed for expression: {}, expected {}, got {}", expr, expected, result);
        }
    }

    #[test]
    fn test_cli_precision() {
        let cli = Cli {
            expr: Some("10 / 3".to_string()),
            precision: 2,
            quiet: false,
        };

        let result = cli.run();
        assert!(result.is_ok(), "CLI run failed unexpectedly");
    }
    
    #[test]
    fn test_error_display() {
        let eval_error = CalculatorError::EvaluationError("test error".to_string());
        let eval_error_msg = eval_error.to_string();
        assert_eq!(eval_error_msg, "Evaluation error: test error");
    }
}
