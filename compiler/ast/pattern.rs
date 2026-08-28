#![allow(dead_code)]

use super::expr::SpannedExpr;
use crate::diagnostics::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct {
        name: String,
        fields: Vec<(String, Option<Pattern>)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    Wildcard,
    Literal(Box<SpannedExpr>),
    Var(String),
    Some(String),
    None,
    Ok(String),
    Err(String),
    EnumVariant(String, Option<String>),
    EnumVariantStruct(String, Vec<(String, Option<MatchPattern>)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Box<SpannedExpr>,
    pub span: Span,
}
