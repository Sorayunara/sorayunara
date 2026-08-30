#![allow(dead_code)]

use crate::ast::*;
use std::collections::HashMap;

/// C symbols already declared by the default include block (stdio.h,
/// stdlib.h, string.h). Re-declaring them via FFI would conflict with
/// the system headers, so prototypes are suppressed for these names.
const DEFAULT_HEADER_SYMBOLS: &[&str] = &[
    // stdio.h
    "printf", "fprintf", "sprintf", "snprintf", "puts", "putchar", "getchar", "fopen", "fclose",
    "fgets", "fputs", "fread", "fwrite", "remove", "rename", // stdlib.h
    "malloc", "calloc", "realloc", "free", "abs", "labs", "rand", "srand", "exit", "abort", "atoi",
    "atol", "atof", "qsort", "bsearch", // string.h
    "strlen", "strcmp", "strncmp", "strcpy", "strncpy", "strcat", "strncat", "memcpy", "memmove",
    "memset", "memcmp", "strstr", "strchr", "strrchr",
];

pub fn emit_c(program: &Program) -> String {
    let mut out = String::new();

    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <stdbool.h>\n");
    out.push_str("#include <string.h>\n\n");

    // FFI declarations: extern "C" blocks become real C prototypes.
    for stmt in &program.statements {
        if let StmtKind::ExternBlock {
            attributes,
            abi,
            functions,
        } = &stmt.kind
        {
            let link_libs: Vec<&String> = attributes
                .iter()
                .filter(|a| a.name == "link")
                .flat_map(|a| a.args.iter())
                .collect();

            out.push_str(&format!("/* FFI: extern \"{}\" */\n", abi));
            for lib in &link_libs {
                out.push_str(&format!(
                    "#pragma comment(lib, \"{lib}\") /* link via: -l{lib} */\n",
                    lib = lib.trim_matches('"')
                ));
            }
            for f in functions {
                // Symbols already declared by the default #include block above
                // must not be re-prototyped, otherwise the C compiler reports
                // conflicting types against the system headers.
                if DEFAULT_HEADER_SYMBOLS.contains(&f.name.as_str()) {
                    out.push_str(&format!("/* {} : provided by system headers */\n", f.name));
                    continue;
                }
                let ret_c = to_c_type(&f.ret_type);
                let param_strs: Vec<String> = if f.params.is_empty() {
                    vec!["void".to_string()]
                } else {
                    f.params
                        .iter()
                        .map(|(p_name, p_ty)| format!("{} {}", to_c_type(p_ty), p_name))
                        .collect()
                };
                out.push_str(&format!(
                    "{} {}({});\n",
                    ret_c,
                    f.name,
                    param_strs.join(", ")
                ));
            }
            out.push('\n');
        }
    }

    // Forward declarations
    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            params,
            ret_type,
            ..
        } = &stmt.kind
        {
            let ret_c = if name == "main" {
                "int".to_string()
            } else {
                to_c_type(ret_type)
            };
            let mut param_types = Vec::new();
            if params.is_empty() {
                param_types.push("void".to_string());
            } else {
                for (p_name, p_ty) in params {
                    param_types.push(format!("{} {}", to_c_type(p_ty), p_name));
                }
            }
            out.push_str(&format!(
                "{} {}({});\n",
                ret_c,
                name,
                param_types.join(", ")
            ));
        }
    }
    out.push('\n');

    // Function definitions
    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            params,
            ret_type,
            body,
            ..
        } = &stmt.kind
        {
            let ret_c = if name == "main" {
                "int".to_string()
            } else {
                to_c_type(ret_type)
            };
            let mut param_types = Vec::new();
            if params.is_empty() {
                param_types.push("void".to_string());
            } else {
                for (p_name, p_ty) in params {
                    param_types.push(format!("{} {}", to_c_type(p_ty), p_name));
                }
            }
            out.push_str(&format!(
                "{} {}({}) {{\n",
                ret_c,
                name,
                param_types.join(", ")
            ));
            let mut var_types: HashMap<String, String> = HashMap::new();
            for (p_name, p_ty) in params {
                var_types.insert(p_name.clone(), to_c_type(p_ty));
            }
            collect_var_types(body, &mut var_types);
            for s in body {
                out.push_str(&emit_stmt_c(s, 1, &var_types));
            }
            if name == "main" {
                out.push_str("    return 0;\n");
            }
            out.push_str("}\n\n");
        }
    }

    out
}

