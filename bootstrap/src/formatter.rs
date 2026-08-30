#![allow(dead_code)]

use crate::ast::*;
use crate::lexer;
use crate::parser;

pub fn format_source(source: &str) -> Result<String, String> {
    let tokens = lexer::tokenize(source).map_err(|e| e.0)?;
    let program = parser::parse(tokens).map_err(|e| e.0)?;

    let mut out = String::new();
    for (i, stmt) in program.statements.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&format_stmt(stmt, 0));
        out.push('\n');
    }

    Ok(out)
}

fn format_type(ty: &TypeNode) -> String {
    match ty {
        TypeNode::Int => "Int".to_string(),
        TypeNode::Float => "Float".to_string(),
        TypeNode::Bool => "Bool".to_string(),
        TypeNode::String => "String".to_string(),
        TypeNode::Char => "Char".to_string(),
        TypeNode::Tuple(items) => format!(
            "({})",
            items.iter().map(format_type).collect::<Vec<_>>().join(", ")
        ),
        TypeNode::Void => "Void".to_string(),
        TypeNode::Infer => "".to_string(),
        TypeNode::Array(inner) => format!("[{}]", format_type(inner)),
        TypeNode::Slice(inner) => format!("Slice<{}>", format_type(inner)),
        TypeNode::Map(k, v) => format!("Map<{}, {}>", format_type(k), format_type(v)),
        TypeNode::Set(inner) => format!("Set<{}>", format_type(inner)),
        TypeNode::Union(items) => items
            .iter()
            .map(format_type)
            .collect::<Vec<_>>()
            .join(" | "),
        TypeNode::Option(inner) => format!("Option<{}>", format_type(inner)),
        TypeNode::Result(ok, err) => format!("Result<{}, {}>", format_type(ok), format_type(err)),
        TypeNode::Ref(inner, is_mut) => {
            if *is_mut {
                format!("&mut {}", format_type(inner))
            } else {
                format!("&{}", format_type(inner))
            }
        }
        TypeNode::Ptr(inner, is_const) => {
            if *is_const {
                format!("*const {}", format_type(inner))
            } else {
                format!("*mut {}", format_type(inner))
            }
        }
        TypeNode::Task(inner) => format!("Task<{}>", format_type(inner)),
        TypeNode::Chan(inner) => format!("Chan<{}>", format_type(inner)),
        TypeNode::Function { params, ret } => format!(
            "fn({}) -> {}",
            params
                .iter()
                .map(format_type)
                .collect::<Vec<_>>()
                .join(", "),
            format_type(ret)
        ),
        TypeNode::Generic { name, args } => format!(
            "{}<{}>",
            name,
            args.iter().map(format_type).collect::<Vec<_>>().join(", ")
        ),
        TypeNode::Custom(name) => name.clone(),
    }
}

fn format_generic_params(params: &[GenericParam]) -> String {
    if params.is_empty() {
        return String::new();
    }

    let params = params
        .iter()
        .map(|param| {
            if param.bounds.is_empty() {
                param.name.clone()
            } else {
                format!(
                    "{}: {}",
                    param.name,
                    param
                        .bounds
                        .iter()
                        .map(format_type)
                        .collect::<Vec<_>>()
                        .join(" + ")
                )
            }
        })
        .collect::<Vec<_>>();
    format!("<{}>", params.join(", "))
}

