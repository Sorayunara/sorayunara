#![allow(dead_code)]

use super::expr::SpannedExpr;
use super::stmt::SpannedStmt;

pub trait AstVisitor {
    fn visit_stmt(&mut self, stmt: &SpannedStmt) {
        let _ = stmt;
    }

    fn visit_expr(&mut self, expr: &SpannedExpr) {
        let _ = expr;
    }
}
