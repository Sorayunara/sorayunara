#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Mut,
    Const,
    Return,
    Print,
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Struct,
    Enum,
    Type,
    Trait,
    Impl,
    Operator,
    Match,
    Is,
    Import,
    Mod,
    True,
    False,
    Null,
    Some,
    None,
    Ok,
    Err,

    // Memory Model Keywords
    Move,
    Unsafe,

    // Concurrency Keywords
    Async,
    Await,
    Task,
    Spawn,
    Chan,
    Scope,
    Comptime,

    // FFI & Testing Keywords
    Extern,
    Test,
    Assert,

    // Identifiers & Literals
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    CharLit(char),

    // Symbols & Delimiters
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp,
    PipePipe,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Arrow,       // ->
    FatArrow,    // =>
    ColonEqual,  // :=
    Pipeline,    // |>
    Question,    // ?
    Dot,
    Comma,
    Colon,
    DoubleColon, // ::
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Eof,
}
