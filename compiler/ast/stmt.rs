#![allow(dead_code)]

use super::expr::SpannedExpr;
use super::pattern::Pattern;
use super::types::TypeNode;
use crate::diagnostics::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TypeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternFnDecl {
    pub name: String,
    pub params: Vec<(String, TypeNode)>,
    pub ret_type: TypeNode,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethodParam {
    pub name: String,
    pub type_annot: Option<TypeNode>,
    pub is_self: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethodSig {
    pub name: String,
    pub type_params: Vec<GenericParam>,
    pub params: Vec<TraitMethodParam>,
    pub ret_type: TypeNode,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedTypeDecl {
    pub name: String,
    pub bounds: Vec<TypeNode>,
    pub default: Option<TypeNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImplItem {
    AssociatedType {
        name: String,
        target: TypeNode,
        span: Span,
    },
    Method(Box<SpannedStmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedStmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    Let {
        name: String,
        is_mut: bool,
        type_annot: Option<TypeNode>,
        value: SpannedExpr,
    },
    LetDestructure {
        pattern: Pattern,
        type_annot: Option<TypeNode>,
        value: SpannedExpr,
    },
    Const {
        name: String,
        type_annot: Option<TypeNode>,
        value: SpannedExpr,
    },
    Comptime(Vec<SpannedStmt>),
    Assign {
        target: String,
        value: SpannedExpr,
    },
    AssignIndex {
        target: SpannedExpr,
        index: SpannedExpr,
        value: SpannedExpr,
    },
    Function {
        attributes: Vec<Attribute>,
        name: String,
        type_params: Vec<GenericParam>,
        is_async: bool,
        params: Vec<(String, TypeNode)>,
        ret_type: TypeNode,
        body: Vec<SpannedStmt>,
    },
    StructDecl {
        attributes: Vec<Attribute>,
        name: String,
        type_params: Vec<GenericParam>,
        fields: Vec<(String, TypeNode)>,
    },
    EnumDecl {
        name: String,
        type_params: Vec<GenericParam>,
        variants: Vec<(String, Option<TypeNode>)>,
    },
    TypeAlias {
        name: String,
        type_params: Vec<GenericParam>,
        target: TypeNode,
    },
    TraitDecl {
        name: String,
        type_params: Vec<GenericParam>,
        associated_types: Vec<AssociatedTypeDecl>,
        methods: Vec<TraitMethodSig>,
    },
    ImplBlock {
        type_params: Vec<GenericParam>,
        trait_ref: Option<TypeNode>,
        target_type: TypeNode,
        items: Vec<ImplItem>,
    },
    Operator {
        attributes: Vec<Attribute>,
        operator: String,
        type_params: Vec<GenericParam>,
        params: Vec<(String, TypeNode)>,
        ret_type: TypeNode,
        body: Vec<SpannedStmt>,
    },
    Import(String),
    Mod(String),
    ExternBlock {
        attributes: Vec<Attribute>,
        abi: String,
        functions: Vec<ExternFnDecl>,
    },
    If {
        condition: SpannedExpr,
        then_branch: Vec<SpannedStmt>,
        else_branch: Option<Vec<SpannedStmt>>,
    },
    While {
        condition: SpannedExpr,
        body: Vec<SpannedStmt>,
    },
    Loop {
        body: Vec<SpannedStmt>,
    },
    Break,
    Continue,
    Return(Option<SpannedExpr>),
    Print(Vec<SpannedExpr>),
    Assert(SpannedExpr),
    TestBlock {
        name: String,
        body: Vec<SpannedStmt>,
    },
    Expr(SpannedExpr),
    UnsafeBlock(Vec<SpannedStmt>),
}
