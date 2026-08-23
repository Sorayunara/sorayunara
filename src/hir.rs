#![allow(dead_code)]

use crate::ast::*;
use crate::symbol_table::Type;

#[derive(Debug, Clone)]
pub struct HirProgram {
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: String,
    pub is_async: bool,
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let {
        name: String,
        ty: Type,
        value: HirExpr,
    },
    Assign {
        target: String,
        value: HirExpr,
    },
    If {
        cond: HirExpr,
        then_branch: Vec<HirStmt>,
        else_branch: Option<Vec<HirStmt>>,
    },
    While {
        cond: HirExpr,
        body: Vec<HirStmt>,
    },
    Return(Option<HirExpr>),
    Print(Vec<HirExpr>),
    Expr(HirExpr),
}

#[derive(Debug, Clone)]
pub struct HirExpr {
    pub kind: HirExprKind,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Var(String),
    Borrow(Box<HirExpr>, bool),
    Binary(Box<HirExpr>, BinaryOpKind, Box<HirExpr>),
    Call(String, Vec<HirExpr>),
    Spawn(String, Vec<HirExpr>),
    Await(Box<HirExpr>),
    Null,
}

pub fn lower_ast_to_hir(program: &Program) -> HirProgram {
    let mut functions = Vec::new();

    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            is_async,
            params,
            body,
            ..
        } = &stmt.kind
        {
            let mut hir_params = Vec::new();
            for (p_name, _) in params {
                hir_params.push((p_name.clone(), Type::Int));
            }

            let mut hir_body = Vec::new();
            for s in body {
                hir_body.push(lower_stmt(s));
            }

            functions.push(HirFunction {
                name: name.clone(),
                is_async: *is_async,
                params: hir_params,
                ret_type: Type::Int,
                body: hir_body,
            });
        }
    }

    HirProgram { functions }
}

fn lower_stmt(stmt: &SpannedStmt) -> HirStmt {
    match &stmt.kind {
        StmtKind::Let { name, value, .. } => HirStmt::Let {
            name: name.clone(),
            ty: Type::Int,
            value: lower_expr(value),
        },
        StmtKind::Const { name, value, .. } => HirStmt::Let {
            name: name.clone(),
            ty: Type::Int,
            value: lower_expr(value),
        },
        StmtKind::Assign { target, value } => HirStmt::Assign {
            target: target.clone(),
            value: lower_expr(value),
        },
        StmtKind::Return(val) => HirStmt::Return(val.as_ref().map(lower_expr)),
        StmtKind::Print(args) => HirStmt::Print(args.iter().map(lower_expr).collect()),
        StmtKind::Expr(e) => HirStmt::Expr(lower_expr(e)),
        _ => HirStmt::Expr(HirExpr {
            kind: HirExprKind::Null,
            ty: Type::Void,
        }),
    }
}

fn lower_expr(expr: &SpannedExpr) -> HirExpr {
    match &expr.kind {
        ExprKind::Int(n) => HirExpr {
            kind: HirExprKind::Int(*n),
            ty: Type::Int,
        },
        ExprKind::Float(f) => HirExpr {
            kind: HirExprKind::Float(*f),
            ty: Type::Float,
        },
        ExprKind::Str(s) => HirExpr {
            kind: HirExprKind::Str(s.clone()),
            ty: Type::String,
        },
        ExprKind::Bool(b) => HirExpr {
            kind: HirExprKind::Bool(*b),
            ty: Type::Bool,
        },
        ExprKind::Var(v) => HirExpr {
            kind: HirExprKind::Var(v.clone()),
            ty: Type::Any,
        },
        ExprKind::Borrow { expr, is_mut } => HirExpr {
            kind: HirExprKind::Borrow(Box::new(lower_expr(expr)), *is_mut),
            ty: Type::Ref(Box::new(Type::Any), *is_mut),
        },
        ExprKind::Binary { left, op, right } => HirExpr {
            kind: HirExprKind::Binary(Box::new(lower_expr(left)), *op, Box::new(lower_expr(right))),
            ty: Type::Int,
        },
        ExprKind::Call { callee, args } => HirExpr {
            kind: HirExprKind::Call(callee.clone(), args.iter().map(lower_expr).collect()),
            ty: Type::Any,
        },
        ExprKind::Spawn { callee, args } => HirExpr {
            kind: HirExprKind::Spawn(callee.clone(), args.iter().map(lower_expr).collect()),
            ty: Type::Task(Box::new(Type::Any)),
        },
        ExprKind::Await(e) => HirExpr {
            kind: HirExprKind::Await(Box::new(lower_expr(e))),
            ty: Type::Any,
        },
        _ => HirExpr {
            kind: HirExprKind::Null,
            ty: Type::Void,
        },
    }
}
