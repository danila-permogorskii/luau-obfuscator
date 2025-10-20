//! AST visitor pattern for traversing full_moon AST

use super::ast::{FunctionInfo, NumericLiteral, Sensitivity, StringLiteral};
use full_moon::ast::{Ast, Expression, FunctionCall, Stmt, Value};
use full_moon::visitors::Visitor;
use log::debug;

/// AST visitor that extracts information during traversal
pub struct AstVisitor {
    pub strings: Vec<StringLiteral>,
    pub numbers: Vec<NumericLiteral>,
    pub functions: Vec<FunctionInfo>,
    current_line: usize,
}

impl AstVisitor {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            numbers: Vec::new(),
            functions: Vec::new(),
            current_line: 1,
        }
    }

    pub fn visit_ast(&mut self, ast: &Ast) {
        // Visit all top-level statements
        for stmt in ast.nodes().stmts() {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalFunction(local_fn) => {
                let name = local_fn.name().to_string();
                let parameters = local_fn
                    .func_body()
                    .parameters()
                    .iter()
                    .map(|p| p.to_string())
                    .collect();

                self.functions.push(FunctionInfo {
                    name: Some(name),
                    parameters,
                    line: self.current_line,
                    is_local: true,
                });

                debug!("Found local function at line {}", self.current_line);
            }

            Stmt::FunctionDeclaration(fn_decl) => {
                let name = fn_decl.name().to_string();
                let parameters = fn_decl
                    .func_body()
                    .parameters()
                    .iter()
                    .map(|p| p.to_string())
                    .collect();

                self.functions.push(FunctionInfo {
                    name: Some(name),
                    parameters,
                    line: self.current_line,
                    is_local: false,
                });

                debug!("Found function declaration at line {}", self.current_line);
            }

            Stmt::LocalAssignment(local_assign) => {
                // Visit expressions in the assignment
                for expr in local_assign.expr_list().iter() {
                    self.visit_expression(expr);
                }
            }

            Stmt::Assignment(assign) => {
                // Visit expressions in the assignment
                for expr in assign.expr_list().iter() {
                    self.visit_expression(expr);
                }
            }

            Stmt::FunctionCall(fn_call) => {
                self.visit_function_call(fn_call);
            }

            _ => {
                // For other statement types, we'd need more detailed handling
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Value { value, .. } => {
                self.visit_value(value);
            }
            Expression::Parentheses { expression, .. } => {
                self.visit_expression(expression);
            }
            Expression::UnaryOperator { expression, .. } => {
                self.visit_expression(expression);
            }
            Expression::BinaryOperator { lhs, rhs, .. } => {
                self.visit_expression(lhs);
                self.visit_expression(rhs);
            }
            _ => {}
        }
    }

    fn visit_value(&mut self, value: &Box<Value>) {
        match value.as_ref() {
            Value::String(token) => {
                let value_str = token.to_string();
                // Remove surrounding quotes
                let cleaned = value_str.trim_matches('"').trim_matches('\'').to_string();

                self.strings.push(StringLiteral {
                    value: cleaned.clone(),
                    line: self.current_line,
                    column: 0, // full_moon doesn't provide column info easily
                    sensitivity: Sensitivity::classify(&cleaned),
                });

                debug!("Found string literal: {:?}", cleaned);
            }

            Value::Number(token) => {
                let value_str = token.to_string();
                let is_float = value_str.contains('.');

                self.numbers.push(NumericLiteral {
                    value: value_str.clone(),
                    line: self.current_line,
                    column: 0,
                    is_float,
                });

                debug!("Found numeric literal: {}", value_str);
            }

            Value::FunctionCall(fn_call) => {
                self.visit_function_call(fn_call);
            }

            Value::Function(func_body) => {
                let parameters = func_body.parameters().iter().map(|p| p.to_string()).collect();

                self.functions.push(FunctionInfo {
                    name: None,
                    parameters,
                    line: self.current_line,
                    is_local: false,
                });

                debug!("Found anonymous function at line {}", self.current_line);
            }

            _ => {}
        }
    }

    fn visit_function_call(&mut self, fn_call: &FunctionCall) {
        // Visit arguments to extract string/number literals
        if let Some(args) = fn_call.suffixes().iter().find_map(|suffix| {
            if let full_moon::ast::Suffix::Call(call) = suffix {
                match call {
                    full_moon::ast::Call::AnonymousCall(args) => Some(args),
                    full_moon::ast::Call::MethodCall(method_call) => Some(method_call.args()),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            match args {
                full_moon::ast::FunctionArgs::Parentheses { arguments, .. } => {
                    for expr in arguments.iter() {
                        self.visit_expression(expr);
                    }
                }
                full_moon::ast::FunctionArgs::String(token) => {
                    let value_str = token.to_string();
                    let cleaned = value_str.trim_matches('"').trim_matches('\'').to_string();

                    self.strings.push(StringLiteral {
                        value: cleaned.clone(),
                        line: self.current_line,
                        column: 0,
                        sensitivity: Sensitivity::classify(&cleaned),
                    });
                }
                _ => {}
            }
        }
    }
}

impl Default for AstVisitor {
    fn default() -> Self {
        Self::new()
    }
}
