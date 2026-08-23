#![allow(dead_code)]

use crate::diagnostic::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeNode {
    Int,
    Float,
    Bool,
    String,
    Char,
    Tuple(Vec<TypeNode>),
    Array(Box<TypeNode>),
    Slice(Box<TypeNode>),
    Map(Box<TypeNode>, Box<TypeNode>),
    Set(Box<TypeNode>),
    Union(Vec<TypeNode>),
    Option(Box<TypeNode>),
    Result(Box<TypeNode>, Box<TypeNode>),
    Function {
        params: Vec<TypeNode>,
        ret: Box<TypeNode>,
    },
    Generic {
        name: String,
        args: Vec<TypeNode>,
    },
    Ref(Box<TypeNode>, bool), // &T or &mut T
    Ptr(Box<TypeNode>, bool), // *const T / *mut T
    Task(Box<TypeNode>),      // Task<T>
    Chan(Box<TypeNode>),      // Chan<T>
    Custom(String),
    Void,
    Infer,
}

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

/// Memory model modes:
/// - `Managed`: GC-managed / reference-counted by default (safe, zero-cost).
/// - `Owned`: Move semantics with single-owner, no runtime GC.
/// - `Unsafe`: Raw pointer access; unchecked operations in `unsafe` blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMode {
    Managed,
    Owned,
    Unsafe,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedExpr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TypeNode>,
}

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
    // Memory Model Expressions
    Move(Box<SpannedExpr>),
    UnsafeBlock(Vec<SpannedStmt>),
    // Concurrency Expressions
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

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedStmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
    pub span: Span,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<SpannedStmt>,
}