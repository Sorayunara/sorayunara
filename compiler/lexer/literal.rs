#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
}