fn to_c_type(ty: &TypeNode) -> String {
    match ty {
        TypeNode::Int => "long long".into(),
        TypeNode::Float => "double".into(),
        TypeNode::Bool => "bool".into(),
        TypeNode::Char => "char".into(),
        TypeNode::String => "const char*".into(),
        TypeNode::Tuple(_) => "void*".into(),
        TypeNode::Array(_) => "void*".into(),
        TypeNode::Slice(_) => "void*".into(),
        TypeNode::Map(_, _) => "void*".into(),
        TypeNode::Set(_) => "void*".into(),
        TypeNode::Union(_) => "void*".into(),
        TypeNode::Option(_) => "void*".into(),
        TypeNode::Result(_, _) => "void*".into(),
        TypeNode::Ref(inner, _) => to_c_type(inner),
        TypeNode::Ptr(inner, is_const) => {
            if *is_const {
                format!("const {}*", to_c_type(inner))
            } else {
                format!("{}*", to_c_type(inner))
            }
        }
        TypeNode::Task(_) => "void*".into(),
        TypeNode::Chan(_) => "void*".into(),
        TypeNode::Function { .. } => "void*".into(),
        TypeNode::Generic { .. } => "void*".into(),
        TypeNode::Custom(_) => "void*".into(),
        TypeNode::Void => "void".into(),
        TypeNode::Infer => "long long".into(),
    }
}

/// Pre-collect declared local variable types within a statement list so
/// printf format specifiers can be chosen correctly for Print statements.
fn collect_var_types(stmts: &[SpannedStmt], var_types: &mut HashMap<String, String>) {
    for s in stmts {
        match &s.kind {
            StmtKind::Let {
                name, type_annot, ..
            } => {
                let ty = type_annot
                    .as_ref()
                    .map(to_c_type)
                    .unwrap_or_else(|| "long long".into());
                var_types.insert(name.clone(), ty);
            }
            StmtKind::Const {
                name, type_annot, ..
            } => {
                let ty = type_annot
                    .as_ref()
                    .map(to_c_type)
                    .unwrap_or_else(|| "long long".into());
                var_types.insert(name.clone(), ty);
            }
            _ => {}
        }
    }
}

fn expr_is_float(expr: &SpannedExpr, var_types: &HashMap<String, String>) -> bool {
    match &expr.kind {
        ExprKind::Float(_) => true,
        ExprKind::Int(_) | ExprKind::Str(_) | ExprKind::Char(_) | ExprKind::Bool(_) => false,
        ExprKind::Var(v) => var_types.get(v).map(|t| t == "double").unwrap_or(false),
        ExprKind::Binary { left, right, .. } => {
            expr_is_float(left, var_types) || expr_is_float(right, var_types)
        }
        ExprKind::Unary { expr, .. } => expr_is_float(expr, var_types),
        _ => false,
    }
}

