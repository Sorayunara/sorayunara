#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TypeNode {
    Int,
    Float,
    Bool,
    String,
    Char,
    Tuple(Vec<TypeNode>),
    Array(Box<TypeNode>),
    Slice(Box<TypeNode>),
    Map(Box<TypeNode>, Box<TypeNode>),
    Set(Box<TypeNode>),
    Union(Vec<TypeNode>),
    Option(Box<TypeNode>),
    Result(Box<TypeNode>, Box<TypeNode>),
    Function {
        params: Vec<TypeNode>,
        ret: Box<TypeNode>,
    },
    Generic {
        name: String,
        args: Vec<TypeNode>,
    },
    Ref(Box<TypeNode>, bool), // &T or &mut T
    Ptr(Box<TypeNode>, bool), // *const T / *mut T
    Task(Box<TypeNode>),      // Task<T>
    Chan(Box<TypeNode>),      // Chan<T>
    Custom(String),
    Void,
    Infer,
}
