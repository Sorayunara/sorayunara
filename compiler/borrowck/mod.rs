#![allow(dead_code)]

use std::collections::HashMap;
use crate::diagnostics::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoanKind {
    Shared,
    Mutable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loan {
    pub variable: String,
    pub kind: LoanKind,
    pub span: Span,
}

pub struct BorrowChecker {
    active_loans: HashMap<String, Vec<Loan>>,
    moved_variables: HashMap<String, Span>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            active_loans: HashMap::new(),
            moved_variables: HashMap::new(),
        }
    }

    pub fn check_move(&mut self, var_name: &str, span: Span) -> Result<(), String> {
        if let Some(prev_span) = self.moved_variables.get(var_name) {
            return Err(format!("Use of moved value '{}' previously moved at line {}", var_name, prev_span.line));
        }
        if let Some(loans) = self.active_loans.get(var_name) {
            if !loans.is_empty() {
                return Err(format!("Cannot move out of '{}' because it is currently borrowed", var_name));
            }
        }
        self.moved_variables.insert(var_name.to_string(), span);
        Ok(())
    }

    pub fn borrow(&mut self, var_name: &str, kind: LoanKind, span: Span) -> Result<(), String> {
        if let Some(prev_span) = self.moved_variables.get(var_name) {
            return Err(format!("Cannot borrow moved value '{}' at line {}", var_name, prev_span.line));
        }
        let loans = self.active_loans.entry(var_name.to_string()).or_default();
        if kind == LoanKind::Mutable && !loans.is_empty() {
            return Err(format!("Cannot borrow '{}' as mutable more than once at a time", var_name));
        }
        if kind == LoanKind::Shared && loans.iter().any(|l| l.kind == LoanKind::Mutable) {
            return Err(format!("Cannot borrow '{}' as shared while it is borrowed mutably", var_name));
        }
        loans.push(Loan {
            variable: var_name.to_string(),
            kind,
            span,
        });
        Ok(())
    }

    pub fn release_loans(&mut self, var_name: &str) {
        self.active_loans.remove(var_name);
    }
}
