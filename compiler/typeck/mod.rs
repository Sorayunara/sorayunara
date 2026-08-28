#![allow(dead_code)]

use std::collections::HashMap;
use crate::ast::TypeNode;
use crate::diagnostics::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum InferredType {
    Int,
    Float,
    Bool,
    String,
    Char,
    Tuple(Vec<InferredType>),
    Array(Box<InferredType>),
    Option(Box<InferredType>),
    Result(Box<InferredType>, Box<InferredType>),
    Function { params: Vec<InferredType>, ret: Box<InferredType> },
    Var(usize),
    Void,
}

pub struct TypeEnv {
    types: HashMap<String, InferredType>,
    type_var_counter: usize,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            type_var_counter: 0,
        }
    }

    pub fn fresh_type_var(&mut self) -> InferredType {
        let id = self.type_var_counter;
        self.type_var_counter += 1;
        InferredType::Var(id)
    }

    pub fn insert(&mut self, name: String, ty: InferredType) {
        self.types.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&InferredType> {
        self.types.get(name)
    }

    pub fn unify(&self, a: &InferredType, b: &InferredType) -> Result<InferredType, String> {
        match (a, b) {
            (InferredType::Int, InferredType::Int) => Ok(InferredType::Int),
            (InferredType::Float, InferredType::Float) => Ok(InferredType::Float),
            (InferredType::Bool, InferredType::Bool) => Ok(InferredType::Bool),
            (InferredType::String, InferredType::String) => Ok(InferredType::String),
            (InferredType::Char, InferredType::Char) => Ok(InferredType::Char),
            (InferredType::Void, InferredType::Void) => Ok(InferredType::Void),
            (InferredType::Var(_), other) => Ok(other.clone()),
            (other, InferredType::Var(_)) => Ok(other.clone()),
            (InferredType::Array(t1), InferredType::Array(t2)) => {
                let inner = self.unify(t1, t2)?;
                Ok(InferredType::Array(Box::new(inner)))
            }
            (InferredType::Option(t1), InferredType::Option(t2)) => {
                let inner = self.unify(t1, t2)?;
                Ok(InferredType::Option(Box::new(inner)))
            }
            _ => Err(format!("Cannot unify type {:?} with {:?}", a, b)),
        }
    }
}
