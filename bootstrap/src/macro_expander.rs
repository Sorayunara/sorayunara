#![allow(dead_code)]

use crate::ast::*;
use crate::diagnostic::Span;

pub fn expand_macros(program: Program) -> Program {
    let mut generated_stmts = Vec::new();
    let mut filtered_stmts = Vec::new();

    // Conditional compilation: filter by @cfg
    for stmt in program.statements {
        let is_included = check_conditional_compilation(&stmt);
        if is_included {
            filtered_stmts.push(stmt);
        }
    }

    for stmt in &filtered_stmts {
        if let StmtKind::StructDecl {
            attributes,
            name,
            fields,
            ..
        } = &stmt.kind
        {
            // Automatic Compile-Time Reflection generator
            generated_stmts.push(generate_reflection_fn(name, fields, stmt.span));

            for attr in attributes {
                if attr.name == "derive" {
                    for trait_name in &attr.args {
                        match trait_name.as_str() {
                            "Debug" => {
                                generated_stmts.push(generate_debug_fn(name, fields, stmt.span));
                            }
                            "Clone" => {
                                generated_stmts.push(generate_clone_fn(name, fields, stmt.span));
                            }
                            "Default" => {
                                generated_stmts.push(generate_default_fn(name, fields, stmt.span));
                            }
                            "Serialize" => {
                                generated_stmts
                                    .push(generate_serialize_fn(name, fields, stmt.span));
                            }
                            "Deserialize" => {
                                generated_stmts
                                    .push(generate_deserialize_fn(name, fields, stmt.span));
                            }
                            "PartialEq" => {
                                generated_stmts
                                    .push(generate_partial_eq_fn(name, fields, stmt.span));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    filtered_stmts.extend(generated_stmts);
    Program {
        statements: filtered_stmts,
    }
}

fn check_conditional_compilation(stmt: &SpannedStmt) -> bool {
    let attrs = match &stmt.kind {
        StmtKind::Function { attributes, .. } => attributes,
        StmtKind::StructDecl { attributes, .. } => attributes,
        _ => return true,
    };

    for attr in attrs {
        if attr.name == "cfg" {
            for arg in &attr.args {
                if arg == "disabled" || arg == "not_target" {
                    return false;
                }
            }
        }
    }

    true
}

fn generate_debug_fn(struct_name: &str, _fields: &[(String, TypeNode)], span: Span) -> SpannedStmt {
    let fn_name = format!("debug_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: vec![("self_val".into(), TypeNode::Infer)],
            ret_type: TypeNode::String,
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Str(format!("<struct {}>", struct_name)),
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_clone_fn(struct_name: &str, _fields: &[(String, TypeNode)], span: Span) -> SpannedStmt {
    let fn_name = format!("clone_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: vec![("self_val".into(), TypeNode::Infer)],
            ret_type: TypeNode::Infer,
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Var("self_val".into()),
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_default_fn(
    struct_name: &str,
    _fields: &[(String, TypeNode)],
    span: Span,
) -> SpannedStmt {
    let fn_name = format!("default_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: Vec::new(),
            ret_type: TypeNode::Custom(struct_name.into()),
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Null,
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_serialize_fn(
    struct_name: &str,
    _fields: &[(String, TypeNode)],
    span: Span,
) -> SpannedStmt {
    let fn_name = format!("serialize_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: vec![("self_val".into(), TypeNode::Infer)],
            ret_type: TypeNode::String,
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Str(format!("{{\"type\": \"{}\"}}", struct_name)),
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_deserialize_fn(
    struct_name: &str,
    _fields: &[(String, TypeNode)],
    span: Span,
) -> SpannedStmt {
    let fn_name = format!("deserialize_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: vec![("raw_json".into(), TypeNode::String)],
            ret_type: TypeNode::Custom(struct_name.into()),
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Null,
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_partial_eq_fn(
    struct_name: &str,
    _fields: &[(String, TypeNode)],
    span: Span,
) -> SpannedStmt {
    let fn_name = format!("eq_{}", struct_name);
    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: vec![
                ("a".into(), TypeNode::Custom(struct_name.into())),
                ("b".into(), TypeNode::Custom(struct_name.into())),
            ],
            ret_type: TypeNode::Bool,
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Bool(true),
                    span,
                })),
                span,
            }],
        },
        span,
    }
}

fn generate_reflection_fn(
    struct_name: &str,
    fields: &[(String, TypeNode)],
    span: Span,
) -> SpannedStmt {
    let fn_name = format!("reflect_fields_{}", struct_name);
    let field_exprs: Vec<SpannedExpr> = fields
        .iter()
        .map(|(f, _)| SpannedExpr {
            kind: ExprKind::Str(f.clone()),
            span,
        })
        .collect();

    SpannedStmt {
        kind: StmtKind::Function {
            attributes: Vec::new(),
            name: fn_name,
            type_params: Vec::new(),
            is_async: false,
            params: Vec::new(),
            ret_type: TypeNode::Array(Box::new(TypeNode::String)),
            body: vec![SpannedStmt {
                kind: StmtKind::Return(Some(SpannedExpr {
                    kind: ExprKind::Array(field_exprs),
                    span,
                })),
                span,
            }],
        },
        span,
    }
}