fn emit_stmt_c(stmt: &SpannedStmt, indent: usize, var_types: &HashMap<String, String>) -> String {
    let pad = "    ".repeat(indent);
    match &stmt.kind {
        StmtKind::StructDecl { .. }
        | StmtKind::EnumDecl { .. }
        | StmtKind::TypeAlias { .. }
        | StmtKind::TraitDecl { .. }
        | StmtKind::ImplBlock { .. }
        | StmtKind::Operator { .. }
        | StmtKind::Import(_)
        | StmtKind::Mod(_)
        | StmtKind::ExternBlock { .. } => String::new(),
        StmtKind::Function { .. } => String::new(),
        StmtKind::LetDestructure { .. } => format!("{}/* destructuring declaration */\n", pad),
        StmtKind::Let {
            name,
            type_annot,
            value,
            ..
        } => {
            let ty = type_annot.as_ref().unwrap_or(&TypeNode::Int);
            format!(
                "{}{} {} = {};\n",
                pad,
                to_c_type(ty),
                name,
                emit_expr_c(value)
            )
        }
        StmtKind::Const {
            name,
            type_annot,
            value,
        } => {
            let ty = type_annot.as_ref().unwrap_or(&TypeNode::Int);
            format!(
                "{}const {} {} = {};\n",
                pad,
                to_c_type(ty),
                name,
                emit_expr_c(value)
            )
        }
        StmtKind::Comptime(body) => {
            let mut s = String::new();
            for st in body {
                s.push_str(&emit_stmt_c(st, indent, var_types));
            }
            s
        }
        StmtKind::Assign { target, value } => {
            format!("{}{} = {};\n", pad, target, emit_expr_c(value))
        }
        StmtKind::AssignIndex { .. } => format!("{}/* array set */;\n", pad),
        StmtKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut s = format!("{}if ({}) {{\n", pad, emit_expr_c(condition));
            for st in then_branch {
                s.push_str(&emit_stmt_c(st, indent + 1, var_types));
            }
            s.push_str(&format!("{}}}", pad));
            if let Some(eb) = else_branch {
                s.push_str(" else {\n");
                for st in eb {
                    s.push_str(&emit_stmt_c(st, indent + 1, var_types));
                }
                s.push_str(&format!("{}}}\n", pad));
            } else {
                s.push('\n');
            }
            s
        }
        StmtKind::While { condition, body } => {
            let mut s = format!("{}while ({}) {{\n", pad, emit_expr_c(condition));
            for st in body {
                s.push_str(&emit_stmt_c(st, indent + 1, var_types));
            }
            s.push_str(&format!("{}}}\n", pad));
            s
        }
        StmtKind::Loop { body } => {
            let mut s = format!("{}while (1) {{\n", pad);
            for st in body {
                s.push_str(&emit_stmt_c(st, indent + 1, var_types));
            }
            s.push_str(&format!("{}}}\n", pad));
            s
        }
        StmtKind::Break => format!("{}break;\n", pad),
        StmtKind::Continue => format!("{}continue;\n", pad),
        StmtKind::Return(Some(expr)) => format!("{}return {};\n", pad, emit_expr_c(expr)),
        StmtKind::Return(None) => format!("{}return;\n", pad),
        StmtKind::Print(args) => {
            let mut formats = String::new();
            let mut exprs = Vec::new();
            for arg in args {
                match &arg.kind {
                    ExprKind::Str(_) => {
                        formats.push_str("%s");
                        exprs.push(emit_expr_c(arg));
                    }
                    ExprKind::Float(_) => {
                        formats.push_str("%f");
                        exprs.push(emit_expr_c(arg));
                    }
                    ExprKind::Char(_) => {
                        formats.push_str("%c");
                        exprs.push(emit_expr_c(arg));
                    }
                    _ if expr_is_float(arg, var_types) => {
                        formats.push_str("%f");
                        exprs.push(emit_expr_c(arg));
                    }
                    _ => {
                        formats.push_str("%lld");
                        exprs.push(emit_expr_c(arg));
                    }
                }
            }
            formats.push_str("\\n");
            let exprs_str = if exprs.is_empty() {
                String::new()
            } else {
                format!(", {}", exprs.join(", "))
            };
            format!("{}printf(\"{}\"{});\n", pad, formats, exprs_str)
        }
        StmtKind::Assert(expr) => format!(
            "{}if (!({})) {{ fprintf(stderr, \"Assertion Failed\\n\"); exit(1); }}\n",
            pad,
            emit_expr_c(expr)
        ),
        StmtKind::TestBlock { .. } => String::new(),
        StmtKind::UnsafeBlock(body) => {
            // Unsafe mode: raw pointer access. Preserve the block structure
            // so native C code can perform unchecked pointer operations.
            let mut s = format!("{}/* unsafe {{ */\n", pad);
            for st in body {
                s.push_str(&emit_stmt_c(st, indent + 1, var_types));
            }
            s.push_str(&format!("{}/* }} */\n", pad));
            s
        }
        StmtKind::Expr(e) => format!("{}{};\n", pad, emit_expr_c(e)),
    }
}

fn emit_expr_c(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Move(inner) => emit_expr_c(inner),
        ExprKind::UnsafeBlock(_) => "/* unsafe expression */ 0".into(),
        _ => emit_expr_c_default(expr),
    }
}

