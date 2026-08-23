#![allow(dead_code)]

use crate::ast::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    LinuxX64,
    LinuxArm64,
    WindowsX64,
    WindowsArm64,
    MacosArm64,
    MacosX64,
    Wasm32,
    Riscv64,
    ArmCortexM,
    ArmCortexA,
    Riscv32,
    Esp32,
    Embedded,
}

impl Target {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "linux-x64" | "linux-x86_64" | "x86_64-linux" => Some(Target::LinuxX64),
            "linux-arm64" | "linux-aarch64" | "aarch64-linux" => Some(Target::LinuxArm64),
            "windows-x64" | "windows-x86_64" | "x86_64-windows" => Some(Target::WindowsX64),
            "windows-arm64" | "aarch64-windows" => Some(Target::WindowsArm64),
            "macos-arm64" | "darwin-arm64" | "arm64-macos" => Some(Target::MacosArm64),
            "macos-x64" | "darwin-x64" | "x86_64-macos" => Some(Target::MacosX64),
            "wasm" | "wasm32" | "wasm32-wasi" => Some(Target::Wasm32),
            "riscv64" | "riscv" | "linux-riscv64" => Some(Target::Riscv64),
            "arm-cortex-m" | "cortex-m" | "thumbv7em" | "thumbv6m" => Some(Target::ArmCortexM),
            "arm-cortex-a" | "cortex-a" | "aarch64-baremetal" => Some(Target::ArmCortexA),
            "riscv32" | "riscv32imc" => Some(Target::Riscv32),
            "esp32" | "xtensa-esp32" => Some(Target::Esp32),
            "embedded" | "baremetal" | "no-std" => Some(Target::Embedded),
            _ => None,
        }
    }

    pub fn triple(&self) -> &'static str {
        match self {
            Target::LinuxX64 => "x86_64-unknown-linux-gnu",
            Target::LinuxArm64 => "aarch64-unknown-linux-gnu",
            Target::WindowsX64 => "x86_64-pc-windows-msvc",
            Target::WindowsArm64 => "aarch64-pc-windows-msvc",
            Target::MacosArm64 => "arm64-apple-macosx",
            Target::MacosX64 => "x86_64-apple-darwin",
            Target::Wasm32 => "wasm32-unknown-wasi",
            Target::Riscv64 => "riscv64gc-unknown-linux-gnu",
            Target::ArmCortexM => "thumbv7em-none-eabihf",
            Target::ArmCortexA => "aarch64-unknown-none-elf",
            Target::Riscv32 => "riscv32imac-unknown-none-elf",
            Target::Esp32 => "xtensa-esp32-none-elf",
            Target::Embedded => "thumbv7m-none-eabi",
        }
    }

    pub fn datalayout(&self) -> &'static str {
        match self {
            Target::LinuxX64 | Target::WindowsX64 | Target::MacosX64 => {
                "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            }
            Target::LinuxArm64 | Target::WindowsArm64 | Target::MacosArm64 | Target::ArmCortexA => {
                "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
            }
            Target::Wasm32 => "e-m:e-p:32:32-i64:64-n32:64-S128",
            Target::Riscv64 => "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128",
            Target::ArmCortexM | Target::Embedded => "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
            Target::Riscv32 | Target::Esp32 => "e-m:e-p:32:32-i64:64-n32-S32",
        }
    }
}

pub fn emit_llvm_ir(program: &Program) -> String {
    emit_llvm_ir_with_target(program, Target::WindowsX64)
}

