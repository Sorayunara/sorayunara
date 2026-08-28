#![allow(dead_code)]

use std::collections::HashMap;
use crate::ast::*;
use crate::diagnostics::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable { is_mut: bool, type_annot: Option<TypeNode> },
    Function { params: Vec<(String, TypeNode)>, ret_type: TypeNode },
    Struct { fields: Vec<(String, TypeNode)> },
    Enum { variants: Vec<String> },
    Trait,
    TypeAlias,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>,
}

pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub current_scope: usize,
}

impl Resolver {
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            parent: None,
        };
        Self {
            scopes: vec![global_scope],
            current_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
        };
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.scopes[self.current_scope].symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut idx = Some(self.current_scope);
        while let Some(i) = idx {
            if let Some(sym) = self.scopes[i].symbols.get(name) {
                return Some(sym);
            }
            idx = self.scopes[i].parent;
        }
        None
    }
}