fn emit_expr_c_default(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Int(n) => n.to_string(),
        ExprKind::Float(f) => format!("{:.6}", f),
        ExprKind::Str(s) => format!("\"{}\"", s),
        ExprKind::Char(c) => format!("'{}'", c),
        ExprKind::Bool(b) => {
            if *b {
                "true".into()
            } else {
                "false".into()
            }
        }
        ExprKind::Some(inner) => emit_expr_c(inner),
        ExprKind::None => "NULL".into(),
        ExprKind::Ok(inner) => emit_expr_c(inner),
        ExprKind::Err(inner) => emit_expr_c(inner),
        ExprKind::Null => "NULL".into(),
        ExprKind::Var(v) => v.clone(),
        ExprKind::Borrow { expr, .. } => format!("&{}", emit_expr_c(expr)),
        ExprKind::Task(inner) => emit_expr_c(inner),
        ExprKind::Await(inner) => emit_expr_c(inner),
        ExprKind::Spawn { callee, args } => {
            let arg_strs: Vec<String> = args.iter().map(emit_expr_c).collect();
            format!("{}({})", callee, arg_strs.join(", "))
        }
        ExprKind::MakeChan(_) => "NULL".into(),
        ExprKind::ChanSend { chan: _, value: _ } => "true".into(),
        ExprKind::ChanRecv(chan) => emit_expr_c(chan),
        ExprKind::Binary { left, op, right } => {
            let op_str = match op {
                BinaryOpKind::Add => "+",
                BinaryOpKind::Sub => "-",
                BinaryOpKind::Mul => "*",
                BinaryOpKind::Div => "/",
                BinaryOpKind::Mod => "%",
                BinaryOpKind::Equal => "==",
                BinaryOpKind::NotEqual => "!=",
                BinaryOpKind::Less => "<",
                BinaryOpKind::LessEqual => "<=",
                BinaryOpKind::Greater => ">",
                BinaryOpKind::GreaterEqual => ">=",
                BinaryOpKind::And => "&&",
                BinaryOpKind::Or => "||",
            };
            format!("({} {} {})", emit_expr_c(left), op_str, emit_expr_c(right))
        }
        ExprKind::Unary { op, expr } => {
            let op_str = match op {
                UnaryOpKind::Neg => "-",
                UnaryOpKind::Not => "!",
                UnaryOpKind::Deref => "*",
            };
            format!("({}{})", op_str, emit_expr_c(expr))
        }
        ExprKind::Call { callee, args } => {
            let arg_strs: Vec<String> = args.iter().map(emit_expr_c).collect();
            if callee == "__aether_method::length" {
                format!("strlen({})", arg_strs.first().cloned().unwrap_or_default())
            } else if callee == "println" {
                format!(
                    "printf(\"%s\\n\", {})",
                    arg_strs.first().cloned().unwrap_or_default()
                )
            } else {
                format!("{}({})", callee, arg_strs.join(", "))
            }
        }
        ExprKind::Array(_) => "NULL".into(),
        ExprKind::Map(_) => "NULL".into(),
        ExprKind::Index { target, index: _ } => emit_expr_c(target),
        ExprKind::Dot { target, field: _ } => emit_expr_c(target),
        ExprKind::Match { value, .. } => emit_expr_c(value),
        ExprKind::Block(_) => "0".into(),
        _ => "0".into(),
    }
}

pub fn emit_js(program: &Program) -> String {
    let mut out = String::new();
    let mut has_main = false;

    // FFI bindings for the JS target.
    for stmt in &program.statements {
        if let StmtKind::ExternBlock { functions, .. } = &stmt.kind {
            out.push_str("// FFI (extern \"C\"): native symbol bindings for JS target\n");
            for f in functions {
                if f.params.is_empty() {
                    continue;
                }
                let param_names: Vec<String> = f.params.iter().map(|p| p.0.clone()).collect();
                let body = js_math_binding(&f.name, &param_names).unwrap_or_else(|| {
                    format!(
                        "throw new Error('FFI symbol \"{}\" is not available on the JS target; link a native build instead');",
                        f.name
                    )
                });
                out.push_str(&format!(
                    "function {}({}) {{ {} }}\n",
                    f.name,
                    param_names.join(", "),
                    body
                ));
            }
            out.push('\n');
        }
    }

    for stmt in &program.statements {
        if let StmtKind::Function {
            name,
            is_async,
            params,
            body,
            ..
        } = &stmt.kind
        {
            if name == "main" {
                has_main = true;
            }
            let async_prefix = if *is_async { "async " } else { "" };
            let param_names: Vec<String> = params.iter().map(|p| p.0.clone()).collect();
            out.push_str(&format!(
                "{}function {}({}) {{\n",
                async_prefix,
                name,
                param_names.join(", ")
            ));
            for s in body {
                out.push_str(&emit_stmt_js(s, 1));
            }
            out.push_str("}\n\n");
        } else {
            out.push_str(&emit_stmt_js(stmt, 0));
        }
    }

    if has_main {
        out.push_str("// Entry point\nmain();\n");
    }

    out
}

