#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::diagnostic::{Diagnostic, DiagnosticEngine, Span};
use crate::symbol_table::{
    FunctionSymbol, OperatorSymbol, SymbolTable, TraitImpl, TraitSymbol, Type, VariableSymbol,
    Substitution,
};

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    engine: DiagnosticEngine,
    current_fn_return: Option<Type>,
    substitution: Substitution,
    pending_trait_checks: Vec<(Type, String, Span)>,
    inferred_annotations: HashMap<Span, Type>,
    call_site_instantiations: Vec<(String, Vec<(Type, Span)>)>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            engine: DiagnosticEngine::new(),
            current_fn_return: None,
            substitution: Substitution::new(),
            pending_trait_checks: Vec::new(),
            inferred_annotations: HashMap::new(),
            call_site_instantiations: Vec::new(),
        }
    }

    fn unify(&mut self, a: &Type, b: &Type, span: Span) -> Result<(), String> {
        let a = self.substitution.apply(a);
        let b = self.substitution.apply(b);
        match (&a, &b) {
            (Type::TypeVar(id), _) => {
                self.substitution.bind(*id, b.clone())
                    .map_err(|e| {
                        self.engine.emit(Diagnostic::error(
                            format!("Unification error: {}", e),
                            span,
                        ).with_hint("Try adding explicit type annotations to resolve the ambiguity"));
                        e
                    })
            }
            (_, Type::TypeVar(id)) => {
                self.substitution.bind(*id, a.clone())
                    .map_err(|e| {
                        self.engine.emit(Diagnostic::error(
                            format!("Unification error: {}", e),
                            span,
                        ).with_hint("Try adding explicit type annotations to resolve the ambiguity"));
                        e
                    })
            }
            (Type::Any, _) | (_, Type::Any) => Ok(()),
            (Type::Generic(na), Type::Generic(nb)) if na == nb => Ok(()),
            (Type::Int, Type::Int) | (Type::Float, Type::Float) | (Type::Bool, Type::Bool)
            | (Type::String, Type::String) | (Type::Char, Type::Char) | (Type::Void, Type::Void) => Ok(()),
            (Type::Tuple(xs), Type::Tuple(ys)) => {
                if xs.len() != ys.len() {
                    return Err(format!("Tuple arity mismatch: {} vs {}", xs.len(), ys.len()));
                }
                for (x, y) in xs.iter().zip(ys.iter()) {
                    self.unify(x, y, span)?;
                }
                Ok(())
            }
            (Type::Array(x), Type::Array(y))
            | (Type::Slice(x), Type::Slice(y))
            | (Type::Set(x), Type::Set(y))
            | (Type::Option(x), Type::Option(y))
            | (Type::Task(x), Type::Task(y))
            | (Type::Chan(x), Type::Chan(y)) => self.unify(x, y, span),
            (Type::Map(k1, v1), Type::Map(k2, v2))
            | (Type::Result(k1, v1), Type::Result(k2, v2)) => {
                self.unify(k1, k2, span)?;
                self.unify(v1, v2, span)
            }
            (Type::Ref(xa, ma), Type::Ref(xb, mb)) => {
                if *ma != *mb && *mb {
                    return Err("Cannot convert immutable ref to mutable ref".into());
                }
                self.unify(xa, xb, span)
            }
            (Type::Ptr(_, _), Type::Ptr(_, _)) => Ok(()),
            (Type::Function(p1, r1), Type::Function(p2, r2)) => {
                if p1.len() != p2.len() {
                    return Err(format!("Function arity mismatch: {} vs {}", p1.len(), p2.len()));
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b, span)?;
                }
                self.unify(r1, r2, span)
            }
            (Type::GenericInstance(na, args_a), Type::GenericInstance(nb, args_b)) => {
                if na != nb {
                    return Err(format!("Generic type mismatch: {} vs {}", na, nb));
                }
                if args_a.len() != args_b.len() {
                    return Err(format!("Generic arity mismatch for {}: {} vs {}", na, args_a.len(), args_b.len()));
                }
                for (a, b) in args_a.iter().zip(args_b.iter()) {
                    self.unify(a, b, span)?;
                }
                Ok(())
            }
            (Type::Struct(na, _), Type::Struct(nb, _)) if na == nb => Ok(()),
            (Type::Enum(na, _), Type::Enum(nb, _)) if na == nb => Ok(()),
            (src, Type::Union(members)) => {
                if members.iter().any(|m| self.substitution.apply(src).is_assignable_to(&self.substitution.apply(m))) {
                    Ok(())
                } else {
                    Err(format!("Type {} is not assignable to union {}", src,
                        members.iter().map(|t| format!("{}", t)).collect::<Vec<_>>().join(" | ")))
                }
            }
            _ => {
                let msg = format!("Type mismatch: expected {}, got {}", b, a);
                self.engine.emit(Diagnostic::error(msg.clone(), span));
                Err(msg)
            }
        }
    }

    fn instantiate_generic(
        &mut self,
        ty: &Type,
        generic_params: &[(String, Vec<String>)],
        call_span: Span,
    ) -> (Type, HashMap<String, Type>) {
        let mut mapping = HashMap::new();
        let mut type_args_record: Vec<(Type, Span)> = Vec::new();
        for (name, bounds) in generic_params {
            let tv = self.symbol_table.fresh_type_var();
            mapping.insert(name.clone(), tv.clone());
            for bound in bounds {
                self.pending_trait_checks.push((tv.clone(), bound.clone(), call_span));
                type_args_record.push((tv.clone(), call_span));
            }
        }
        let instantiated = self.apply_mapping(ty, &mapping);
        (instantiated, mapping)
    }

    fn apply_mapping(&self, ty: &Type, mapping: &HashMap<String, Type>) -> Type {
        match ty {
            Type::Generic(name) => {
                mapping.get(name).cloned().unwrap_or_else(|| ty.clone())
            }
            Type::Tuple(items) => {
                Type::Tuple(items.iter().map(|t| self.apply_mapping(t, mapping)).collect())
            }
            Type::Array(i) => Type::Array(Box::new(self.apply_mapping(i, mapping))),
            Type::Slice(i) => Type::Slice(Box::new(self.apply_mapping(i, mapping))),
            Type::Set(i) => Type::Set(Box::new(self.apply_mapping(i, mapping))),
            Type::Option(i) => Type::Option(Box::new(self.apply_mapping(i, mapping))),
            Type::Task(i) => Type::Task(Box::new(self.apply_mapping(i, mapping))),
            Type::Chan(i) => Type::Chan(Box::new(self.apply_mapping(i, mapping))),
            Type::Map(k, v) => Type::Map(
                Box::new(self.apply_mapping(k, mapping)),
                Box::new(self.apply_mapping(v, mapping)),
            ),
            Type::Result(k, v) => Type::Result(
                Box::new(self.apply_mapping(k, mapping)),
                Box::new(self.apply_mapping(v, mapping)),
            ),
            Type::Function(p, r) => Type::Function(
                p.iter().map(|t| self.apply_mapping(t, mapping)).collect(),
                Box::new(self.apply_mapping(r, mapping)),
            ),
            Type::GenericInstance(name, args) => Type::GenericInstance(
                name.clone(),
                args.iter().map(|t| self.apply_mapping(t, mapping)).collect(),
            ),
            Type::Ref(i, m) => Type::Ref(Box::new(self.apply_mapping(i, mapping)), *m),
            Type::Ptr(i, c) => Type::Ptr(Box::new(self.apply_mapping(i, mapping)), *c),
            Type::Struct(name, fields) => {
                let mut new_fields = HashMap::new();
                for (k, v) in fields {
                    new_fields.insert(k.clone(), self.apply_mapping(v, mapping));
                }
                Type::Struct(name.clone(), new_fields)
            }
            Type::Enum(name, variants) => {
                let mut new_variants = HashMap::new();
                for (k, v) in variants {
                    new_variants.insert(k.clone(), v.as_ref().map(|t| self.apply_mapping(t, mapping)));
                }
                Type::Enum(name.clone(), new_variants)
            }
            Type::Union(members) => {
                Type::Union(members.iter().map(|t| self.apply_mapping(t, mapping)).collect())
            }
            other => other.clone(),
        }
    }

    fn verify_trait_constraints(&mut self) {
        let checks: Vec<_> = self.pending_trait_checks.drain(..).collect();
        for (ty, trait_name, span) in checks {
            let resolved = self.substitution.apply(&ty);
            if !self.symbol_table.satisfies_trait(&resolved, &trait_name, &self.substitution) {
                let msg = format!(
                    "Type `{}` does not implement trait `{}`",
                    resolved, trait_name
                );
                let hint = match trait_name.as_str() {
                    "Comparable" => "Comparable is required for ordering/comparison operations. Implement the trait for this type.".into(),
                    "Hashable" => "Hashable is required for Set/Map keys. It requires Comparable as a super-trait.".into(),
                    "Addable" => "Addable is required for custom `+` operations with output type.".into(),
                    "Display" => "Display is required for formatting/printing values as strings.".into(),
                    _ => format!("Consider adding `impl {} for {}`", trait_name, resolved),
                };
                self.engine.emit(Diagnostic::error(msg, span).with_hint(hint));
            }
        }
    }

    pub fn analyze(mut self, program: &Program) -> Result<(SymbolTable, HashMap<Span, Type>), DiagnosticEngine> {
        for stmt in &program.statements {
            match &stmt.kind {
                StmtKind::StructDecl { name, fields, type_params, .. } => {
                    for gp in type_params {
                        let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                            TypeNode::Custom(s) => s.clone(),
                            _ => format!("{:?}", b),
                        }).collect();
                        self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds);
                    }
                    let mut field_map = HashMap::new();
                    for (f_name, f_type_node) in fields {
                        if let Ok(f_ty) = Type::from_node(f_type_node, &self.symbol_table.structs, &self.symbol_table.enums) {
                            field_map.insert(f_name.clone(), f_ty);
                        }
                    }
                    self.symbol_table.structs.insert(name.clone(), field_map);
                    self.symbol_table.current_generic_env = crate::symbol_table::GenericEnv::new();
                }
                StmtKind::EnumDecl { name, variants, type_params, .. } => {
                    for gp in type_params {
                        let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                            TypeNode::Custom(s) => s.clone(),
                            _ => format!("{:?}", b),
                        }).collect();
                        self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds);
                    }
                    let mut variant_map = HashMap::new();
                    for (v_name, v_type_node) in variants {
                        let v_ty = match v_type_node {
                            Some(tn) => Type::from_node(tn, &self.symbol_table.structs, &self.symbol_table.enums).ok(),
                            None => None,
                        };
                        variant_map.insert(v_name.clone(), v_ty);
                    }
                    self.symbol_table.enums.insert(name.clone(), variant_map);
                    self.symbol_table.current_generic_env = crate::symbol_table::GenericEnv::new();
                }
                StmtKind::TraitDecl {
                    name,
                    type_params,
                    associated_types,
                    methods,
                    ..
                } => {
                    let super_traits: Vec<String> = type_params.iter()
                        .flat_map(|gp| gp.bounds.iter()
                            .map(|b| match b {
                                TypeNode::Custom(s) => s.clone(),
                                _ => format!("{:?}", b),
                            }))
                        .collect();
                    self.symbol_table.traits.insert(
                        name.clone(),
                        TraitSymbol {
                            name: name.clone(),
                            associated_types: associated_types.iter().map(|t| t.name.clone()).collect(),
                            methods: methods.iter().map(|m| m.name.clone()).collect(),
                            span: stmt.span,
                            super_traits,
                        },
                    );
                }
                StmtKind::ImplBlock {
                    type_params,
                    trait_ref,
                    target_type,
                    items,
                    ..
                } => {
                    for gp in type_params {
                        let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                            TypeNode::Custom(s) => s.clone(),
                            _ => format!("{:?}", b),
                        }).collect();
                        self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds);
                    }
                    let trait_name = match trait_ref {
                        Some(TypeNode::Custom(s)) => s.clone(),
                        Some(_) => "Unknown".to_string(),
                        None => "__inherent__".to_string(),
                    };
                    let target_ty = Type::from_node(target_type, &self.symbol_table.structs, &self.symbol_table.enums)
                        .unwrap_or(Type::Any);
                    let item_names: Vec<String> = items.iter().filter_map(|it| match it {
                        ImplItem::Method(m) => match &m.kind {
                            StmtKind::Function { name, .. } => Some(name.clone()),
                            _ => None,
                        },
                        ImplItem::AssociatedType { name, .. } => Some(format!("type {}", name)),
                    }).collect();
                    self.symbol_table.trait_impls.push(TraitImpl {
                        trait_name: trait_name.clone(),
                        type_params: type_params.iter().map(|gp| {
                            let b: Vec<String> = gp.bounds.iter().map(|b| match b {
                                TypeNode::Custom(s) => s.clone(),
                                _ => format!("{:?}", b),
                            }).collect();
                            (gp.name.clone(), b)
                        }).collect(),
                        target_type: target_ty,
                        impl_items: item_names,
                        span: stmt.span,
                    });
                    self.symbol_table.current_generic_env = crate::symbol_table::GenericEnv::new();
                }
                _ => {}
            }
        }

        for stmt in &program.statements {
            match &stmt.kind {
                StmtKind::Function {
                    name,
                    is_async,
                    params,
                    ret_type,
                    type_params,
                    ..
                } => {
                    let saved_env = self.symbol_table.current_generic_env.clone();
                    let typed_gp: Vec<(String, Vec<String>)> = type_params.iter().map(|gp| {
                        let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                            TypeNode::Custom(s) => s.clone(),
                            _ => format!("{:?}", b),
                        }).collect();
                        self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds.clone());
                        (gp.name.clone(), bounds)
                    }).collect();
                    let mut typed_params = Vec::new();
                    for (p_name, p_ty_node) in params {
                        match Type::from_node(p_ty_node, &self.symbol_table.structs, &self.symbol_table.enums) {
                            Ok(t) => typed_params.push((p_name.clone(), t)),
                            Err(e) => {
                                self.engine.emit(Diagnostic::error(e, stmt.span));
                            }
                        }
                    }
                    let ret = match Type::from_node(ret_type, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => t,
                        Err(e) => {
                            self.engine.emit(Diagnostic::error(e, stmt.span));
                            Type::Void
                        }
                    };

                    let func_sym = FunctionSymbol {
                        name: name.clone(),
                        is_async: *is_async,
                        is_extern: false,
                        type_params: typed_gp,
                        params: typed_params,
                        ret_type: ret,
                        span: stmt.span,
                    };

                    if let Err(err) = self.symbol_table.define_function(func_sym) {
                        self.engine.emit(Diagnostic::error(err, stmt.span));
                    }
                    self.symbol_table.current_generic_env = saved_env;
                }
                StmtKind::Operator {
                    operator,
                    params,
                    ret_type,
                    type_params,
                    ..
                } => {
                    let saved_env = self.symbol_table.current_generic_env.clone();
                    let typed_gp: Vec<(String, Vec<String>)> = type_params.iter().map(|gp| {
                        let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                            TypeNode::Custom(s) => s.clone(),
                            _ => format!("{:?}", b),
                        }).collect();
                        self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds.clone());
                        (gp.name.clone(), bounds)
                    }).collect();
                    let mut typed_params = Vec::new();
                    for (p_name, p_ty_node) in params {
                        match Type::from_node(p_ty_node, &self.symbol_table.structs, &self.symbol_table.enums) {
                            Ok(t) => typed_params.push((p_name.clone(), t)),
                            Err(e) => self.engine.emit(Diagnostic::error(e, stmt.span)),
                        }
                    }
                    let ret = match Type::from_node(ret_type, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => t,
                        Err(e) => {
                            self.engine.emit(Diagnostic::error(e, stmt.span));
                            Type::Any
                        }
                    };

                    if typed_params.len() != 2 {
                        self.engine.emit(
                            Diagnostic::error(
                                format!("Operator '{}' must declare exactly two operands", operator),
                                stmt.span,
                            )
                            .with_hint("Use syntax like: operator <+>(left: T, right: T) -> T { ... }"),
                        );
                    }

                    let op_sym = OperatorSymbol {
                        operator: operator.clone(),
                        type_params: typed_gp,
                        params: typed_params,
                        ret_type: ret,
                        span: stmt.span,
                    };
                    if let Err(err) = self.symbol_table.define_operator(op_sym) {
                        self.engine.emit(Diagnostic::error(err, stmt.span));
                    }
                    self.symbol_table.current_generic_env = saved_env;
                }
                StmtKind::ExternBlock { abi, functions, .. } => {
                    for ext_fn in functions {
                        let mut typed_params = Vec::new();
                        for (p_name, p_ty_node) in &ext_fn.params {
                            match Type::from_node(p_ty_node, &self.symbol_table.structs, &self.symbol_table.enums) {
                                Ok(t) => typed_params.push((p_name.clone(), t)),
                                Err(e) => {
                                    self.engine.emit(Diagnostic::error(e, ext_fn.span));
                                }
                            }
                        }
                        let ret = match Type::from_node(&ext_fn.ret_type, &self.symbol_table.structs, &self.symbol_table.enums) {
                            Ok(t) => t,
                            Err(e) => {
                                self.engine.emit(Diagnostic::error(e, ext_fn.span));
                                Type::Void
                            }
                        };

                        let func_sym = FunctionSymbol {
                            name: ext_fn.name.clone(),
                            is_async: false,
                            is_extern: true,
                            type_params: Vec::new(),
                            params: typed_params,
                            ret_type: ret,
                            span: ext_fn.span,
                        };

                        if let Err(err) = self.symbol_table.define_function(func_sym) {
                            self.engine.emit(
                                Diagnostic::error(
                                    format!("FFI conflict: {} (ABI '{}')", err, abi),
                                    ext_fn.span,
                                )
                                .with_hint("External symbols must not collide with Aether functions"),
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        for stmt in &program.statements {
            self.analyze_statement(stmt);
        }

        self.verify_trait_constraints();

        if self.engine.has_errors() {
            Err(self.engine)
        } else {
            Ok((self.symbol_table, self.inferred_annotations))
        }
    }

    fn analyze_statement(&mut self, stmt: &SpannedStmt) {
        match &stmt.kind {
            StmtKind::StructDecl { .. }
            | StmtKind::EnumDecl { .. }
            | StmtKind::TraitDecl { .. }
            | StmtKind::ImplBlock { .. }
            | StmtKind::TypeAlias { .. }
            | StmtKind::Import(_)
            | StmtKind::Mod(_)
            | StmtKind::ExternBlock { .. } => {}
            StmtKind::Function {
                name,
                type_params,
                body,
                ..
            } => {
                let sig = match self.symbol_table.lookup_function(name) {
                    Some(s) => s.clone(),
                    None => return,
                };

                self.current_fn_return = Some(sig.ret_type.clone());
                let saved_env = self.symbol_table.current_generic_env.clone();
                for (n, bounds) in &sig.type_params {
                    self.symbol_table.current_generic_env.push_generic(n.clone(), bounds.clone());
                }
                self.symbol_table.enter_scope(false);

                for (p_name, p_type) in &sig.params {
                    let _ = self.symbol_table.define_variable(VariableSymbol {
                        name: p_name.clone(),
                        ty: p_type.clone(),
                        is_mut: false,
                        is_moved: false,
                        borrow_count: 0,
                        is_mut_borrowed: false,
                        span: stmt.span,
                        narrowed_type: None,
                    });
                }

                for s in body {
                    self.analyze_statement(s);
                }

                self.symbol_table.exit_scope();
                self.symbol_table.current_generic_env = saved_env;
                self.current_fn_return = None;
                let _ = type_params;
            }
            StmtKind::Operator {
                operator,
                params,
                ret_type,
                type_params,
                body,
                ..
            } => {
                let mut typed_params = Vec::new();
                for (p_name, p_type_node) in params {
                    match Type::from_node(p_type_node, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => typed_params.push((p_name.clone(), t)),
                        Err(e) => self.engine.emit(Diagnostic::error(e, stmt.span)),
                    }
                }
                let expected_ret = Type::from_node(ret_type, &self.symbol_table.structs, &self.symbol_table.enums)
                    .unwrap_or(Type::Any);

                self.current_fn_return = Some(expected_ret);
                let saved_env = self.symbol_table.current_generic_env.clone();
                for gp in type_params {
                    let bounds: Vec<String> = gp.bounds.iter().map(|b| match b {
                        TypeNode::Custom(s) => s.clone(),
                        _ => format!("{:?}", b),
                    }).collect();
                    self.symbol_table.current_generic_env.push_generic(gp.name.clone(), bounds);
                }
                self.symbol_table.enter_scope(false);
                for (p_name, p_type) in typed_params {
                    let _ = self.symbol_table.define_variable(VariableSymbol {
                        name: p_name,
                        ty: p_type,
                        is_mut: false,
                        is_moved: false,
                        borrow_count: 0,
                        is_mut_borrowed: false,
                        span: stmt.span,
                        narrowed_type: None,
                    });
                }
                for s in body {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();
                self.symbol_table.current_generic_env = saved_env;
                self.current_fn_return = None;

                if self.symbol_table.lookup_operator(operator).is_none() {
                    self.engine.emit(Diagnostic::error(
                        format!("Operator '{}' was not registered", operator),
                        stmt.span,
                    ));
                }
            }
            StmtKind::UnsafeBlock(body) => {
                // Unsafe mode: careful optimization disabled, ownership rules relaxed.
                // The analyzer still type-checks the inner statements.
                self.symbol_table.enter_scope(false);
                for s in body {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();
            }
            StmtKind::Let {
                name,
                is_mut,
                type_annot,
                value,
            } => {
                let value_span = value.span;
                let inferred_ty_before = self.symbol_table.fresh_type_var();
                self.inferred_annotations.insert(value.span, inferred_ty_before.clone());

                let inferred_ty = self.analyze_expr(value);

                let _ = self.unify(&inferred_ty_before, &inferred_ty, value_span);

                let declared_ty = if let Some(tn) = type_annot {
                    match Type::from_node(tn, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => t,
                        Err(e) => {
                            self.engine.emit(Diagnostic::error(e, stmt.span));
                            Type::Any
                        }
                    }
                } else {
                    self.substitution.apply(&inferred_ty)
                };

                if let Err(_) = self.unify(&inferred_ty, &declared_ty, value.span) {
                    self.engine.emit(
                        Diagnostic::error(
                            format!(
                                "Type mismatch for '{}': declared {}, got {}",
                                name, declared_ty, self.substitution.apply(&inferred_ty)
                            ),
                            value.span,
                        )
                        .with_hint("Ensure the assigned value matches the declared variable type"),
                    );
                }

                let final_ty = self.substitution.apply(&declared_ty);
                self.inferred_annotations.insert(stmt.span, final_ty.clone());
                if type_annot.is_none() {
                    self.engine.emit(Diagnostic {
                        level: crate::diagnostic::DiagnosticLevel::Note,
                        message: format!("Inferred type of `{}` is `{}`", name, final_ty),
                        span: stmt.span,
                        hint: None,
                    });
                }

                // Track memory model: `let` from a non-Copy value in
                // Owned mode transfers ownership (auto-move). If the RHS
                // is a variable, it becomes moved.
                //
                // Auto-move only applies when the source and destination share
                // the same scope (deterministic sequential control flow).
                // Variables from sibling/outer branch scopes must not be marked
                // moved because the move would be conditional.
                if !final_ty.is_copy() {
                    if let ExprKind::Var(src_name) = &value.kind {
                        let in_current_scope = self.symbol_table.is_in_current_scope(src_name);
                        if in_current_scope {
                            if let Some(src_var) = self.symbol_table.get_variable_mut(src_name) {
                                if !src_var.is_moved {
                                    src_var.is_moved = true;
                                    self.engine.emit(Diagnostic {
                                        level: crate::diagnostic::DiagnosticLevel::Note,
                                        message: format!(
                                            "Ownership of `{}` moved to `{}` (Owned mode)",
                                            src_name, name
                                        ),
                                        span: stmt.span,
                                        hint: Some(
                                            "Use `&value` to borrow instead of move, or wrap the value in `managed` for the Managed (GC) mode.".to_string(),
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }

                let var_sym = VariableSymbol {
                    name: name.clone(),
                    ty: final_ty,
                    is_mut: *is_mut,
                    is_moved: false,
                    borrow_count: 0,
                    is_mut_borrowed: false,
                    span: stmt.span,
                    narrowed_type: None,
                };

                if let Err(err) = self.symbol_table.define_variable(var_sym) {
                    self.engine.emit(Diagnostic::error(err, stmt.span));
                }
            }
            StmtKind::LetDestructure {
                pattern,
                type_annot,
                value,
            } => {
                let inferred_ty = self.analyze_expr(value);
                let declared_ty = if let Some(tn) = type_annot {
                    match Type::from_node(tn, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => t,
                        Err(e) => {
                            self.engine.emit(Diagnostic::error(e, stmt.span));
                            Type::Any
                        }
                    }
                } else {
                    inferred_ty.clone()
                };

                let _ = self.unify(&inferred_ty, &declared_ty, value.span);
                self.bind_pattern(pattern, &declared_ty, stmt.span);
            }
            StmtKind::Const {
                name,
                type_annot,
                value,
            } => {
                let inferred_ty = self.analyze_expr(value);
                let declared_ty = if let Some(tn) = type_annot {
                    match Type::from_node(tn, &self.symbol_table.structs, &self.symbol_table.enums) {
                        Ok(t) => t,
                        Err(e) => {
                            self.engine.emit(Diagnostic::error(e, stmt.span));
                            Type::Any
                        }
                    }
                } else {
                    inferred_ty.clone()
                };

                let _ = self.unify(&inferred_ty, &declared_ty, value.span);
                let final_ty = self.substitution.apply(&declared_ty);

                let var_sym = VariableSymbol {
                    name: name.clone(),
                    ty: final_ty,
                    is_mut: false,
                    is_moved: false,
                    borrow_count: 0,
                    is_mut_borrowed: false,
                    span: stmt.span,
                    narrowed_type: None,
                };
                if let Err(err) = self.symbol_table.define_variable(var_sym) {
                    self.engine.emit(Diagnostic::error(err, stmt.span));
                }
            }
            StmtKind::Comptime(body) => {
                for s in body {
                    self.analyze_statement(s);
                }
            }
            StmtKind::Assign { target, value } => {
                let val_ty = self.analyze_expr(value);
                match self.symbol_table.get_variable(target).cloned() {
                    Some(var_sym) => {
                        if !var_sym.is_mut {
                            self.engine.emit(
                                Diagnostic::error(
                                    format!("Cannot assign to immutable variable '{}'", target),
                                    stmt.span,
                                )
                                .with_hint(format!("Make variable mutable: 'let mut {}'", target)),
                            );
                        }
                        if var_sym.borrow_count > 0 || var_sym.is_mut_borrowed {
                            self.engine.emit(Diagnostic::error(
                                format!("Cannot mutate '{}' while it is borrowed", target),
                                stmt.span,
                            ));
                        }
                        let _ = self.unify(&val_ty, &var_sym.ty, value.span);
                    }
                    None => {
                        self.engine.emit(Diagnostic::error(
                            format!("Undeclared variable '{}'", target),
                            stmt.span,
                        ));
                    }
                }
            }
            StmtKind::AssignIndex {
                target,
                index,
                value,
            } => {
                let target_ty = self.analyze_expr(target);
                let idx_ty = self.analyze_expr(index);
                let val_ty = self.analyze_expr(value);

                if idx_ty != Type::Int && idx_ty != Type::Any && idx_ty != Type::String {
                    self.engine.emit(Diagnostic::error(
                        format!("Index must be Int or String, got {}", idx_ty),
                        index.span,
                    ));
                }

                match target_ty {
                    Type::Array(elem_ty) => {
                        let _ = self.unify(&val_ty, &elem_ty, value.span);
                    }
                    Type::Map(_k, v) => {
                        let _ = self.unify(&val_ty, &v, value.span);
                    }
                    Type::Any => {}
                    other => {
                        self.engine.emit(Diagnostic::error(
                            format!("Cannot index non-collection type {}", other),
                            target.span,
                        ));
                    }
                }
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_ty = self.analyze_expr(condition);
                if cond_ty != Type::Bool && cond_ty != Type::Any {
                    let t = self.substitution.apply(&cond_ty);
                    self.engine.emit(Diagnostic::error(
                        format!("If condition must evaluate to Bool, got {}", t),
                        condition.span,
                    ));
                }

                self.symbol_table.enter_scope(false);
                self.apply_narrowing_from_condition(condition, true);
                for s in then_branch {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();

                if let Some(eb) = else_branch {
                    self.symbol_table.enter_scope(false);
                    self.apply_narrowing_from_condition(condition, false);
                    for s in eb {
                        self.analyze_statement(s);
                    }
                    self.symbol_table.exit_scope();
                }
                self.symbol_table.reset_narrowing();
            }
            StmtKind::While { condition, body } => {
                let cond_ty = self.analyze_expr(condition);
                if cond_ty != Type::Bool && cond_ty != Type::Any {
                    self.engine.emit(Diagnostic::error(
                        format!("While condition must evaluate to Bool, got {}", cond_ty),
                        condition.span,
                    ));
                }

                self.symbol_table.enter_scope(true);
                for s in body {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();
            }
            StmtKind::Loop { body } => {
                self.symbol_table.enter_scope(true);
                for s in body {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();
            }
            StmtKind::Break => {
                if !self.symbol_table.is_inside_loop() {
                    self.engine.emit(Diagnostic::error(
                        "'break' statement outside of loop context",
                        stmt.span,
                    ));
                }
            }
            StmtKind::Continue => {
                if !self.symbol_table.is_inside_loop() {
                    self.engine.emit(Diagnostic::error(
                        "'continue' statement outside of loop context",
                        stmt.span,
                    ));
                }
            }
            StmtKind::Return(expr_opt) => {
                let actual_ty = match expr_opt {
                    Some(e) => self.analyze_expr(e),
                    None => Type::Void,
                };

                if let Some(expected_ty) = self.current_fn_return.clone() {
                    let _ = self.unify(&actual_ty, &expected_ty, stmt.span);
                }
            }
            StmtKind::Print(args) => {
                for arg in args {
                    let arg_ty = self.analyze_expr(arg);
                    self.pending_trait_checks.push((arg_ty, "Display".to_string(), arg.span));
                }
            }
            StmtKind::TestBlock { body, .. } => {
                self.symbol_table.enter_scope(false);
                for s in body {
                    self.analyze_statement(s);
                }
                self.symbol_table.exit_scope();
            }
            StmtKind::Assert(expr) => {
                let ty = self.analyze_expr(expr);
                if ty != Type::Bool && ty != Type::Any {
                    self.engine.emit(Diagnostic::error(
                        format!("assert() condition must be of type Bool, got {}", ty),
                        expr.span,
                    ));
                }
            }
            StmtKind::Expr(expr) => {
                self.analyze_expr(expr);
            }
        }
    }

    fn apply_narrowing_from_condition(&mut self, condition: &SpannedExpr, positive: bool) {
        match &condition.kind {
            ExprKind::Binary { left, op: BinaryOpKind::And, right } => {
                if positive {
                    self.apply_narrowing_from_condition(left, true);
                    self.apply_narrowing_from_condition(right, true);
                } else {
                    self.apply_narrowing_from_condition(left, false);
                }
            }
            ExprKind::Binary { left, op: BinaryOpKind::Or, right } => {
                if !positive {
                    self.apply_narrowing_from_condition(left, false);
                    self.apply_narrowing_from_condition(right, false);
                } else {
                    self.apply_narrowing_from_condition(left, true);
                }
            }
            ExprKind::Unary { op: UnaryOpKind::Not, expr: inner } => {
                self.apply_narrowing_from_condition(inner, !positive);
            }
            ExprKind::IsA { value, type_node } => {
                if let ExprKind::Var(name) = &value.kind {
                    let target_ty = Type::from_node(
                        type_node,
                        &self.symbol_table.structs,
                        &self.symbol_table.enums,
                    ).unwrap_or(Type::Any);
                    let original_ty = self.symbol_table.get_variable_type(name).unwrap_or(Type::Any);

                    if positive {
                        self.symbol_table.narrow_variable_type(name, target_ty.clone());
                        self.engine.emit(Diagnostic {
                            level: crate::diagnostic::DiagnosticLevel::Note,
                            message: format!(
                                "Type-narrowed `{}` from `{}` to `{}`",
                                name, original_ty, target_ty
                            ),
                            span: value.span,
                            hint: Some("Use of `is` type guard enables flow-sensitive typing".into()),
                        });
                    } else {
                        if let Type::Union(members) = &original_ty {
                            let remaining: Vec<Type> = members.iter()
                                .filter(|m| !m.is_assignable_to(&target_ty))
                                .cloned()
                                .collect();
                            if remaining.len() == 1 {
                                self.symbol_table.narrow_variable_type(name, remaining[0].clone());
                            } else if remaining.len() > 1 {
                                self.symbol_table.narrow_variable_type(name, Type::Union(remaining));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn bind_pattern(&mut self, pattern: &Pattern, ty: &Type, span: Span) {
        match pattern {
            Pattern::Wildcard => {}
            Pattern::Identifier(name) => {
                let var_sym = VariableSymbol {
                    name: name.clone(),
                    ty: ty.clone(),
                    is_mut: false,
                    is_moved: false,
                    borrow_count: 0,
                    is_mut_borrowed: false,
                    span,
                    narrowed_type: None,
                };
                if let Err(err) = self.symbol_table.define_variable(var_sym) {
                    self.engine.emit(Diagnostic::error(err, span));
                }
            }
            Pattern::Tuple(patterns) => {
                if let Type::Tuple(items) = ty {
                    if patterns.len() != items.len() {
                        self.engine.emit(Diagnostic::error(
                            format!(
                                "Tuple destructuring expects {} fields, got {}",
                                items.len(),
                                patterns.len()
                            ),
                            span,
                        ));
                    }
                    for (pat, item_ty) in patterns.iter().zip(items.iter()) {
                        self.bind_pattern(pat, item_ty, span);
                    }
                } else if *ty == Type::Any {
                    for pat in patterns {
                        self.bind_pattern(pat, &Type::Any, span);
                    }
                } else {
                    self.engine.emit(Diagnostic::error(
                        format!("Cannot tuple-destructure non-tuple type {}", ty),
                        span,
                    ));
                }
            }
            Pattern::Struct { fields, .. } => {
                if let Type::Struct(_, struct_fields) = ty {
                    for (field_name, nested) in fields {
                        let field_ty = struct_fields.get(field_name).cloned().unwrap_or(Type::Any);
                        if let Some(nested_pat) = nested {
                            self.bind_pattern(nested_pat, &field_ty, span);
                        } else {
                            self.bind_pattern(&Pattern::Identifier(field_name.clone()), &field_ty, span);
                        }
                    }
                } else if *ty == Type::Any {
                    for (field_name, nested) in fields {
                        if let Some(nested_pat) = nested {
                            self.bind_pattern(nested_pat, &Type::Any, span);
                        } else {
                            self.bind_pattern(&Pattern::Identifier(field_name.clone()), &Type::Any, span);
                        }
                    }
                } else {
                    self.engine.emit(Diagnostic::error(
                        format!("Cannot struct-destructure non-struct type {}", ty),
                        span,
                    ));
                }
            }
        }
    }

    fn resolve_operator(
        &mut self,
        operator: &str,
        left_ty: &Type,
        right_ty: &Type,
        span: Span,
    ) -> Option<Type> {
        let op_sym = self.symbol_table.lookup_operator(operator).cloned()?;
        if op_sym.params.len() != 2 {
            self.engine.emit(Diagnostic::error(
                format!("Operator '{}' must have exactly two operands", operator),
                span,
            ));
            return Some(Type::Any);
        }

        let (inst_params, mapping) = self.instantiate_generic(
            &Type::Function(
                op_sym.params.iter().map(|(_, t)| t.clone()).collect(),
                Box::new(op_sym.ret_type.clone()),
            ),
            &op_sym.type_params,
            span,
        );
        if let Type::Function(expected_params, expected_ret) = inst_params {
            if let Some(exp_left) = expected_params.first() {
                let _ = self.unify(left_ty, exp_left, span);
            }
            if let Some(exp_right) = expected_params.get(1) {
                let _ = self.unify(right_ty, exp_right, span);
            }
            let _ = mapping;
            Some(*expected_ret)
        } else {
            Some(Type::Any)
        }
    }

    fn check_match_exhaustiveness(&mut self, arms: &[MatchArm], scrutinee_ty: &Type, match_span: Span) {
        let scrutinee = self.substitution.apply(scrutinee_ty);
        match &scrutinee {
            Type::Enum(enum_name, variants) => {
                let mut handled_variants: HashSet<String> = HashSet::new();
                let mut has_wildcard = false;

                for arm in arms {
                    match &arm.pattern {
                        MatchPattern::Wildcard => {
                            has_wildcard = true;
                        }
                        MatchPattern::Some(_) | MatchPattern::None => {
                            if enum_name == "Option" {
                                if let MatchPattern::Some(_) = arm.pattern {
                                    handled_variants.insert("Some".to_string());
                                } else {
                                    handled_variants.insert("None".to_string());
                                }
                            }
                        }
                        MatchPattern::Ok(_) | MatchPattern::Err(_) => {
                            if enum_name == "Result" {
                                if let MatchPattern::Ok(_) = arm.pattern {
                                    handled_variants.insert("Ok".to_string());
                                } else {
                                    handled_variants.insert("Err".to_string());
                                }
                            }
                        }
                        MatchPattern::EnumVariant(name, _) | MatchPattern::EnumVariantStruct(name, _) => {
                            handled_variants.insert(name.clone());
                        }
                        MatchPattern::Var(_) => {
                            has_wildcard = true;
                        }
                        _ => {}
                    }
                }

                if !has_wildcard {
                    let all_variants: HashSet<String> = if enum_name == "Option" {
                        vec!["Some".to_string(), "None".to_string()].into_iter().collect()
                    } else if enum_name == "Result" {
                        vec!["Ok".to_string(), "Err".to_string()].into_iter().collect()
                    } else {
                        variants.keys().cloned().collect()
                    };
                    let missing: Vec<String> = all_variants.difference(&handled_variants).cloned().collect();
                    if !missing.is_empty() {
                        let msg = format!(
                            "Non-exhaustive `match` on `{}`: variant{} {} not covered",
                            enum_name,
                            if missing.len() > 1 { "s" } else { "" },
                            missing.iter().map(|v| format!("`{}`", v)).collect::<Vec<_>>().join(", ")
                        );
                        let hint = format!(
                            "Consider adding arm{} for: {}; or use a `_` wildcard catch-all arm",
                            if missing.len() > 1 { "s" } else { "" },
                            missing.iter().map(|v| format!("{} => ...", v)).collect::<Vec<_>>().join(", ")
                        );
                        self.engine.emit(Diagnostic::error(msg, match_span).with_hint(hint));
                    }
                }
            }
            Type::Option(_) => {
                let mut handled_some = false;
                let mut handled_none = false;
                let mut has_wildcard = false;
                for arm in arms {
                    match &arm.pattern {
                        MatchPattern::Wildcard | MatchPattern::Var(_) => has_wildcard = true,
                        MatchPattern::Some(_) => handled_some = true,
                        MatchPattern::None => handled_none = true,
                        _ => {}
                    }
                }
                if !has_wildcard && !(handled_some && handled_none) {
                    let msg = "Non-exhaustive `match` on `Option`: both `Some` and `None` must be covered";
                    self.engine.emit(Diagnostic::error(msg, match_span)
                        .with_hint("Add arms: `Some(v) => ..., None => ...` or use `_ => ...`"));
                }
            }
            Type::Result(_, _) => {
                let mut handled_ok = false;
                let mut handled_err = false;
                let mut has_wildcard = false;
                for arm in arms {
                    match &arm.pattern {
                        MatchPattern::Wildcard | MatchPattern::Var(_) => has_wildcard = true,
                        MatchPattern::Ok(_) => handled_ok = true,
                        MatchPattern::Err(_) => handled_err = true,
                        _ => {}
                    }
                }
                if !has_wildcard && !(handled_ok && handled_err) {
                    let msg = "Non-exhaustive `match` on `Result`: both `Ok` and `Err` must be covered";
                    self.engine.emit(Diagnostic::error(msg, match_span)
                        .with_hint("Add arms: `Ok(v) => ..., Err(e) => ...` or use `_ => ...`"));
                }
            }
            Type::Bool => {
                let mut saw_true = false;
                let mut saw_false = false;
                let mut has_wildcard = false;
                for arm in arms {
                    if let MatchPattern::Literal(lit) = &arm.pattern {
                        if let ExprKind::Bool(b) = lit.kind {
                            if b { saw_true = true; } else { saw_false = true; }
                        }
                    }
                    if let MatchPattern::Wildcard | MatchPattern::Var(_) = &arm.pattern {
                        has_wildcard = true;
                    }
                }
                if !has_wildcard && !(saw_true && saw_false) {
                    self.engine.emit(Diagnostic::error(
                        "Non-exhaustive `match` on `Bool`: both `true` and `false` must be covered",
                        match_span,
                    ).with_hint("Use `true => ..., false => ...` or `_ => ...`"));
                }
            }
            _ => {}
        }
    }

    fn analyze_expr(&mut self, expr: &SpannedExpr) -> Type {
        let result = match &expr.kind {
            ExprKind::Int(_) => Type::Int,
            ExprKind::Float(_) => Type::Float,
            ExprKind::Str(_) => Type::String,
            ExprKind::Char(_) => Type::Char,
            ExprKind::Bool(_) => Type::Bool,
            ExprKind::Null => Type::Any,
            ExprKind::Some(inner) => {
                let inner_ty = self.analyze_expr(inner);
                Type::Option(Box::new(inner_ty))
            }
            ExprKind::None => Type::Option(Box::new(self.symbol_table.fresh_type_var())),
            ExprKind::Ok(inner) => {
                let inner_ty = self.analyze_expr(inner);
                Type::Result(Box::new(inner_ty), Box::new(self.symbol_table.fresh_type_var()))
            }
            ExprKind::Err(inner) => {
                let inner_ty = self.analyze_expr(inner);
                Type::Result(Box::new(self.symbol_table.fresh_type_var()), Box::new(inner_ty))
            }
            ExprKind::Task(inner) => {
                let inner_ty = self.analyze_expr(inner);
                Type::Task(Box::new(inner_ty))
            }
            ExprKind::Await(inner) => {
                let inner_ty = self.analyze_expr(inner);
                match inner_ty {
                    Type::Task(t) => *t,
                    Type::Any => Type::Any,
                    Type::TypeVar(_) => {
                        let t_inner = self.symbol_table.fresh_type_var();
                        let _ = self.unify(&inner_ty, &Type::Task(Box::new(t_inner.clone())), expr.span);
                        t_inner
                    }
                    other => {
                        self.engine.emit(Diagnostic::error(
                            format!("'await' can only be applied to Task<T>, got {}", other),
                            expr.span,
                        ));
                        Type::Any
                    }
                }
            }
            ExprKind::Spawn { callee, args } => {
                let fn_sym = match self.symbol_table.lookup_function(callee).cloned() {
                    Some(s) => s,
                    None => {
                        self.engine.emit(Diagnostic::error(
                            format!("Undeclared function '{}' in spawn", callee),
                            expr.span,
                        ));
                        return Type::Task(Box::new(Type::Any));
                    }
                };
                for arg in args {
                    self.analyze_expr(arg);
                }
                Type::Task(Box::new(fn_sym.ret_type))
            }
            ExprKind::MakeChan(type_node) => {
                let elem_ty = Type::from_node(type_node, &self.symbol_table.structs, &self.symbol_table.enums).unwrap_or(Type::Any);
                Type::Chan(Box::new(elem_ty))
            }
            ExprKind::ChanSend { chan, value } => {
                let chan_ty = self.analyze_expr(chan);
                let val_ty = self.analyze_expr(value);
                if let Type::Chan(ref elem_ty) = chan_ty {
                    let _ = self.unify(&val_ty, elem_ty, expr.span);
                }
                Type::Bool
            }
            ExprKind::ChanRecv(chan) => {
                let chan_ty = self.analyze_expr(chan);
                match chan_ty {
                    Type::Chan(elem_ty) => *elem_ty,
                    Type::Any => Type::Any,
                    Type::TypeVar(_) => {
                        let t_inner = self.symbol_table.fresh_type_var();
                        let _ = self.unify(&chan_ty, &Type::Chan(Box::new(t_inner.clone())), expr.span);
                        t_inner
                    }
                    other => {
                        self.engine.emit(Diagnostic::error(
                            format!("Cannot receive from non-channel type {}", other),
                            expr.span,
                        ));
                        Type::Any
                    }
                }
            }
            ExprKind::Borrow { expr: sub_expr, is_mut } => {
                let inner_ty = self.analyze_expr(sub_expr);
                if let ExprKind::Var(ref name) = sub_expr.kind {
                    if let Some(var) = self.symbol_table.get_variable_mut(name) {
                        if *is_mut && !var.is_mut {
                            self.engine.emit(Diagnostic::error(
                                format!("Cannot borrow immutable local variable '{}' as mutable", name),
                                expr.span,
                            ));
                        }
                        if *is_mut {
                            if var.borrow_count > 0 || var.is_mut_borrowed {
                                self.engine.emit(Diagnostic::error(
                                    format!("Cannot borrow '{}' as mutable more than once at a time", name),
                                    expr.span,
                                ));
                            }
                            var.is_mut_borrowed = true;
                        } else {
                            if var.is_mut_borrowed {
                                self.engine.emit(Diagnostic::error(
                                    format!("Cannot borrow '{}' as immutable because it is already borrowed as mutable", name),
                                    expr.span,
                                ));
                            }
                            var.borrow_count += 1;
                        }
                    }
                }
                Type::Ref(Box::new(inner_ty), *is_mut)
            }
            ExprKind::Move(inner) => {
                // Explicit ownership transfer (Owned mode): the operand
                // becomes moved after this expression is evaluated.
                let inner_ty = self.analyze_expr(inner);
                if let ExprKind::Var(ref name) = inner.kind {
                    if let Some(var) = self.symbol_table.get_variable_mut(name) {
                        if !var.is_moved {
                            var.is_moved = true;
                            self.engine.emit(Diagnostic {
                                level: crate::diagnostic::DiagnosticLevel::Note,
                                message: format!("Ownership of `{}` explicitly moved (move {})", name, name),
                                span: expr.span,
                                hint: Some("In Owned mode the old binding can no longer be used.".to_string()),
                            });
                        }
                    }
                }
                inner_ty
            }
            ExprKind::UnsafeBlock(stmts) => {
                // Unsafe block as an expression: statements are analyzed
                // and the block yields the type of its last expression.
                self.symbol_table.enter_scope(false);
                let mut last = Type::Void;
                for s in stmts {
                    self.analyze_statement(s);
                    if let StmtKind::Expr(e) = &s.kind {
                        let expr_ty = self.analyze_expr(e);
                        last = self.substitution.apply(&expr_ty);
                    }
                }
                self.symbol_table.exit_scope();
                last
            }
            ExprKind::Var(name) => match self.symbol_table.get_variable_type(name) {
                Some(ty) => {
                    if let Some(var) = self.symbol_table.get_variable(name) {
                        if var.is_moved {
                            self.engine.emit(
                                Diagnostic::error(
                                    format!("Use of moved value: '{}'", name),
                                    expr.span,
                                )
                                .with_hint(format!("Value was previously moved. Consider borrowing with '&{}'", name)),
                            );
                        }
                    }
                    ty
                }
                None => {
                    self.engine.emit(Diagnostic::error(
                        format!("Undeclared variable '{}'", name),
                        expr.span,
                    ));
                    Type::Any
                }
            },
            ExprKind::IsA { value, type_node: _ } => {
                self.analyze_expr(value);
                Type::Bool
            }
            ExprKind::EnumVariantConstruct { enum_name, variant_name, payload } => {
                let variant_payload_ty = if let Some(p) = payload {
                    let ty = self.analyze_expr(p);
                    Some(ty)
                } else {
                    None
                };

                let known_enum = self.symbol_table.enums.get(enum_name).cloned();
                let variants_map: HashMap<String, Option<Type>> = known_enum.clone().unwrap_or_else(|| {
                    let mut m = HashMap::new();
                    m.insert(variant_name.clone(), variant_payload_ty.clone());
                    m
                });

                if let Some(var_map) = &known_enum {
                    if let Some(expected_payload) = var_map.get(variant_name) {
                        match (expected_payload, &variant_payload_ty) {
                            (Some(exp), Some(got)) => {
                                let _ = self.unify(got, exp, expr.span);
                            }
                            (None, Some(_)) => {
                                self.engine.emit(Diagnostic::error(
                                    format!("Variant `{}::{}` takes no payload, but one provided", enum_name, variant_name),
                                    expr.span,
                                ));
                            }
                            (Some(_), None) => {
                                self.engine.emit(Diagnostic::error(
                                    format!("Variant `{}::{}` requires a payload", enum_name, variant_name),
                                    expr.span,
                                ));
                            }
                            _ => {}
                        }
                    }
                }

                self.engine.emit(Diagnostic {
                    level: crate::diagnostic::DiagnosticLevel::Note,
                    message: format!("Constructed ADT variant `{}::{}`{}",
                        enum_name, variant_name,
                        variant_payload_ty.as_ref().map(|t| format!(" with payload {}", t)).unwrap_or_default()
                    ),
                    span: expr.span,
                    hint: None,
                });

                Type::Enum(enum_name.clone(), variants_map)
            }
            ExprKind::Binary { left, op, right } => {
                let lt = self.analyze_expr(left);
                let rt = self.analyze_expr(right);

                match op {
                    BinaryOpKind::Add
                    | BinaryOpKind::Sub
                    | BinaryOpKind::Mul
                    | BinaryOpKind::Div
                    | BinaryOpKind::Mod => {
                        let result_tv = self.symbol_table.fresh_type_var();
                        let (int_unified, _) = (
                            self.unify(&lt, &Type::Int, left.span).is_ok() && self.unify(&rt, &Type::Int, right.span).is_ok(),
                            ()
                        );
                        if int_unified {
                            self.substitution = Substitution::new();
                            let _ = self.unify(&lt, &Type::Int, left.span);
                            let _ = self.unify(&rt, &Type::Int, right.span);
                            let _ = self.unify(&result_tv, &Type::Int, expr.span);
                            Type::Int
                        } else if self.substitution.apply(&lt) == Type::Float && self.substitution.apply(&rt) == Type::Float {
                            Type::Float
                        } else if *op == BinaryOpKind::Add && (self.substitution.apply(&lt) == Type::String || matches!(self.substitution.apply(&lt), Type::Ref(ref b, _) if **b == Type::String)) {
                            Type::String
                        } else if let Some(ret_ty) = self.resolve_operator(
                            match op {
                                BinaryOpKind::Add => "+",
                                BinaryOpKind::Sub => "-",
                                BinaryOpKind::Mul => "*",
                                BinaryOpKind::Div => "/",
                                BinaryOpKind::Mod => "%",
                                _ => unreachable!(),
                            },
                            &lt,
                            &rt,
                            expr.span,
                        ) {
                            ret_ty
                        } else {
                            self.pending_trait_checks.push((lt.clone(), "Addable".to_string(), left.span));
                            self.pending_trait_checks.push((rt.clone(), "Addable".to_string(), right.span));
                            let a = self.substitution.apply(&lt);
                            let b = self.substitution.apply(&rt);
                            self.engine.emit(Diagnostic::error(
                                format!(
                                    "Operator '{:?}' not supported between {} and {}",
                                    op, a, b
                                ),
                                expr.span,
                            ));
                            result_tv
                        }
                    }
                    BinaryOpKind::Equal
                    | BinaryOpKind::NotEqual
                    | BinaryOpKind::Less
                    | BinaryOpKind::LessEqual
                    | BinaryOpKind::Greater
                    | BinaryOpKind::GreaterEqual => {
                        self.pending_trait_checks.push((lt.clone(), "Comparable".to_string(), left.span));
                        self.pending_trait_checks.push((rt.clone(), "Comparable".to_string(), right.span));
                        let _ = self.unify(&lt, &rt, expr.span);
                        Type::Bool
                    }
                    BinaryOpKind::And | BinaryOpKind::Or => {
                        let _ = self.unify(&lt, &Type::Bool, left.span);
                        let _ = self.unify(&rt, &Type::Bool, right.span);
                        Type::Bool
                    }
                }
            }
            ExprKind::Unary { op, expr: sub_expr } => {
                let ty = self.analyze_expr(sub_expr);
                match op {
                    UnaryOpKind::Neg => {
                        let unified = self.substitution.apply(&ty);
                        if unified == Type::Int || unified == Type::Float || unified == Type::Any || matches!(unified, Type::TypeVar(_)) {
                            ty
                        } else {
                            self.engine.emit(Diagnostic::error(
                                format!("Negation '-' not supported for {}", ty),
                                expr.span,
                            ));
                            Type::Any
                        }
                    }
                    UnaryOpKind::Not => {
                        let _ = self.unify(&ty, &Type::Bool, sub_expr.span);
                        Type::Bool
                    }
                    UnaryOpKind::Deref => {
                        if let Type::Ref(inner, _) = self.substitution.apply(&ty) {
                            *inner
                        } else {
                            ty
                        }
                    }
                }
            }
            ExprKind::Call { callee, args } => {
                if callee == "println" {
                    for arg in args {
                        let arg_ty = self.analyze_expr(arg);
                        self.pending_trait_checks.push((arg_ty, "Display".to_string(), arg.span));
                    }
                    Type::Void
                } else if let Some(method) = callee.strip_prefix("__aether_method::") {
                    let receiver = match args.first() {
                        Some(receiver) => receiver,
                        None => {
                            self.engine.emit(Diagnostic::error(
                                format!("Method '{}' requires a receiver", method),
                                expr.span,
                            ));
                            return Type::Any;
                        }
                    };
                    let receiver_ty_raw = self.analyze_expr(receiver);
                    let receiver_ty = self.substitution.apply(&receiver_ty_raw);
                    match (method, receiver_ty) {
                        ("length", Type::String) => Type::Int,
                        ("length", Type::Ref(inner, _)) if *inner == Type::String => Type::Int,
                        ("length", Type::Array(_)) | ("length", Type::Set(_)) | ("length", Type::Map(_, _)) => Type::Int,
                        ("length", Type::Any) | (_, Type::Any) => Type::Any,
                        ("length", other) => {
                            self.engine.emit(Diagnostic::error(
                                format!("Method 'length' is not available on type {}", other),
                                receiver.span,
                            ));
                            Type::Any
                        }
                        (other, receiver_ty) => {
                            self.engine.emit(Diagnostic::error(
                                format!("Unknown method '{}' for type {}", other, receiver_ty),
                                receiver.span,
                            ));
                            Type::Any
                        }
                    }
                } else {
                match self.symbol_table.lookup_function(callee).cloned() {
                    Some(fn_sym) => {
                        if fn_sym.params.len() != args.len() {
                            self.engine.emit(Diagnostic::error(
                                format!(
                                    "Function '{}' expects {} arguments, but got {}",
                                    callee,
                                    fn_sym.params.len(),
                                    args.len()
                                ),
                                expr.span,
                            ));
                        }

                        let (instantiated_fn, _mapping) = self.instantiate_generic(
                            &Type::Function(
                                fn_sym.params.iter().map(|(_, t)| t.clone()).collect(),
                                Box::new(fn_sym.ret_type.clone()),
                            ),
                            &fn_sym.type_params,
                            expr.span,
                        );

                        let (expected_params, expected_ret) = match instantiated_fn {
                            Type::Function(p, r) => (p, *r),
                            _ => (Vec::new(), Type::Any),
                        };

                        let mut type_args = Vec::new();
                        for (idx, (arg_expr, expected_ty)) in
                            args.iter().zip(expected_params.iter()).enumerate()
                        {
                            let actual_ty = self.analyze_expr(arg_expr);
                            type_args.push((actual_ty.clone(), arg_expr.span));
                            if let Err(e) = self.unify(&actual_ty, expected_ty, arg_expr.span) {
                                let got = self.substitution.apply(&actual_ty);
                                let exp = self.substitution.apply(expected_ty);
                                self.engine.emit(Diagnostic::error(
                                    format!(
                                        "Arg #{} in call to '{}': expected {}, got {} ({})",
                                        idx + 1,
                                        callee,
                                        exp,
                                        got,
                                        e
                                    ),
                                    arg_expr.span,
                                ));
                            }
                        }

                        if !fn_sym.type_params.is_empty() {
                            self.call_site_instantiations.push((callee.clone(), type_args));
                        }

                        for (_, (_, bounds)) in fn_sym.type_params.iter().enumerate() {
                            let applied = self.substitution.apply(&expected_ret);
                            for bound in bounds {
                                self.pending_trait_checks.push((applied.clone(), bound.clone(), expr.span));
                            }
                        }

                        self.substitution.apply(&expected_ret)
                    }
                    None => {
                        self.engine.emit(Diagnostic::error(
                            format!("Undeclared function '{}'", callee),
                            expr.span,
                        ));
                        Type::Any
                    }
                }
                }
            }
            ExprKind::Array(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(self.symbol_table.fresh_type_var()))
                } else {
                    let elem_tv = self.symbol_table.fresh_type_var();
                    for elem in elements {
                        let ty = self.analyze_expr(elem);
                        if let Err(_) = self.unify(&ty, &elem_tv, elem.span) {
                            let a = self.substitution.apply(&ty);
                            let b = self.substitution.apply(&elem_tv);
                            self.engine.emit(Diagnostic::error(
                                format!("Array elements must have uniform type. Found {} and {}", b, a),
                                elem.span,
                            ));
                        }
                    }
                    Type::Array(Box::new(self.substitution.apply(&elem_tv)))
                }
            }
            ExprKind::Map(entries) => {
                if entries.is_empty() {
                    Type::Map(Box::new(self.symbol_table.fresh_type_var()), Box::new(self.symbol_table.fresh_type_var()))
                } else {
                    let ktv = self.symbol_table.fresh_type_var();
                    let vtv = self.symbol_table.fresh_type_var();
                    for (k, v) in entries {
                        let k_ty = self.analyze_expr(k);
                        let v_ty = self.analyze_expr(v);
                        let _ = self.unify(&k_ty, &ktv, k.span);
                        let _ = self.unify(&v_ty, &vtv, v.span);
                        self.pending_trait_checks.push((k_ty, "Hashable".to_string(), k.span));
                    }
                    Type::Map(
                        Box::new(self.substitution.apply(&ktv)),
                        Box::new(self.substitution.apply(&vtv)),
                    )
                }
            }
            ExprKind::Set(elements) => {
                if elements.is_empty() {
                    Type::Set(Box::new(self.symbol_table.fresh_type_var()))
                } else {
                    let etv = self.symbol_table.fresh_type_var();
                    for elem in elements {
                        let ty = self.analyze_expr(elem);
                        let _ = self.unify(&ty, &etv, elem.span);
                        self.pending_trait_checks.push((ty, "Hashable".to_string(), elem.span));
                    }
                    Type::Set(Box::new(self.substitution.apply(&etv)))
                }
            }
            ExprKind::Index { target, index } => {
                let target_ty = self.analyze_expr(target);
                let idx_ty = self.analyze_expr(index);
                let result_tv = self.symbol_table.fresh_type_var();
                let resolved = self.substitution.apply(&target_ty);
                match resolved {
                    Type::Array(inner) => {
                        let _ = self.unify(&idx_ty, &Type::Int, index.span);
                        let _ = self.unify(&result_tv, &inner, expr.span);
                        *inner
                    }
                    Type::Map(_, v) => {
                        let _ = self.unify(&result_tv, &v, expr.span);
                        *v
                    }
                    Type::Any => {
                        let _ = self.unify(&idx_ty, &Type::Any, index.span);
                        Type::Any
                    }
                    Type::TypeVar(_) => {
                        let arr_inner = self.symbol_table.fresh_type_var();
                        let _ = self.unify(&target_ty, &Type::Array(Box::new(arr_inner.clone())), target.span);
                        arr_inner
                    }
                    other => {
                        self.engine.emit(Diagnostic::error(
                            format!("Cannot index into non-collection type {}", other),
                            target.span,
                        ));
                        Type::Any
                    }
                }
            }
            ExprKind::Dot { target, field } => {
                let target_ty = self.analyze_expr(target);
                let resolved = self.substitution.apply(&target_ty);
                match resolved {
                    Type::Struct(_, ref fields) => {
                        if let Some(f_ty) = fields.get(field) {
                            f_ty.clone()
                        } else {
                            self.engine.emit(Diagnostic::error(
                                format!("Field '{}' does not exist on struct", field),
                                expr.span,
                            ));
                            Type::Any
                        }
                    }
                    Type::Any => Type::Any,
                    Type::TypeVar(_) => {
                        self.symbol_table.fresh_type_var()
                    }
                    other => {
                        self.engine.emit(Diagnostic::error(
                            format!("Cannot access field '{}' on non-struct type {}", field, other),
                            expr.span,
                        ));
                        Type::Any
                    }
                }
            }
            ExprKind::Match { value, arms } => {
                let val_ty = self.analyze_expr(value);
                let ret_ty = self.symbol_table.fresh_type_var();

                self.check_match_exhaustiveness(arms, &val_ty, expr.span);

                for (idx, arm) in arms.iter().enumerate() {
                    self.symbol_table.enter_scope(false);
                    match &arm.pattern {
                        MatchPattern::Some(var) => {
                            let inner_ty = match self.substitution.apply(&val_ty) {
                                Type::Option(ref inner) => (**inner).clone(),
                                Type::TypeVar(_) => self.symbol_table.fresh_type_var(),
                                _ => Type::Any,
                            };
                            let _ = self.symbol_table.define_variable(VariableSymbol {
                                name: var.clone(),
                                ty: inner_ty,
                                is_mut: false,
                                is_moved: false,
                                borrow_count: 0,
                                is_mut_borrowed: false,
                                span: arm.span,
                                narrowed_type: None,
                            });
                        }
                        MatchPattern::Ok(var) => {
                            let inner_ty = match self.substitution.apply(&val_ty) {
                                Type::Result(ref ok, _) => (**ok).clone(),
                                Type::TypeVar(_) => self.symbol_table.fresh_type_var(),
                                _ => Type::Any,
                            };
                            let _ = self.symbol_table.define_variable(VariableSymbol {
                                name: var.clone(),
                                ty: inner_ty,
                                is_mut: false,
                                is_moved: false,
                                borrow_count: 0,
                                is_mut_borrowed: false,
                                span: arm.span,
                                narrowed_type: None,
                            });
                        }
                        MatchPattern::Err(var) => {
                            let inner_ty = match self.substitution.apply(&val_ty) {
                                Type::Result(_, ref err) => (**err).clone(),
                                Type::TypeVar(_) => self.symbol_table.fresh_type_var(),
                                _ => Type::Any,
                            };
                            let _ = self.symbol_table.define_variable(VariableSymbol {
                                name: var.clone(),
                                ty: inner_ty,
                                is_mut: false,
                                is_moved: false,
                                borrow_count: 0,
                                is_mut_borrowed: false,
                                span: arm.span,
                                narrowed_type: None,
                            });
                        }
                        MatchPattern::Var(var) => {
                            let _ = self.symbol_table.define_variable(VariableSymbol {
                                name: var.clone(),
                                ty: self.substitution.apply(&val_ty),
                                is_mut: false,
                                is_moved: false,
                                borrow_count: 0,
                                is_mut_borrowed: false,
                                span: arm.span,
                                narrowed_type: None,
                            });
                        }
                        MatchPattern::EnumVariant(variant, binding) => {
                            let resolved_val = self.substitution.apply(&val_ty);
                            if let Type::Enum(_, ref variants) = resolved_val {
                                let payload_ty = variants.get(variant).cloned().flatten().unwrap_or(Type::Any);
                                if let Some(bind_name) = binding {
                                    let _ = self.symbol_table.define_variable(VariableSymbol {
                                        name: bind_name.clone(),
                                        ty: payload_ty,
                                        is_mut: false,
                                        is_moved: false,
                                        borrow_count: 0,
                                        is_mut_borrowed: false,
                                        span: arm.span,
                                        narrowed_type: None,
                                    });
                                }
                            }
                        }
                        MatchPattern::EnumVariantStruct(variant, field_binds) => {
                            let resolved_val = self.substitution.apply(&val_ty);
                            if let Type::Enum(_, ref variants) = resolved_val {
                                if let Some(Some(Type::Struct(_, struct_fields))) = variants.get(variant) {
                                    for (field_name, pat_opt) in field_binds {
                                        let f_ty = struct_fields.get(field_name).cloned().unwrap_or(Type::Any);
                                        match pat_opt {
                                            Some(MatchPattern::Var(bind_name)) => {
                                                let _ = self.symbol_table.define_variable(VariableSymbol {
                                                    name: bind_name.clone(),
                                                    ty: f_ty,
                                                    is_mut: false,
                                                    is_moved: false,
                                                    borrow_count: 0,
                                                    is_mut_borrowed: false,
                                                    span: arm.span,
                                                    narrowed_type: None,
                                                });
                                            }
                                            None => {
                                                let _ = self.symbol_table.define_variable(VariableSymbol {
                                                    name: field_name.clone(),
                                                    ty: f_ty,
                                                    is_mut: false,
                                                    is_moved: false,
                                                    borrow_count: 0,
                                                    is_mut_borrowed: false,
                                                    span: arm.span,
                                                    narrowed_type: None,
                                                });
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    let arm_body_ty = self.analyze_expr(&arm.body);
                    if idx == 0 {
                        let _ = self.unify(&arm_body_ty, &ret_ty, arm.span);
                    } else {
                        let _ = self.unify(&arm_body_ty, &ret_ty, arm.span);
                    }
                    self.symbol_table.exit_scope();
                }

                self.substitution.apply(&ret_ty)
            }
            ExprKind::CustomBinary { left, operator, right } => {
                let lt = self.analyze_expr(left);
                let rt = self.analyze_expr(right);
                self.resolve_operator(operator, &lt, &rt, expr.span).unwrap_or(Type::Any)
            }
            ExprKind::Block(stmts) => {
                self.symbol_table.enter_scope(false);
                let mut last = Type::Void;
                for s in stmts {
                    self.analyze_statement(s);
                    if let StmtKind::Expr(e) = &s.kind {
                        let expr_ty = self.analyze_expr(e);
                        last = self.substitution.apply(&expr_ty);
                    }
                }
                self.symbol_table.exit_scope();
                last
            }
            ExprKind::Tuple(items) => {
                Type::Tuple(items.iter().map(|i| self.analyze_expr(i)).collect())
            }
        };

        let final_result = self.substitution.apply(&result);
        self.inferred_annotations.insert(expr.span, final_result.clone());
        final_result
    }
}

pub fn check_semantics(program: &Program) -> Result<(SymbolTable, HashMap<Span, Type>), DiagnosticEngine> {
    SemanticAnalyzer::new().analyze(program)
}