fn format_stmt(stmt: &SpannedStmt, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    match &stmt.kind {
        StmtKind::Import(path) => format!("{}import {}", pad, path),
        StmtKind::Mod(name) => format!("{}mod {}", pad, name),
        StmtKind::ExternBlock {
            attributes,
            abi,
            functions,
        } => {
            let mut s = String::new();
            for attr in attributes {
                if attr.args.is_empty() {
                    s.push_str(&format!("{}@{}\n", pad, attr.name));
                } else {
                    s.push_str(&format!(
                        "{}@{}({})\n",
                        pad,
                        attr.name,
                        attr.args.join(", ")
                    ));
                }
            }
            s.push_str(&format!("{}extern \"{}\" {{\n", pad, abi));
            for f in functions {
                let p_strs: Vec<String> = f
                    .params
                    .iter()
                    .map(|(p, t)| format!("{}: {}", p, format_type(t)))
                    .collect();
                let ret_str = if f.ret_type != TypeNode::Void {
                    format!(" -> {}", format_type(&f.ret_type))
                } else {
                    String::new()
                };
                s.push_str(&format!(
                    "{}    fn {}({}){}\n",
                    pad,
                    f.name,
                    p_strs.join(", "),
                    ret_str
                ));
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::StructDecl {
            attributes,
            name,
            type_params,
            fields,
        } => {
            let mut s = String::new();
            for attr in attributes {
                if attr.args.is_empty() {
                    s.push_str(&format!("{}@{}\n", pad, attr.name));
                } else {
                    s.push_str(&format!(
                        "{}@{}({})\n",
                        pad,
                        attr.name,
                        attr.args.join(", ")
                    ));
                }
            }
            s.push_str(&format!(
                "{}struct {}{} {{\n",
                pad,
                name,
                format_generic_params(type_params)
            ));
            for (f_name, f_ty) in fields {
                s.push_str(&format!("{}    {}: {},\n", pad, f_name, format_type(f_ty)));
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::EnumDecl {
            name,
            type_params,
            variants,
        } => {
            let mut s = format!(
                "{}enum {}{} {{\n",
                pad,
                name,
                format_generic_params(type_params)
            );
            for (v_name, payload) in variants {
                if let Some(p) = payload {
                    s.push_str(&format!("{}    {}({}),\n", pad, v_name, format_type(p)));
                } else {
                    s.push_str(&format!("{}    {},\n", pad, v_name));
                }
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::TypeAlias { name, target, .. } => {
            format!("{}type {} = {}", pad, name, format_type(target))
        }
        StmtKind::TraitDecl {
            name,
            type_params,
            associated_types,
            methods,
        } => {
            let mut s = format!(
                "{}trait {}{} {{\n",
                pad,
                name,
                format_generic_params(type_params)
            );
            for associated in associated_types {
                s.push_str(&format!("{}    type {}\n", pad, associated.name));
            }
            for method in methods {
                let params = method
                    .params
                    .iter()
                    .map(|param| match &param.type_annot {
                        Some(ty) => format!("{}: {}", param.name, format_type(ty)),
                        None => param.name.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                s.push_str(&format!(
                    "{}    fn {}{}({}) -> {}\n",
                    pad,
                    method.name,
                    format_generic_params(&method.type_params),
                    params,
                    format_type(&method.ret_type)
                ));
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::ImplBlock {
            type_params,
            trait_ref,
            target_type,
            items,
        } => {
            let relation = trait_ref
                .as_ref()
                .map(|trait_ref| format!("{} for ", format_type(trait_ref)))
                .unwrap_or_default();
            let mut s = format!(
                "{}impl {}{}{} {{\n",
                pad,
                format_generic_params(type_params),
                relation,
                format_type(target_type)
            );
            for item in items {
                match item {
                    ImplItem::AssociatedType { name, target, .. } => s.push_str(&format!(
                        "{}    type {} = {}\n",
                        pad,
                        name,
                        format_type(target)
                    )),
                    ImplItem::Method(method) => {
                        s.push_str(&format!("{}\n", format_stmt(method, indent + 1)))
                    }
                }
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Operator {
            operator,
            type_params,
            params,
            ret_type,
            body,
            ..
        } => {
            let params = params
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, format_type(ty)))
                .collect::<Vec<_>>()
                .join(", ");
            let mut s = format!(
                "{}operator {}{}({}) -> {} {{\n",
                pad,
                operator,
                format_generic_params(type_params),
                params,
                format_type(ret_type)
            );
            for statement in body {
                s.push_str(&format_stmt(statement, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Function {
            attributes,
            name,
            type_params,
            is_async,
            params,
            ret_type,
            body,
            ..
        } => {
            let mut s = String::new();
            for attr in attributes {
                if attr.args.is_empty() {
                    s.push_str(&format!("{}@{}\n", pad, attr.name));
                } else {
                    s.push_str(&format!(
                        "{}@{}({})\n",
                        pad,
                        attr.name,
                        attr.args.join(", ")
                    ));
                }
            }
            let async_prefix = if *is_async { "async " } else { "" };
            let p_strs: Vec<String> = params
                .iter()
                .map(|(p, t)| format!("{}: {}", p, format_type(t)))
                .collect();
            let ret_str = if *ret_type != TypeNode::Void {
                format!(" -> {}", format_type(ret_type))
            } else {
                String::new()
            };

            let mut s = format!(
                "{}{}fn {}{}({}){} {{\n",
                pad,
                async_prefix,
                name,
                format_generic_params(type_params),
                p_strs.join(", "),
                ret_str
            );
            for b in body {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Let {
            name,
            is_mut,
            type_annot,
            value,
        } => {
            let mut_str = if *is_mut { "mut " } else { "" };
            let ty_str = if let Some(t) = type_annot {
                format!(": {}", format_type(t))
            } else {
                String::new()
            };
            format!(
                "{}let {}{}{} = {}",
                pad,
                mut_str,
                name,
                ty_str,
                format_expr(value)
            )
        }
        StmtKind::LetDestructure {
            pattern,
            type_annot,
            value,
        } => {
            let pattern = match pattern {
                Pattern::Wildcard => "_".into(),
                Pattern::Identifier(name) => name.clone(),
                Pattern::Tuple(items) => format!(
                    "({})",
                    items
                        .iter()
                        .map(|item| match item {
                            Pattern::Wildcard => "_".into(),
                            Pattern::Identifier(name) => name.clone(),
                            _ => "…".into(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Pattern::Struct { name, .. } => format!("{} {{ … }}", name),
            };
            let ty = type_annot
                .as_ref()
                .map(|ty| format!(": {}", format_type(ty)))
                .unwrap_or_default();
            format!("{}let {}{} = {}", pad, pattern, ty, format_expr(value))
        }
        StmtKind::Const {
            name,
            type_annot,
            value,
        } => {
            let ty_str = if let Some(t) = type_annot {
                format!(": {}", format_type(t))
            } else {
                String::new()
            };
            format!("{}const {}{} = {}", pad, name, ty_str, format_expr(value))
        }
        StmtKind::Comptime(body) => {
            let mut s = format!("{}comptime {{\n", pad);
            for b in body {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Assign { target, value } => {
            format!("{}{} = {}", pad, target, format_expr(value))
        }
        StmtKind::AssignIndex {
            target,
            index,
            value,
        } => {
            format!(
                "{}[{}] = {}",
                format_expr(target),
                format_expr(index),
                format_expr(value)
            )
        }
        StmtKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut s = format!("{}if {} {{\n", pad, format_expr(condition));
            for b in then_branch {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            if let Some(eb) = else_branch {
                s.push_str(" else {\n");
                for b in eb {
                    s.push_str(&format_stmt(b, indent + 1));
                    s.push('\n');
                }
                s.push_str(&format!("{}}}", pad));
            }
            s
        }
        StmtKind::While { condition, body } => {
            let mut s = format!("{}while {} {{\n", pad, format_expr(condition));
            for b in body {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Loop { body } => {
            let mut s = format!("{}loop {{\n", pad);
            for b in body {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Break => format!("{}break", pad),
        StmtKind::Continue => format!("{}continue", pad),
        StmtKind::Return(Some(val)) => format!("{}return {}", pad, format_expr(val)),
        StmtKind::Return(None) => format!("{}return", pad),
        StmtKind::Print(args) => {
            let arg_strs: Vec<String> = args.iter().map(format_expr).collect();
            format!("{}print({})", pad, arg_strs.join(", "))
        }
        StmtKind::Assert(expr) => format!("{}assert({})", pad, format_expr(expr)),
        StmtKind::TestBlock { name, body } => {
            let mut s = format!("{}test \"{}\" {{\n", pad, name);
            for b in body {
                s.push_str(&format_stmt(b, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
        StmtKind::Expr(expr) => format!("{}{}", pad, format_expr(expr)),
        StmtKind::UnsafeBlock(body) => {
            let mut s = format!("{}unsafe {{\n", pad);
            for st in body {
                s.push_str(&format_stmt(st, indent + 1));
                s.push('\n');
            }
            s.push_str(&format!("{}}}", pad));
            s
        }
    }
}

fn format_expr(expr: &SpannedExpr) -> String {
    match &expr.kind {
        ExprKind::Move(inner) => format!("move {}", format_expr(inner)),
        ExprKind::UnsafeBlock(body) => {
            let mut s = "unsafe {\n".to_string();
            for st in body {
                s.push_str(&format_stmt(st, 1));
                s.push('\n');
            }
            s.push('}');
            s
        }
        ExprKind::Int(n) => n.to_string(),
        ExprKind::Float(f) => f.to_string(),
        ExprKind::Str(s) => format!("\"{}\"", s),
        ExprKind::Char(c) => format!("'{}'", c),
        ExprKind::Bool(b) => b.to_string(),
        ExprKind::Null => "null".to_string(),
        ExprKind::Some(inner) => format!("Some({})", format_expr(inner)),
        ExprKind::None => "None".to_string(),
        ExprKind::Ok(inner) => format!("Ok({})", format_expr(inner)),
        ExprKind::Err(inner) => format!("Err({})", format_expr(inner)),
        ExprKind::Var(v) => v.clone(),
        ExprKind::Borrow { expr, is_mut } => {
            if *is_mut {
                format!("&mut {}", format_expr(expr))
            } else {
                format!("&{}", format_expr(expr))
            }
        }
        ExprKind::Task(inner) => format!("task {}", format_expr(inner)),
        ExprKind::Await(inner) => format!("await {}", format_expr(inner)),
        ExprKind::Spawn { callee, args } => {
            let a_strs: Vec<String> = args.iter().map(format_expr).collect();
            format!("spawn {}({})", callee, a_strs.join(", "))
        }
        ExprKind::MakeChan(ty) => format!("chan<{}>()", format_type(ty)),
        ExprKind::ChanSend { chan, value } => {
            format!("{}.send({})", format_expr(chan), format_expr(value))
        }
        ExprKind::ChanRecv(chan) => format!("{}.recv()", format_expr(chan)),
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
            format!("{} {} {}", format_expr(left), op_str, format_expr(right))
        }
        ExprKind::Unary { op, expr } => {
            let op_str = match op {
                UnaryOpKind::Neg => "-",
                UnaryOpKind::Not => "!",
                UnaryOpKind::Deref => "*",
            };
            format!("{}{}", op_str, format_expr(expr))
        }
        ExprKind::Call { callee, args } => {
            let a_strs: Vec<String> = args.iter().map(format_expr).collect();
            format!("{}({})", callee, a_strs.join(", "))
        }
        ExprKind::Tuple(items) => format!(
            "({})",
            items.iter().map(format_expr).collect::<Vec<_>>().join(", ")
        ),
        ExprKind::Array(elements) => {
            let a_strs: Vec<String> = elements.iter().map(format_expr).collect();
            format!("[{}]", a_strs.join(", "))
        }
        ExprKind::Set(elements) => format!(
            "set {{{}}}",
            elements
                .iter()
                .map(format_expr)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ExprKind::Map(entries) => {
            let e_strs: Vec<String> = entries
                .iter()
                .map(|(k, v)| format!("{}: {}", format_expr(k), format_expr(v)))
                .collect();
            format!("{{{}}}", e_strs.join(", "))
        }
        ExprKind::Index { target, index } => {
            format!("{}[{}]", format_expr(target), format_expr(index))
        }
        ExprKind::Dot { target, field } => {
            format!("{}.{}", format_expr(target), field)
        }
        ExprKind::CustomBinary {
            left,
            operator,
            right,
        } => {
            format!("{} {} {}", format_expr(left), operator, format_expr(right))
        }
        ExprKind::IsA { value, type_node } => {
            format!("{} is {}", format_expr(value), format_type(type_node))
        }
        ExprKind::EnumVariantConstruct {
            enum_name,
            variant_name,
            payload,
        } => match payload {
            Some(value) => format!("{}::{}({})", enum_name, variant_name, format_expr(value)),
            None => format!("{}::{}", enum_name, variant_name),
        },
        ExprKind::Match { value, arms } => {
            let mut s = format!("match {} {{\n", format_expr(value));
            for arm in arms {
                let pat_str = match &arm.pattern {
                    MatchPattern::Wildcard => "_".to_string(),
                    MatchPattern::Literal(e) => format_expr(e),
                    MatchPattern::Var(v) => v.clone(),
                    MatchPattern::Some(v) => format!("Some({})", v),
                    MatchPattern::None => "None".to_string(),
                    MatchPattern::Ok(v) => format!("Ok({})", v),
                    MatchPattern::Err(v) => format!("Err({})", v),
                    MatchPattern::EnumVariant(v, p) => {
                        if let Some(payload) = p {
                            format!("{}({})", v, payload)
                        } else {
                            v.clone()
                        }
                    }
                    MatchPattern::EnumVariantStruct(variant, fields) => {
                        let fields = fields
                            .iter()
                            .map(|(name, pattern)| match pattern {
                                Some(MatchPattern::Var(binding)) => {
                                    format!("{}: {}", name, binding)
                                }
                                _ => name.clone(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{} {{ {} }}", variant, fields)
                    }
                };
                s.push_str(&format!("    {} => {},\n", pat_str, format_expr(&arm.body)));
            }
            s.push('}');
            s
        }
        ExprKind::Block(stmts) => {
            let mut s = "{\n".to_string();
            for st in stmts {
                s.push_str(&format_stmt(st, 1));
                s.push('\n');
            }
            s.push('}');
            s
        }
    }
}
