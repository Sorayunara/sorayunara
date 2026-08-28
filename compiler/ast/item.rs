#![allow(dead_code)]

use super::stmt::SpannedStmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<SpannedStmt>,
}

impl Program {
    pub fn new(statements: Vec<SpannedStmt>) -> Self {
        Self { statements }
    }
}
