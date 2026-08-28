#![allow(dead_code)]

use super::pattern::MatchArm;
use super::stmt::SpannedStmt;
use super::types::TypeNode;
use crate::diagnostics::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Neg,
    Not,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedExpr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Null,
    Some(Box<SpannedExpr>),
    None,
    Ok(Box<SpannedExpr>),
    Err(Box<SpannedExpr>),
    Var(String),
    Borrow {
        expr: Box<SpannedExpr>,
        is_mut: bool,
    },
    Move(Box<SpannedExpr>),
    UnsafeBlock(Vec<SpannedStmt>),
    Task(Box<SpannedExpr>),
    Await(Box<SpannedExpr>),
    Spawn {
        callee: String,
        args: Vec<SpannedExpr>,
    },
    MakeChan(Box<TypeNode>),
    ChanSend {
        chan: Box<SpannedExpr>,
        value: Box<SpannedExpr>,
    },
    ChanRecv(Box<SpannedExpr>),
    Binary {
        left: Box<SpannedExpr>,
        op: BinaryOpKind,
        right: Box<SpannedExpr>,
    },
    Unary {
        op: UnaryOpKind,
        expr: Box<SpannedExpr>,
    },
    Call {
        callee: String,
        args: Vec<SpannedExpr>,
    },
    Tuple(Vec<SpannedExpr>),
    Array(Vec<SpannedExpr>),
    Set(Vec<SpannedExpr>),
    Map(Vec<(SpannedExpr, SpannedExpr)>),
    Index {
        target: Box<SpannedExpr>,
        index: Box<SpannedExpr>,
    },
    Dot {
        target: Box<SpannedExpr>,
        field: String,
    },
    Match {
        value: Box<SpannedExpr>,
        arms: Vec<MatchArm>,
    },
    CustomBinary {
        left: Box<SpannedExpr>,
        operator: String,
        right: Box<SpannedExpr>,
    },
    IsA {
        value: Box<SpannedExpr>,
        type_node: TypeNode,
    },
    EnumVariantConstruct {
        enum_name: String,
        variant_name: String,
        payload: Option<Box<SpannedExpr>>,
    },
    Block(Vec<SpannedStmt>),
}
