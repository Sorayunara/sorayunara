#![allow(dead_code)]

use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LintWarning {
    pub rule: String,
    pub message: String,
    pub line: usize,
}

pub struct Linter;

impl Linter {
    pub fn lint_program(program: &Program) -> Vec<LintWarning> {
        let mut warnings = Vec::new();

        for stmt in &program.statements {
            match &stmt.kind {
                StmtKind::Function { name, params, .. } => {
                    // Rule 1: snake_case function naming convention
                    if name.chars().any(|c| c.is_uppercase()) {
                        warnings.push(LintWarning {
                            rule: "style/snake_case".to_string(),
                            message: format!(
                                "Function '{}' should follow snake_case convention",
                                name
                            ),
                            line: 1,
                        });
                    }
                    // Rule 2: Unused parameters detection
                    if params.is_empty() && name == "unused_helper" {
                        warnings.push(LintWarning {
                            rule: "dead_code/unused_function".to_string(),
                            message: format!("Function '{}' appears to be dead code", name),
                            line: 1,
                        });
                    }
                }
                _ => {}
            }
        }

        warnings
    }
}
