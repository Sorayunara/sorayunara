#![allow(dead_code)]

use crate::hir::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MirProgram {
    pub functions: HashMap<String, MirBody>,
}

#[derive(Debug, Clone)]
pub struct MirBody {
    pub basic_blocks: Vec<BasicBlockData>,
}

#[derive(Debug, Clone)]
pub struct BasicBlockData {
    pub statements: Vec<MirStatement>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum MirStatement {
    Assign(String, MirRvalue),
    StorageLive(String),
    StorageDead(String),
    Nop,
}

#[derive(Debug, Clone)]
pub enum MirRvalue {
    Use(MirOperand),
    BinaryOp(crate::ast::BinaryOpKind, MirOperand, MirOperand),
    Ref(String, bool),
    Call(String, Vec<MirOperand>),
}

#[derive(Debug, Clone)]
pub enum MirOperand {
    Copy(String),
    Move(String),
    ConstantInt(i64),
    ConstantFloat(f64),
    ConstantStr(String),
    ConstantBool(bool),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Goto(usize),
    SwitchInt {
        discr: MirOperand,
        targets: Vec<(i64, usize)>,
        otherwise: usize,
    },
    Return,
    Unreachable,
}

pub fn lower_hir_to_mir(hir: &HirProgram) -> MirProgram {
    let mut mir_functions = HashMap::new();

    for func in &hir.functions {
        let mut stmts = Vec::new();

        for s in &func.body {
            match s {
                HirStmt::Let { name, value, .. } => {
                    stmts.push(MirStatement::StorageLive(name.clone()));
                    stmts.push(MirStatement::Assign(
                        name.clone(),
                        lower_hir_expr_to_rvalue(value),
                    ));
                }
                HirStmt::Assign { target, value } => {
                    stmts.push(MirStatement::Assign(
                        target.clone(),
                        lower_hir_expr_to_rvalue(value),
                    ));
                }
                _ => {}
            }
        }

        let entry_bb = BasicBlockData {
            statements: stmts,
            terminator: Terminator::Return,
        };

        mir_functions.insert(
            func.name.clone(),
            MirBody {
                basic_blocks: vec![entry_bb],
            },
        );
    }

    MirProgram {
        functions: mir_functions,
    }
}

fn lower_hir_expr_to_rvalue(expr: &HirExpr) -> MirRvalue {
    match &expr.kind {
        HirExprKind::Int(n) => MirRvalue::Use(MirOperand::ConstantInt(*n)),
        HirExprKind::Float(f) => MirRvalue::Use(MirOperand::ConstantFloat(*f)),
        HirExprKind::Str(s) => MirRvalue::Use(MirOperand::ConstantStr(s.clone())),
        HirExprKind::Bool(b) => MirRvalue::Use(MirOperand::ConstantBool(*b)),
        HirExprKind::Var(v) => MirRvalue::Use(MirOperand::Copy(v.clone())),
        HirExprKind::Borrow(inner, is_mut) => {
            if let HirExprKind::Var(v) = &inner.kind {
                MirRvalue::Ref(v.clone(), *is_mut)
            } else {
                MirRvalue::Use(MirOperand::ConstantInt(0))
            }
        }
        HirExprKind::Binary(l, op, r) => {
            let l_op = lower_hir_expr_to_operand(l);
            let r_op = lower_hir_expr_to_operand(r);
            MirRvalue::BinaryOp(*op, l_op, r_op)
        }
        HirExprKind::Call(callee, args) => {
            let ops: Vec<MirOperand> = args.iter().map(lower_hir_expr_to_operand).collect();
            MirRvalue::Call(callee.clone(), ops)
        }
        _ => MirRvalue::Use(MirOperand::ConstantInt(0)),
    }
}

fn lower_hir_expr_to_operand(expr: &HirExpr) -> MirOperand {
    match &expr.kind {
        HirExprKind::Int(n) => MirOperand::ConstantInt(*n),
        HirExprKind::Float(f) => MirOperand::ConstantFloat(*f),
        HirExprKind::Str(s) => MirOperand::ConstantStr(s.clone()),
        HirExprKind::Bool(b) => MirOperand::ConstantBool(*b),
        HirExprKind::Var(v) => MirOperand::Copy(v.clone()),
        _ => MirOperand::ConstantInt(0),
    }
}
