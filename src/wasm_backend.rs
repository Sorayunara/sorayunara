#![allow(dead_code)]

use crate::ast::*;

pub fn emit_wat(program: &Program) -> String {
    let mut wat = String::new();

    wat.push_str("(module\n");
    wat.push_str("  ;; Aether Standard WebAssembly Runtime Imports\n");
    wat.push_str("  (import \"env\" \"print_i64\" (func $print_i64 (param i64)))\n");
    wat.push_str("  (memory (export \"memory\") 1)\n\n");

    // FFI: extern symbols become WASM host imports resolved by the embedder.
    for stmt in &program.statements {
        if let StmtKind::ExternBlock { abi, functions, .. } = &stmt.kind {
            wat.push_str(&format!("  ;; FFI: extern \"{}\"\n", abi));
            for f in functions {
                let mut sig = String::new();
                for (p_name, _) in &f.params {
                    sig.push_str(&format!(" (param ${} i64)", p_name));
                }
                if f.ret_type != TypeNode::Void {
                    sig.push_str(" (result i64)");
                }
                wat.push_str(&format!(
                    "  (import \"env\" \"{}\" (func ${}{}))\n",
                    f.name, f.name, sig
                ));
            }
            wat.push('\n');
        }
    }

    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            params,
            ret_type,
            body,
            ..
        } = &stmt.kind
        {
            let mut p_sigs = String::new();
            for (p_name, _) in params {
                p_sigs.push_str(&format!(" (param ${} i64)", p_name));
            }

            let ret_sig = if *ret_type != TypeNode::Void {
                " (result i64)"
            } else {
                ""
            };

            wat.push_str(&format!("  (func ${} (export \"{}\"){}{}\n", name, name, p_sigs, ret_sig));

            for s in body {
                wat.push_str(&emit_stmt_wat(s, 2));
            }

            wat.push_str("  )\n\n");
        }
    }

    wat.push_str(")\n");
    wat
}

fn emit_stmt_wat(stmt: &SpannedStmt, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match &stmt.kind {
        StmtKind::Return(Some(expr)) => {
            format!("{}{}\n{}return\n", pad, emit_expr_wat(expr), pad)
        }
        StmtKind::Return(None) => format!("{}return\n", pad),
        StmtKind::Let { name, value, .. } => {
            format!("{}(local.set ${} {})\n", pad, name, emit_expr_wat(value))
        }
        StmtKind::Assign { target, value } => {
            format!("{}(local.set ${} {})\n", pad, target, emit_expr_wat(value))
        }
        StmtKind::Print(args) => {
            let mut s = String::new();
            for arg in args {
                s.push_str(&format!("{}{}\n{}call $print_i64\n", pad, emit_expr_wat(arg), pad));
            }
            s
        }
        StmtKind::Expr(expr) => format!("{}{}\n", pad, emit_expr_wat(expr)),
        _ => String::new(),
    }
}

fn emit_expr_wat(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Int(n) => format!("(i64.const {})", n),
        ExprKind::Float(f) => format!("(f64.const {})", f),
        ExprKind::Bool(b) => format!("(i64.const {})", if *b { 1 } else { 0 }),
        ExprKind::Var(v) => format!("(local.get ${})", v),
        ExprKind::Binary { left, op, right } => {
            let op_wat = match op {
                BinaryOpKind::Add => "i64.add",
                BinaryOpKind::Sub => "i64.sub",
                BinaryOpKind::Mul => "i64.mul",
                BinaryOpKind::Div => "i64.div_s",
                BinaryOpKind::Equal => "i64.eq",
                BinaryOpKind::NotEqual => "i64.ne",
                BinaryOpKind::Less => "i64.lt_s",
                BinaryOpKind::LessEqual => "i64.le_s",
                BinaryOpKind::Greater => "i64.gt_s",
                BinaryOpKind::GreaterEqual => "i64.ge_s",
                _ => "i64.add",
            };
            format!("({} {} {})", op_wat, emit_expr_wat(left), emit_expr_wat(right))
        }
        ExprKind::Call { callee, args } => {
            let a_strs: Vec<String> = args.iter().map(emit_expr_wat).collect();
            format!("(call ${} {})", callee, a_strs.join(" "))
        }
        _ => "(i64.const 0)".to_string(),
    }
}