fn js_math_binding(name: &str, params: &[String]) -> Option<String> {
    let math_fns = [
        "sqrt", "pow", "abs", "floor", "ceil", "round", "sin", "cos", "tan", "asin", "acos",
        "atan", "log", "log2", "exp",
    ];
    if params.len() == 1 && math_fns.contains(&name) {
        return Some(format!("return Math.{}({});", name, params[0]));
    }
    if name == "fabs" && params.len() == 1 {
        return Some(format!("return Math.abs({});", params[0]));
    }
    if name == "atan2" && params.len() == 2 {
        return Some(format!("return Math.atan2({}, {});", params[0], params[1]));
    }
    if name == "strlen" && params.len() == 1 {
        return Some(format!("return {}.length;", params[0]));
    }
    None
}

fn emit_stmt_js(stmt: &SpannedStmt, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match &stmt.kind {
        StmtKind::StructDecl { .. }
        | StmtKind::EnumDecl { .. }
        | StmtKind::TypeAlias { .. }
        | StmtKind::TraitDecl { .. }
        | StmtKind::ImplBlock { .. }
        | StmtKind::Operator { .. }
        | StmtKind::Import(_)
        | StmtKind::Mod(_)
        | StmtKind::ExternBlock { .. } => String::new(),
        StmtKind::Function { .. } => String::new(),
        StmtKind::LetDestructure { .. } => format!("{}/* destructuring declaration */\n", pad),
        StmtKind::Let { name, value, .. } => {
            format!("{}let {} = {};\n", pad, name, emit_expr_js(value))
        }
        StmtKind::Const { name, value, .. } => {
            format!("{}const {} = {};\n", pad, name, emit_expr_js(value))
        }
        StmtKind::Comptime(body) => {
            let mut s = String::new();
            for st in body {
                s.push_str(&emit_stmt_js(st, indent));
            }
            s
        }
        StmtKind::Assign { target, value } => {
            format!("{}{} = {};\n", pad, target, emit_expr_js(value))
        }
        StmtKind::AssignIndex {
            target,
            index,
            value,
        } => {
            format!(
                "{}{}[{}] = {};\n",
                pad,
                emit_expr_js(target),
                emit_expr_js(index),
                emit_expr_js(value)
            )
        }
        StmtKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut s = format!("{}if ({}) {{\n", pad, emit_expr_js(condition));
            for st in then_branch {
                s.push_str(&emit_stmt_js(st, indent + 1));
            }
            s.push_str(&format!("{}}}", pad));
            if let Some(eb) = else_branch {
                s.push_str(" else {\n");
                for st in eb {
                    s.push_str(&emit_stmt_js(st, indent + 1));
                }
                s.push_str(&format!("{}}}\n", pad));
            } else {
                s.push('\n');
            }
            s
        }
        StmtKind::While { condition, body } => {
            let mut s = format!("{}while ({}) {{\n", pad, emit_expr_js(condition));
            for st in body {
                s.push_str(&emit_stmt_js(st, indent + 1));
            }
            s.push_str(&format!("{}}}\n", pad));
            s
        }
        StmtKind::Loop { body } => {
            let mut s = format!("{}while (true) {{\n", pad);
            for st in body {
                s.push_str(&emit_stmt_js(st, indent + 1));
            }
            s.push_str(&format!("{}}}\n", pad));
            s
        }
        StmtKind::Break => format!("{}break;\n", pad),
        StmtKind::Continue => format!("{}continue;\n", pad),
        StmtKind::Return(Some(expr)) => format!("{}return {};\n", pad, emit_expr_js(expr)),
        StmtKind::Return(None) => format!("{}return;\n", pad),
        StmtKind::Print(args) => {
            let args_js: Vec<String> = args.iter().map(emit_expr_js).collect();
            format!("{}console.log({});\n", pad, args_js.join(", "))
        }
        StmtKind::Assert(expr) => format!(
            "{}if (!({})) throw new Error(\"Assertion Failed\");\n",
            pad,
            emit_expr_js(expr)
        ),
        StmtKind::TestBlock { .. } => String::new(),
        StmtKind::UnsafeBlock(body) => {
            let mut s = format!("{}// unsafe {{\n", pad);
            for st in body {
                s.push_str(&emit_stmt_js(st, indent + 1));
            }
            s.push_str(&format!("{}// }}\n", pad));
            s
        }
        StmtKind::Expr(e) => format!("{}{};\n", pad, emit_expr_js(e)),
    }
}

fn emit_expr_js(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Move(inner) => emit_expr_js(inner),
        ExprKind::UnsafeBlock(_) => "null /* unsafe expression */".into(),
        _ => emit_expr_js_default(expr),
    }
}

