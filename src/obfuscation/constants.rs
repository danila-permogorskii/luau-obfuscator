//! Constant obfuscation using mathematical expressions

use super::ObfuscatedConstant;
use crate::parser::NumericLiteral;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use rand::Rng;

/// Constant obfuscator
pub struct ConstantObfuscator {
    complexity: usize,
}

impl ConstantObfuscator {
    pub fn new() -> Self {
        Self { complexity: 2 }
    }

    /// Obfuscate numeric constants
    pub fn obfuscate(&self, numbers: &[NumericLiteral]) -> Result<Vec<ObfuscatedConstant>> {
        numbers
            .iter()
            .map(|num_lit| self.obfuscate_number(num_lit))
            .collect()
    }

    /// Obfuscate a single number
    fn obfuscate_number(&self, num_lit: &NumericLiteral) -> Result<ObfuscatedConstant> {
        let value = if num_lit.is_float {
            self.obfuscate_float(&num_lit.value)?
        } else {
            self.obfuscate_integer(&num_lit.value)?
        };

        Ok(ObfuscatedConstant {
            original: num_lit.value.clone(),
            obfuscated_expr: value,
            line: num_lit.line,
        })
    }

    /// Obfuscate an integer
    fn obfuscate_integer(&self, value: &str) -> Result<String> {
        let num: i64 = value
            .parse()
            .map_err(|e| ObfuscatorError::ObfuscationError(format!("Invalid integer: {}", e)))?;

        let mut rng = rand::thread_rng();
        
        // Generate random operations that result in the target number
        match rng.gen_range(0..4) {
            0 => {
                // Addition: (num - rand) + rand
                let rand_val = rng.gen_range(-1000..1000);
                Ok(format!("({} + {})", num - rand_val, rand_val))
            }
            1 => {
                // Multiplication: (num * rand) / rand
                let rand_val = rng.gen_range(2..10);
                Ok(format!("({} / {})", num * rand_val, rand_val))
            }
            2 => {
                // XOR: (num ^ rand) ^ rand
                let rand_val = rng.gen_range(1..255);
                Ok(format!("(({} ~ {}) ~ {})", num, rand_val, rand_val))
            }
            _ => {
                // Subtraction: (num + rand) - rand
                let rand_val = rng.gen_range(1..1000);
                Ok(format!("({} - {})", num + rand_val, rand_val))
            }
        }
    }

    /// Obfuscate a float
    fn obfuscate_float(&self, value: &str) -> Result<String> {
        let num: f64 = value
            .parse()
            .map_err(|e| ObfuscatorError::ObfuscationError(format!("Invalid float: {}", e)))?;

        let mut rng = rand::thread_rng();
        
        // For floats, use simpler operations to avoid precision loss
        match rng.gen_range(0..2) {
            0 => {
                // Addition
                let rand_val = rng.gen_range(1.0..100.0);
                Ok(format!("({} + {})", num - rand_val, rand_val))
            }
            _ => {
                // Multiplication
                let rand_val = rng.gen_range(2.0..10.0);
                Ok(format!("({} / {})", num * rand_val, rand_val))
            }
        }
    }

    /// Generate complex nested expression (for premium tier)
    pub fn obfuscate_complex(&self, num_lit: &NumericLiteral) -> Result<ObfuscatedConstant> {
        let value = if num_lit.is_float {
            self.obfuscate_float_complex(&num_lit.value)?
        } else {
            self.obfuscate_integer_complex(&num_lit.value)?
        };

        Ok(ObfuscatedConstant {
            original: num_lit.value.clone(),
            obfuscated_expr: value,
            line: num_lit.line,
        })
    }

    fn obfuscate_integer_complex(&self, value: &str) -> Result<String> {
        let num: i64 = value.parse().unwrap();
        let mut rng = rand::thread_rng();
        
        // Multi-layer obfuscation
        let r1 = rng.gen_range(1..100);
        let r2 = rng.gen_range(1..100);
        let r3 = rng.gen_range(2..10);
        
        Ok(format!(
            "((({} + {}) - {}) * {}) / {}",
            num - r1,
            r1 + r2,
            r2,
            r3,
            r3
        ))
    }

    fn obfuscate_float_complex(&self, value: &str) -> Result<String> {
        let num: f64 = value.parse().unwrap();
        let mut rng = rand::thread_rng();
        
        let r1 = rng.gen_range(1.0..50.0);
        let r2 = rng.gen_range(2.0..5.0);
        
        Ok(format!("(({} + {}) * {}) / {}", num - r1, r1, r2, r2))
    }
}

impl Default for ConstantObfuscator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_obfuscation() {
        let obfuscator = ConstantObfuscator::new();
        
        let num_lit = NumericLiteral {
            value: "42".to_string(),
            line: 1,
            column: 0,
            is_float: false,
        };

        let obfuscated = obfuscator.obfuscate_number(&num_lit).unwrap();
        
        assert_eq!(obfuscated.original, "42");
        assert!(obfuscated.obfuscated_expr.contains('('));
        assert!(obfuscated.obfuscated_expr.contains(')'));
    }

    #[test]
    fn test_float_obfuscation() {
        let obfuscator = ConstantObfuscator::new();
        
        let num_lit = NumericLiteral {
            value: "3.14".to_string(),
            line: 1,
            column: 0,
            is_float: true,
        };

        let obfuscated = obfuscator.obfuscate_number(&num_lit).unwrap();
        
        assert_eq!(obfuscated.original, "3.14");
        assert!(obfuscated.obfuscated_expr.contains('('));
    }

    #[test]
    fn test_complex_obfuscation() {
        let obfuscator = ConstantObfuscator::new();
        
        let num_lit = NumericLiteral {
            value: "100".to_string(),
            line: 1,
            column: 0,
            is_float: false,
        };

        let obfuscated = obfuscator.obfuscate_complex(&num_lit).unwrap();
        
        // Complex obfuscation should have multiple operators
        let op_count = obfuscated.obfuscated_expr.matches(&['+', '-', '*', '/']).count();
        assert!(op_count >= 3);
    }

    #[test]
    fn test_batch_obfuscation() {
        let obfuscator = ConstantObfuscator::new();
        
        let numbers = vec![
            NumericLiteral {
                value: "1".to_string(),
                line: 1,
                column: 0,
                is_float: false,
            },
            NumericLiteral {
                value: "2.5".to_string(),
                line: 2,
                column: 0,
                is_float: true,
            },
        ];

        let obfuscated = obfuscator.obfuscate(&numbers).unwrap();
        assert_eq!(obfuscated.len(), 2);
    }
}
