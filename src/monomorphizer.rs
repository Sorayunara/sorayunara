#![allow(dead_code)]

use crate::ast::*;
use std::collections::HashMap;

fn is_generic_type(ty: &TypeNode) -> bool {
    match ty {
        TypeNode::Infer => true,
        TypeNode::Custom(s) if s == "T" => true,
        _ => false,
    }
}

pub struct Monomorphizer {
    specialized_functions: HashMap<String, SpannedStmt>,
}

impl Monomorphizer {
    pub fn new() -> Self {
        Self {
            specialized_functions: HashMap::new(),
        }
    }

    pub fn monomorphize_program(&mut self, mut program: Program) -> Program {
        let mut specialized = Vec::new();

        for stmt in &program.statements {
            if let StmtKind::Function {
                name,
                is_async,
                params,
                ret_type,
                body,
                attributes,
                ..
            } = &stmt.kind
            {
                let is_generic = params.iter().any(|(_, ty)| is_generic_type(ty));

                if is_generic {
                    let specialized_int_fn = format!("{}_Int", name);
                    let specialized_int_params = params
                        .iter()
                        .map(|(p_name, ty)| {
                            if is_generic_type(ty) {
                                (p_name.clone(), TypeNode::Int)
                            } else {
                                (p_name.clone(), ty.clone())
                            }
                        })
                        .collect();

                    let int_ret = if is_generic_type(ret_type) {
                        TypeNode::Int
                    } else {
                        ret_type.clone()
                    };

                    specialized.push(SpannedStmt {
                        kind: StmtKind::Function {
                            attributes: attributes.clone(),
                            name: specialized_int_fn,
                            type_params: Vec::new(),
                            is_async: *is_async,
                            params: specialized_int_params,
                            ret_type: int_ret,
                            body: body.clone(),
                        },
                        span: stmt.span,
                    });
                }
            }
        }

        program.statements.extend(specialized);
        program
    }
}

pub fn monomorphize(program: Program) -> Program {
    let mut m = Monomorphizer::new();
    m.monomorphize_program(program)
}
