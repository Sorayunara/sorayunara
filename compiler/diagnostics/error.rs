#![allow(dead_code)]

use super::span::Span;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub span: Span,
    pub code: Option<&'static str>,
}
