#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use crate::ast::TypeNode;
use crate::diagnostic::Span;

pub type TypeVarId = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>),
    Slice(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Union(Vec<Type>),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Generic(String),
    GenericInstance(String, Vec<Type>),
    Ref(Box<Type>, bool),
    Ptr(Box<Type>, bool),
    Task(Box<Type>),
    Chan(Box<Type>),
    Struct(String, HashMap<String, Type>),
    Enum(String, HashMap<String, Option<Type>>),
    Void,
    Any,
    TypeVar(TypeVarId),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Char => write!(f, "Char"),
            Type::Void => write!(f, "Void"),
            Type::Any => write!(f, "Any"),
            Type::TypeVar(id) => write!(f, "τ{}", id),
            Type::Tuple(items) => {
                let parts: Vec<String> = items.iter().map(|t| format!("{}", t)).collect();
                write!(f, "({})", parts.join(", "))
            }
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::Slice(inner) => write!(f, "&[{}]", inner),
            Type::Map(k, v) => write!(f, "Map<{}, {}>", k, v),
            Type::Set(inner) => write!(f, "Set<{}>", inner),
            Type::Option(inner) => write!(f, "Option<{}>", inner),
            Type::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            Type::Function(params, ret) => {
                let ps: Vec<String> = params.iter().map(|t| format!("{}", t)).collect();
                write!(f, "fn({}) -> {}", ps.join(", "), ret)
            }
            Type::Generic(name) => write!(f, "{}", name),
            Type::GenericInstance(name, args) => {
                let as_: Vec<String> = args.iter().map(|t| format!("{}", t)).collect();
                write!(f, "{}<{}>", name, as_.join(", "))
            }
            Type::Ref(inner, is_mut) => {
                if *is_mut { write!(f, "&mut {}", inner) } else { write!(f, "&{}", inner) }
            }
            Type::Ptr(inner, is_const) => {
                if *is_const { write!(f, "*const {}", inner) } else { write!(f, "*mut {}", inner) }
            }
            Type::Task(inner) => write!(f, "Task<{}>", inner),
            Type::Chan(inner) => write!(f, "Chan<{}>", inner),
            Type::Struct(name, _) => write!(f, "{}", name),
            Type::Enum(name, _) => write!(f, "{}", name),
            Type::Union(members) => {
                let ms: Vec<String> = members.iter().map(|t| format!("{}", t)).collect();
                write!(f, "{}", ms.join(" | "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Substitution {
    pub map: HashMap<TypeVarId, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::TypeVar(id) => {
                if let Some(replacement) = self.map.get(id) {
                    self.apply(replacement)
                } else {
                    ty.clone()
                }
            }
            Type::Tuple(items) => {
                Type::Tuple(items.iter().map(|t| self.apply(t)).collect())
            }
            Type::Array(inner) => Type::Array(Box::new(self.apply(inner))),
            Type::Slice(inner) => Type::Slice(Box::new(self.apply(inner))),
            Type::Map(k, v) => Type::Map(Box::new(self.apply(k)), Box::new(self.apply(v))),
            Type::Set(inner) => Type::Set(Box::new(self.apply(inner))),
            Type::Union(members) => Type::Union(members.iter().map(|t| self.apply(t)).collect()),
            Type::Option(inner) => Type::Option(Box::new(self.apply(inner))),
            Type::Result(ok, err) => Type::Result(Box::new(self.apply(ok)), Box::new(self.apply(err))),
            Type::Function(params, ret) => {
                Type::Function(
                    params.iter().map(|t| self.apply(t)).collect(),
                    Box::new(self.apply(ret)),
                )
            }
            Type::GenericInstance(name, args) => {
                Type::GenericInstance(
                    name.clone(),
                    args.iter().map(|t| self.apply(t)).collect(),
                )
            }
            Type::Ref(inner, is_mut) => Type::Ref(Box::new(self.apply(inner)), *is_mut),
            Type::Ptr(inner, is_const) => Type::Ptr(Box::new(self.apply(inner)), *is_const),
            Type::Task(inner) => Type::Task(Box::new(self.apply(inner))),
            Type::Chan(inner) => Type::Chan(Box::new(self.apply(inner))),
            Type::Struct(name, fields) => {
                let mut new_fields = HashMap::new();
                for (k, v) in fields {
                    new_fields.insert(k.clone(), self.apply(v));
                }
                Type::Struct(name.clone(), new_fields)
            }
            Type::Enum(name, variants) => {
                let mut new_variants = HashMap::new();
                for (k, v) in variants {
                    new_variants.insert(k.clone(), v.as_ref().map(|t| self.apply(t)));
                }
                Type::Enum(name.clone(), new_variants)
            }
            other => other.clone(),
        }
    }

    pub fn compose(self, other: Substitution) -> Substitution {
        let mut new_map = other.map.clone();
        for (k, v) in self.map {
            new_map.insert(k, other.apply(&v));
        }
        Substitution { map: new_map }
    }

    pub fn bind(&mut self, id: TypeVarId, ty: Type) -> Result<(), String> {
        if let Type::TypeVar(other_id) = ty {
            if other_id == id {
                return Ok(());
            }
        }
        if self.occurs_check(id, &ty) {
            return Err(format!("Infinite type: τ{} = {}", id, ty));
        }
        self.map.insert(id, ty);
        Ok(())
    }

    fn occurs_check(&self, id: TypeVarId, ty: &Type) -> bool {
        match ty {
            Type::TypeVar(tv_id) => {
                if *tv_id == id {
                    return true;
                }
                if let Some(replacement) = self.map.get(tv_id) {
                    self.occurs_check(id, replacement)
                } else {
                    false
                }
            }
            Type::Tuple(items) => items.iter().any(|t| self.occurs_check(id, t)),
            Type::Array(inner) | Type::Slice(inner) | Type::Set(inner)
            | Type::Option(inner) | Type::Task(inner) | Type::Chan(inner) => {
                self.occurs_check(id, inner)
            }
            Type::Map(k, v) | Type::Result(k, v) => self.occurs_check(id, k) || self.occurs_check(id, v),
            Type::Union(members) => members.iter().any(|t| self.occurs_check(id, t)),
            Type::Function(params, ret) => {
                params.iter().any(|t| self.occurs_check(id, t)) || self.occurs_check(id, ret)
            }
            Type::GenericInstance(_, args) => args.iter().any(|t| self.occurs_check(id, t)),
            Type::Ref(inner, _) | Type::Ptr(inner, _) => self.occurs_check(id, inner),
            Type::Struct(_, fields) => fields.values().any(|t| self.occurs_check(id, t)),
            Type::Enum(_, variants) => variants.values().any(|opt| {
                opt.as_ref().map(|t| self.occurs_check(id, t)).unwrap_or(false)
            }),
            _ => false,
        }
    }
}

impl Type {
    pub fn free_type_vars(&self) -> HashSet<TypeVarId> {
        let mut vars = HashSet::new();
        self.collect_type_vars(&mut vars);
        vars
    }

    fn collect_type_vars(&self, set: &mut HashSet<TypeVarId>) {
        match self {
            Type::TypeVar(id) => { set.insert(*id); }
            Type::Tuple(items) => items.iter().for_each(|t| t.collect_type_vars(set)),
            Type::Array(inner) | Type::Slice(inner) | Type::Set(inner)
            | Type::Option(inner) | Type::Task(inner) | Type::Chan(inner) => {
                inner.collect_type_vars(set)
            }
            Type::Map(k, v) | Type::Result(k, v) => {
                k.collect_type_vars(set);
                v.collect_type_vars(set);
            }
            Type::Union(members) => members.iter().for_each(|t| t.collect_type_vars(set)),
            Type::Function(params, ret) => {
                params.iter().for_each(|t| t.collect_type_vars(set));
                ret.collect_type_vars(set);
            }
            Type::GenericInstance(_, args) => args.iter().for_each(|t| t.collect_type_vars(set)),
            Type::Ref(inner, _) | Type::Ptr(inner, _) => inner.collect_type_vars(set),
            Type::Struct(_, fields) => fields.values().for_each(|t| t.collect_type_vars(set)),
            Type::Enum(_, variants) => variants.values().for_each(|opt| {
                if let Some(t) = opt { t.collect_type_vars(set); }
            }),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraitImpl {
    pub trait_name: String,
    pub type_params: Vec<(String, Vec<String>)>,
    pub target_type: Type,
    pub impl_items: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GenericVarInfo {
    pub name: String,
    pub bounds: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GenericEnv {
    pub vars: HashMap<String, Type>,
    pub generic_vars: Vec<GenericVarInfo>,
}

impl GenericEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            generic_vars: Vec::new(),
        }
    }

    pub fn push_generic(&mut self, name: String, bounds: Vec<String>) {
        self.generic_vars.push(GenericVarInfo { name: name.clone(), bounds });
    }

    pub fn lookup_bound(&self, name: &str) -> Option<&[String]> {
        self.generic_vars.iter()
            .find(|v| v.name == name)
            .map(|v| v.bounds.as_slice())
    }
}

impl Type {
    pub fn is_copy(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Bool | Type::Char | Type::Ref(_, _) | Type::Ptr(_, _) | Type::TypeVar(_))
    }

    pub fn from_node(node: &TypeNode, structs: &HashMap<String, HashMap<String, Type>>, enums: &HashMap<String, HashMap<String, Option<Type>>>) -> Result<Self, String> {
        match node {
            TypeNode::Int => Ok(Type::Int),
            TypeNode::Float => Ok(Type::Float),
            TypeNode::Bool => Ok(Type::Bool),
            TypeNode::String => Ok(Type::String),
            TypeNode::Char => Ok(Type::Char),
            TypeNode::Void => Ok(Type::Void),
            TypeNode::Infer => Ok(Type::Any),
            TypeNode::Tuple(items) => {
                let mut typed_items = Vec::new();
                for item in items {
                    typed_items.push(Type::from_node(item, structs, enums)?);
                }
                Ok(Type::Tuple(typed_items))
            }
            TypeNode::Array(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Array(Box::new(inner_ty)))
            }
            TypeNode::Slice(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Slice(Box::new(inner_ty)))
            }
            TypeNode::Map(k, v) => {
                let k_ty = Type::from_node(k, structs, enums)?;
                let v_ty = Type::from_node(v, structs, enums)?;
                Ok(Type::Map(Box::new(k_ty), Box::new(v_ty)))
            }
            TypeNode::Set(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Set(Box::new(inner_ty)))
            }
            TypeNode::Union(members) => {
                let mut typed_members = Vec::new();
                for member in members {
                    typed_members.push(Type::from_node(member, structs, enums)?);
                }
                Ok(Type::Union(typed_members))
            }
            TypeNode::Option(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Option(Box::new(inner_ty)))
            }
            TypeNode::Result(ok, err) => {
                let ok_ty = Type::from_node(ok, structs, enums)?;
                let err_ty = Type::from_node(err, structs, enums)?;
                Ok(Type::Result(Box::new(ok_ty), Box::new(err_ty)))
            }
            TypeNode::Function { params, ret } => {
                let mut typed_params = Vec::new();
                for param in params {
                    typed_params.push(Type::from_node(param, structs, enums)?);
                }
                let ret_ty = Type::from_node(ret, structs, enums)?;
                Ok(Type::Function(typed_params, Box::new(ret_ty)))
            }
            TypeNode::Generic { name, args } => {
                let mut typed_args = Vec::new();
                for arg in args {
                    typed_args.push(Type::from_node(arg, structs, enums)?);
                }
                Ok(Type::GenericInstance(name.clone(), typed_args))
            }
            TypeNode::Ref(inner, is_mut) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Ref(Box::new(inner_ty), *is_mut))
            }
            TypeNode::Ptr(inner, is_const) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Ptr(Box::new(inner_ty), *is_const))
            }
            TypeNode::Task(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Task(Box::new(inner_ty)))
            }
            TypeNode::Chan(inner) => {
                let inner_ty = Type::from_node(inner, structs, enums)?;
                Ok(Type::Chan(Box::new(inner_ty)))
            }
            TypeNode::Custom(name) => {
                if let Some(fields) = structs.get(name) {
                    Ok(Type::Struct(name.clone(), fields.clone()))
                } else if let Some(variants) = enums.get(name) {
                    Ok(Type::Enum(name.clone(), variants.clone()))
                } else {
                    Ok(Type::Generic(name.clone()))
                }
            }
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if self == target || *self == Type::Any || *target == Type::Any {
            return true;
        }
        if let (Type::TypeVar(_), _) | (_, Type::TypeVar(_)) = (self, target) {
            return true;
        }
        match (self, target) {
            (Type::Ref(a, a_mut), Type::Ref(b, b_mut)) => {
                if *b_mut && !*a_mut {
                    return false;
                }
                a.is_assignable_to(b)
            }
            (Type::Ptr(_, _), Type::Ptr(_, _)) => true,
            (Type::Array(a), Type::Array(b)) => a.is_assignable_to(b),
            (Type::Slice(a), Type::Slice(b)) => a.is_assignable_to(b),
            (Type::Set(a), Type::Set(b)) => a.is_assignable_to(b),
            (Type::Option(a), Type::Option(b)) => a.is_assignable_to(b),
            (Type::Result(a_ok, a_err), Type::Result(b_ok, b_err)) => {
                a_ok.is_assignable_to(b_ok) && a_err.is_assignable_to(b_err)
            }
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                k1.is_assignable_to(k2) && v1.is_assignable_to(v2)
            }
            (Type::Tuple(a), Type::Tuple(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a.is_assignable_to(b))
            }
            (Type::Function(a_params, a_ret), Type::Function(b_params, b_ret)) => {
                a_params.len() == b_params.len()
                    && a_params
                        .iter()
                        .zip(b_params.iter())
                        .all(|(a, b)| a.is_assignable_to(b))
                    && a_ret.is_assignable_to(b_ret)
            }
            (source, Type::Union(members)) => members.iter().any(|member| source.is_assignable_to(member)),
            (Type::Union(members), target) => members.iter().all(|member| member.is_assignable_to(target)),
            (Type::Generic(a), Type::Generic(b)) => a == b,
            (Type::GenericInstance(a_name, a_args), Type::GenericInstance(b_name, b_args)) => {
                a_name == b_name
                    && a_args.len() == b_args.len()
                    && a_args.iter().zip(b_args.iter()).all(|(a, b)| a.is_assignable_to(b))
            }
            (Type::Struct(a_name, _), Type::Struct(b_name, _)) => a_name == b_name,
            (Type::Enum(a_name, _), Type::Enum(b_name, _)) => a_name == b_name,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableSymbol {
    pub name: String,
    pub ty: Type,
    pub is_mut: bool,
    pub is_moved: bool,
    pub borrow_count: usize,
    pub is_mut_borrowed: bool,
    pub span: Span,
    pub narrowed_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: String,
    pub is_async: bool,
    pub is_extern: bool,
    pub type_params: Vec<(String, Vec<String>)>,
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TraitSymbol {
    pub name: String,
    pub associated_types: Vec<String>,
    pub methods: Vec<String>,
    pub span: Span,
    pub super_traits: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OperatorSymbol {
    pub operator: String,
    pub type_params: Vec<(String, Vec<String>)>,
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub variables: HashMap<String, VariableSymbol>,
    pub is_loop_scope: bool,
    pub narrowed_vars: HashMap<String, Type>,
}

impl Scope {
    pub fn new(is_loop_scope: bool) -> Self {
        Self {
            variables: HashMap::new(),
            is_loop_scope,
            narrowed_vars: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    completed_variables: HashMap<String, VariableSymbol>,
    functions: HashMap<String, FunctionSymbol>,
    operators: HashMap<String, OperatorSymbol>,
    pub structs: HashMap<String, HashMap<String, Type>>,
    pub enums: HashMap<String, HashMap<String, Option<Type>>>,
    pub traits: HashMap<String, TraitSymbol>,
    pub trait_impls: Vec<TraitImpl>,
    pub next_type_var: TypeVarId,
    pub current_generic_env: GenericEnv,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = Scope::new(false);
        let mut st = Self {
            scopes: vec![global_scope],
            completed_variables: HashMap::new(),
            functions: HashMap::new(),
            operators: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            traits: HashMap::new(),
            trait_impls: Vec::new(),
            next_type_var: 0,
            current_generic_env: GenericEnv::new(),
        };
        st.register_builtin_traits();
        st
    }

    fn register_builtin_traits(&mut self) {
        self.traits.insert("Comparable".to_string(), TraitSymbol {
            name: "Comparable".to_string(),
            associated_types: vec![],
            methods: vec!["compare".to_string(), "less".to_string(), "greater".to_string(), "equal".to_string()],
            span: Span::dummy(),
            super_traits: vec![],
        });
        self.traits.insert("Addable".to_string(), TraitSymbol {
            name: "Addable".to_string(),
            associated_types: vec!["Output".to_string()],
            methods: vec!["add".to_string()],
            span: Span::dummy(),
            super_traits: vec![],
        });
        self.traits.insert("Hashable".to_string(), TraitSymbol {
            name: "Hashable".to_string(),
            associated_types: vec![],
            methods: vec!["hash".to_string()],
            span: Span::dummy(),
            super_traits: vec!["Comparable".to_string()],
        });
        self.traits.insert("Display".to_string(), TraitSymbol {
            name: "Display".to_string(),
            associated_types: vec![],
            methods: vec!["to_string".to_string()],
            span: Span::dummy(),
            super_traits: vec![],
        });
        self.traits.insert("Iterator".to_string(), TraitSymbol {
            name: "Iterator".to_string(),
            associated_types: vec!["Item".to_string()],
            methods: vec!["next".to_string()],
            span: Span::dummy(),
            super_traits: vec![],
        });
        for t in [Type::Int, Type::Float, Type::Char, Type::String, Type::Bool] {
            self.trait_impls.push(TraitImpl {
                trait_name: "Comparable".to_string(),
                type_params: vec![],
                target_type: t.clone(),
                impl_items: vec!["compare".to_string(), "less".to_string(), "greater".to_string(), "equal".to_string()],
                span: Span::dummy(),
            });
            self.trait_impls.push(TraitImpl {
                trait_name: "Display".to_string(),
                type_params: vec![],
                target_type: t,
                impl_items: vec!["to_string".to_string()],
                span: Span::dummy(),
            });
        }
    }

    pub fn fresh_type_var(&mut self) -> Type {
        let id = self.next_type_var;
        self.next_type_var += 1;
        Type::TypeVar(id)
    }

    pub fn enter_scope(&mut self, is_loop: bool) {
        self.scopes.push(Scope::new(is_loop));
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            if let Some(scope) = self.scopes.pop() {
                self.completed_variables.extend(scope.variables);
            }
        }
    }

    pub fn is_inside_loop(&self) -> bool {
        self.scopes.iter().any(|s| s.is_loop_scope)
    }

    pub fn define_variable(&mut self, sym: VariableSymbol) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.variables.contains_key(&sym.name) {
            return Err(format!("Variable '{}' already declared in this scope", sym.name));
        }
        current_scope.variables.insert(sym.name.clone(), sym);
        Ok(())
    }

    pub fn narrow_variable_type(&mut self, name: &str, narrowed: Type) {
        if self.scopes.iter().rev().any(|scope| scope.variables.contains_key(name)) {
            if let Some(scope) = self.scopes.last_mut() {
                scope.narrowed_vars.insert(name.to_string(), narrowed);
            }
        }
    }

    pub fn reset_narrowing(&mut self) {
        for scope in self.scopes.iter_mut() {
            for (_, var) in scope.variables.iter_mut() {
                var.narrowed_type = None;
            }
            scope.narrowed_vars.clear();
        }
    }

    pub fn lookup_variable(&self) -> &HashMap<String, VariableSymbol> {
        &self.scopes.last().unwrap().variables
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableSymbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.variables.get(name) {
                return Some(sym);
            }
        }
        self.completed_variables.get(name)
    }

    /// Returns true if `name` is declared in the innermost (current) scope.
    /// Used to distinguish deterministic sequential moves from conditional
    /// moves in sibling branches (Owned mode).
    pub fn is_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map_or(false, |s| s.variables.contains_key(name))
    }

    pub fn get_variable_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(narrowed) = scope.narrowed_vars.get(name) {
                return Some(narrowed.clone());
            }
            if let Some(sym) = scope.variables.get(name) {
                return Some(sym.ty.clone());
            }
        }
        None
    }

    pub fn get_variable_mut(&mut self, name: &str) -> Option<&mut VariableSymbol> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(sym) = scope.variables.get_mut(name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn define_function(&mut self, sym: FunctionSymbol) -> Result<(), String> {
        if self.functions.contains_key(&sym.name) {
            return Err(format!("Function '{}' already declared", sym.name));
        }
        self.functions.insert(sym.name.clone(), sym);
        Ok(())
    }

    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSymbol> {
        self.functions.get(name)
    }

    pub fn define_operator(&mut self, sym: OperatorSymbol) -> Result<(), String> {
        if self.operators.contains_key(&sym.operator) {
            return Err(format!("Operator '{}' already declared", sym.operator));
        }
        self.operators.insert(sym.operator.clone(), sym);
        Ok(())
    }

    pub fn lookup_operator(&self, operator: &str) -> Option<&OperatorSymbol> {
        self.operators.get(operator)
    }

    pub fn lookup_trait_impl(&self, trait_name: &str, ty: &Type) -> Option<&TraitImpl> {
        self.trait_impls.iter().find(|imp| {
            imp.trait_name == trait_name && imp.target_type.is_assignable_to(ty)
        })
    }

    pub fn satisfies_trait(&self, ty: &Type, trait_name: &str, sub: &Substitution) -> bool {
        let resolved = sub.apply(ty);
        if let Type::Ref(inner, _) = &resolved {
            return self.satisfies_trait(inner, trait_name, sub);
        }
        if let Type::Generic(name) = &resolved {
            if let Some(bounds) = self.current_generic_env.lookup_bound(name) {
                return bounds.iter().any(|b| b == trait_name || self.has_supertrait_impl(b, trait_name));
            }
            return self.functions.values().any(|function| {
                function.type_params.iter().any(|(param, bounds)| {
                    param == name && bounds.iter().any(|bound| {
                        bound == trait_name || self.has_supertrait_impl(bound, trait_name)
                    })
                })
            });
        }
        self.trait_impls.iter().any(|imp| {
            if imp.trait_name != trait_name {
                return false;
            }
            imp.target_type.is_assignable_to(&resolved)
        })
    }

    pub fn has_supertrait_impl(&self, base: &str, target: &str) -> bool {
        if base == target { return true; }
        if let Some(trait_sym) = self.traits.get(base) {
            trait_sym.super_traits.iter().any(|st| self.has_supertrait_impl(st, target))
        } else {
            false
        }
    }
}
