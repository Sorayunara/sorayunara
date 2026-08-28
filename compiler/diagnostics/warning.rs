#![allow(dead_code)]

use super::span::Span;

#[derive(Debug, Clone)]
pub struct CompilerWarning {
    pub message: String,
    pub span: Span,
}