fn emit_expr_js_default(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Int(n) => n.to_string(),
        ExprKind::Float(f) => f.to_string(),
        ExprKind::Str(s) => format!("\"{}\"", s),
        ExprKind::Char(c) => format!("\"{}\"", c),
        ExprKind::Bool(b) => b.to_string(),
        ExprKind::Some(inner) => format!("{{ isSome: true, value: {} }}", emit_expr_js(inner)),
        ExprKind::None => "{ isSome: false }".into(),
        ExprKind::Ok(inner) => format!("{{ isOk: true, value: {} }}", emit_expr_js(inner)),
        ExprKind::Err(inner) => format!("{{ isOk: false, error: {} }}", emit_expr_js(inner)),
        ExprKind::Null => "null".into(),
        ExprKind::Var(v) => v.clone(),
        ExprKind::Borrow { expr, .. } => emit_expr_js(expr),
        ExprKind::Task(inner) => format!("(async () => {})()", emit_expr_js(inner)),
        ExprKind::Await(inner) => format!("(await {})", emit_expr_js(inner)),
        ExprKind::Spawn { callee, args } => {
            let arg_strs: Vec<String> = args.iter().map(emit_expr_js).collect();
            format!("(async () => {}({}))()", callee, arg_strs.join(", "))
        }
        ExprKind::MakeChan(_) => "[]".into(),
        ExprKind::ChanSend { chan, value } => {
            format!("{}.push({})", emit_expr_js(chan), emit_expr_js(value))
        }
        ExprKind::ChanRecv(chan) => format!("{}.shift()", emit_expr_js(chan)),
        ExprKind::Binary { left, op, right } => {
            let op_str = match op {
                BinaryOpKind::Add => "+",
                BinaryOpKind::Sub => "-",
                BinaryOpKind::Mul => "*",
                BinaryOpKind::Div => "/",
                BinaryOpKind::Mod => "%",
                BinaryOpKind::Equal => "===",
                BinaryOpKind::NotEqual => "!==",
                BinaryOpKind::Less => "<",
                BinaryOpKind::LessEqual => "<=",
                BinaryOpKind::Greater => ">",
                BinaryOpKind::GreaterEqual => ">=",
                BinaryOpKind::And => "&&",
                BinaryOpKind::Or => "||",
            };
            format!(
                "({} {} {})",
                emit_expr_js(left),
                op_str,
                emit_expr_js(right)
            )
        }
        ExprKind::Unary { op, expr } => {
            let op_str = match op {
                UnaryOpKind::Neg => "-",
                UnaryOpKind::Not => "!",
                UnaryOpKind::Deref => "",
            };
            format!("({}{})", op_str, emit_expr_js(expr))
        }
        ExprKind::Call { callee, args } => {
            let arg_strs: Vec<String> = args.iter().map(emit_expr_js).collect();
            if callee == "__aether_method::length" {
                format!("{}.length", arg_strs.first().cloned().unwrap_or_default())
            } else if callee == "println" {
                format!("console.log({})", arg_strs.join(", "))
            } else {
                format!("{}({})", callee, arg_strs.join(", "))
            }
        }
        ExprKind::Array(elements) => {
            let elems: Vec<String> = elements.iter().map(emit_expr_js).collect();
            format!("[{}]", elems.join(", "))
        }
        ExprKind::Map(entries) => {
            let kvs: Vec<String> = entries
                .iter()
                .map(|(k, v)| format!("{}: {}", emit_expr_js(k), emit_expr_js(v)))
                .collect();
            format!("{{{}}}", kvs.join(", "))
        }
        ExprKind::Index { target, index } => {
            format!("{}[{}]", emit_expr_js(target), emit_expr_js(index))
        }
        ExprKind::Dot { target, field } => {
            format!("{}.{}", emit_expr_js(target), field)
        }
        ExprKind::Match { value, arms } => {
            let mut s = format!("((__val) => {{\n");
            for arm in arms {
                s.push_str(&format!("  return {};\n", emit_expr_js(&arm.body)));
            }
            s.push_str(&format!("}})({})", emit_expr_js(value)));
            s
        }
        ExprKind::Block(stmts) => {
            let mut s = "(() => {\n".to_string();
            for st in stmts {
                s.push_str(&emit_stmt_js(st, 1));
            }
            s.push_str("})()");
            s
        }
        _ => "null".into(),
    }
}