pub fn emit_llvm_ir_with_target(program: &Program, target: Target) -> String {
    let mut ir = String::new();

    ir.push_str("; ModuleID = 'sorayunara_module'\n");
    ir.push_str("source_filename = \"main.sora\"\n");
    ir.push_str(&format!("target datalayout = \"{}\"\n", target.datalayout()));
    ir.push_str(&format!("target triple = \"{}\"\n\n", target.triple()));

    ir.push_str("@.str_fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n");
    ir.push_str("@.str_fmt_s = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\", align 1\n\n");
    ir.push_str("declare i32 @printf(i8*, ...)\n");

    // FFI: extern "C" blocks become LLVM declarations resolved by the linker.
    for stmt in &program.statements {
        if let StmtKind::ExternBlock { functions, .. } = &stmt.kind {
            ir.push('\n');
            for f in functions {
                let ret_ty = to_llvm_type(&f.ret_type);
                let param_strs: Vec<String> = if f.params.is_empty() {
                    vec!["void".to_string()]
                } else {
                    f.params
                        .iter()
                        .map(|(_, p_ty)| to_llvm_type(p_ty).to_string())
                        .collect()
                };
                ir.push_str(&format!(
                    "declare {} @{}({})\n",
                    ret_ty,
                    f.name,
                    param_strs.join(", ")
                ));
            }
        }
    }

    ir.push('\n');

    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            params,
            ret_type,
            body,
            ..
        } = &stmt.kind
        {
            let ret_ty = to_llvm_type(ret_type);
            let mut param_strs = Vec::new();
            for (p_name, p_ty) in params {
                param_strs.push(format!("{} %{}", to_llvm_type(p_ty), p_name));
            }

            ir.push_str(&format!("define {} @{}({}) {{\n", ret_ty, name, param_strs.join(", ")));
            ir.push_str("entry:\n");

            for s in body {
                ir.push_str(&emit_stmt_llvm(s));
            }

            if *ret_type == TypeNode::Void {
                ir.push_str("  ret void\n");
            }

            ir.push_str("}\n\n");
        }
    }

    ir
}

fn to_llvm_type(ty: &TypeNode) -> &'static str {
    match ty {
        TypeNode::Int => "i64",
        TypeNode::Float => "double",
        TypeNode::Bool => "i1",
        TypeNode::Char => "i8",
        TypeNode::String => "i8*",
        TypeNode::Void => "void",
        _ => "i64",
    }
}

fn emit_stmt_llvm(stmt: &SpannedStmt) -> String {
    let mut out = String::new();
    match &stmt.kind {
        StmtKind::Return(Some(expr)) => {
            let (val, ty) = emit_expr_llvm(expr, &mut out);
            out.push_str(&format!("  ret {} {}\n", ty, val));
        }
        StmtKind::Return(None) => {
            out.push_str("  ret void\n");
        }
        StmtKind::Print(args) => {
            for arg in args {
                let (val, ty) = emit_expr_llvm(arg, &mut out);
                if ty == "i8*" {
                    out.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str_fmt_s, i32 0, i32 0), i8* {})\n", val));
                } else {
                    out.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt, i32 0, i32 0), i64 {})\n", val));
                }
            }
        }
        StmtKind::Expr(expr) => {
            let _ = emit_expr_llvm(expr, &mut out);
        }
        _ => {}
    }
    out
}

fn emit_expr_llvm(expr: &SpannedExpr, out: &mut String) -> (String, &'static str) {
    match &expr.kind {
        ExprKind::Int(n) => (n.to_string(), "i64"),
        ExprKind::Float(f) => (f.to_string(), "double"),
        ExprKind::Bool(b) => (if *b { "1".into() } else { "0".into() }, "i1"),
        ExprKind::Var(v) => (format!("%{}", v), "i64"),
        ExprKind::Binary { left, op, right } => {
            let (l_val, _) = emit_expr_llvm(left, out);
            let (r_val, _) = emit_expr_llvm(right, out);
            let temp = format!("%t_{}", expr.span.start);
            let op_str = match op {
                BinaryOpKind::Add => "add i64",
                BinaryOpKind::Sub => "sub i64",
                BinaryOpKind::Mul => "mul i64",
                BinaryOpKind::Div => "sdiv i64",
                BinaryOpKind::Mod => "srem i64",
                _ => "add i64",
            };
            out.push_str(&format!("  {} = {} {}, {}\n", temp, op_str, l_val, r_val));
            (temp, "i64")
        }
        ExprKind::Call { callee, args } => {
            let mut arg_items = Vec::new();
            for arg in args {
                let (val, ty) = emit_expr_llvm(arg, out);
                arg_items.push(format!("{} {}", ty, val));
            }
            let temp = format!("%c_{}", expr.span.start);
            out.push_str(&format!("  {} = call i64 @{}({})\n", temp, callee, arg_items.join(", ")));
            (temp, "i64")
        }
        _ => ("0".to_string(), "i64"),
    }
}
