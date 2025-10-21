//! Dead code injection for analysis confusion

use crate::parser::ParseResult;
use anyhow::Result;
use rand::Rng;

/// Dead code injector
pub struct DeadCodeInjector {
    density: f32, // 0.0 to 1.0
}

impl DeadCodeInjector {
    pub fn new(density: f32) -> Self {
        Self {
            density: density.clamp(0.0, 1.0),
        }
    }

    /// Generate dead code snippets
    pub fn generate(&self, parse_result: &ParseResult) -> Result<Vec<String>> {
        let num_snippets = (parse_result.strings.len() as f32 * self.density) as usize;
        
        let mut snippets = Vec::with_capacity(num_snippets);
        let mut rng = rand::thread_rng();
        
        for _ in 0..num_snippets {
            let snippet_type = rng.gen_range(0..5);
            let snippet = match snippet_type {
                0 => self.generate_fake_calculation(),
                1 => self.generate_fake_condition(),
                2 => self.generate_fake_loop(),
                3 => self.generate_fake_function(),
                _ => self.generate_fake_assignment(),
            };
            
            snippets.push(snippet);
        }
        
        log::debug!("Generated {} dead code snippets", snippets.len());
        Ok(snippets)
    }

    /// Generate fake calculation that never executes
    fn generate_fake_calculation(&self) -> String {
        let mut rng = rand::thread_rng();
        let var1 = format!("_tmp{}", rng.gen_range(1000..9999));
        let var2 = format!("_tmp{}", rng.gen_range(1000..9999));
        let val1 = rng.gen_range(1..100);
        let val2 = rng.gen_range(1..100);
        
        format!(
            "if false then local {} = {}; local {} = {} + {}; end",
            var1, val1, var2, var1, val2
        )
    }

    /// Generate fake conditional that never executes
    fn generate_fake_condition(&self) -> String {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1..100);
        
        format!(
            "if {} ~= {} then error('Unreachable') end",
            val, val
        )
    }

    /// Generate fake loop that never executes
    fn generate_fake_loop(&self) -> String {
        let mut rng = rand::thread_rng();
        let var = format!("_i{}", rng.gen_range(1000..9999));
        
        format!(
            "for {} = 1, 0 do print('Never executes') end",
            var
        )
    }

    /// Generate fake function that never gets called
    fn generate_fake_function(&self) -> String {
        let mut rng = rand::thread_rng();
        let fn_name = format!("_fn{}", rng.gen_range(1000..9999));
        let param = format!("_p{}", rng.gen_range(100..999));
        
        format!(
            "local function {}({}) return {} * 2 end",
            fn_name, param, param
        )
    }

    /// Generate fake assignment
    fn generate_fake_assignment(&self) -> String {
        let mut rng = rand::thread_rng();
        let var = format!("_var{}", rng.gen_range(1000..9999));
        let val = rng.gen_range(1..100);
        
        format!("do local {} = {} end", var, val)
    }

    /// Generate fake table operations
    pub fn generate_fake_table(&self) -> String {
        let mut rng = rand::thread_rng();
        let tbl_name = format!("_tbl{}", rng.gen_range(1000..9999));
        
        format!(
            "do local {} = {{}} {}[1] = nil end",
            tbl_name, tbl_name
        )
    }

    /// Generate fake string operations
    pub fn generate_fake_string(&self) -> String {
        let mut rng = rand::thread_rng();
        let var = format!("_str{}", rng.gen_range(1000..9999));
        let chars = ["a", "b", "c", "x", "y", "z"];
        let char = chars[rng.gen_range(0..chars.len())];
        
        format!(
            "if false then local {} = '{}' .. '{}' end",
            var, char, char
        )
    }

    /// Generate fake metamethod
    pub fn generate_fake_metamethod(&self) -> String {
        let mut rng = rand::thread_rng();
        let tbl = format!("_mt{}", rng.gen_range(1000..9999));
        
        format!(
            "do local {} = {{}} setmetatable({}, {{__index = function() return nil end}}) end",
            tbl, tbl
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NumericLiteral, StringLiteral};

    fn create_test_parse_result() -> ParseResult {
        ParseResult {
            ast: None,
            strings: vec![
                StringLiteral {
                    value: "test1".to_string(),
                    line: 1,
                    column: 0,
                    sensitivity: crate::parser::Sensitivity::Low,
                },
                StringLiteral {
                    value: "test2".to_string(),
                    line: 2,
                    column: 0,
                    sensitivity: crate::parser::Sensitivity::Low,
                },
            ],
            numbers: vec![],
            functions: vec![],
        }
    }

    #[test]
    fn test_dead_code_generation() {
        let injector = DeadCodeInjector::new(0.5);
        let parse_result = create_test_parse_result();
        
        let snippets = injector.generate(&parse_result).unwrap();
        
        // With 2 strings and 0.5 density, should generate ~1 snippet
        assert!(!snippets.is_empty());
    }

    #[test]
    fn test_density_clamping() {
        let injector1 = DeadCodeInjector::new(-0.5);
        assert_eq!(injector1.density, 0.0);
        
        let injector2 = DeadCodeInjector::new(1.5);
        assert_eq!(injector2.density, 1.0);
    }

    #[test]
    fn test_fake_calculation() {
        let injector = DeadCodeInjector::new(0.1);
        let calc = injector.generate_fake_calculation();
        
        assert!(calc.contains("if false then"));
        assert!(calc.contains("_tmp"));
    }

    #[test]
    fn test_fake_condition() {
        let injector = DeadCodeInjector::new(0.1);
        let cond = injector.generate_fake_condition();
        
        assert!(cond.contains("if"));
        assert!(cond.contains("~="));
    }

    #[test]
    fn test_fake_loop() {
        let injector = DeadCodeInjector::new(0.1);
        let loop_code = injector.generate_fake_loop();
        
        assert!(loop_code.contains("for"));
        assert!(loop_code.contains("= 1, 0"));
    }

    #[test]
    fn test_fake_function() {
        let injector = DeadCodeInjector::new(0.1);
        let func = injector.generate_fake_function();
        
        assert!(func.contains("local function"));
        assert!(func.contains("return"));
    }

    #[test]
    fn test_fake_table() {
        let injector = DeadCodeInjector::new(0.1);
        let tbl = injector.generate_fake_table();
        
        assert!(tbl.contains("{}"));
        assert!(tbl.contains("[1] = nil"));
    }

    #[test]
    fn test_zero_density() {
        let injector = DeadCodeInjector::new(0.0);
        let parse_result = create_test_parse_result();
        
        let snippets = injector.generate(&parse_result).unwrap();
        
        assert_eq!(snippets.len(), 0);
    }

    #[test]
    fn test_high_density() {
        let injector = DeadCodeInjector::new(1.0);
        let mut parse_result = create_test_parse_result();
        
        // Add more strings to test high density
        for i in 3..10 {
            parse_result.strings.push(StringLiteral {
                value: format!("test{}", i),
                line: i,
                column: 0,
                sensitivity: crate::parser::Sensitivity::Low,
            });
        }
        
        let snippets = injector.generate(&parse_result).unwrap();
        
        // With 9 strings and 1.0 density, should generate ~9 snippets
        assert!(snippets.len() >= 5);
    }
}