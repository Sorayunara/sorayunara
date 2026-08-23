#![allow(dead_code)]

use crate::diagnostic::Span;

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
    Scope, // structured concurrency: joins all spawned tasks before exit
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

    // Arithmetic Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Assignment & Compound
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Comparison Operators
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical & Reference Operators
    Amp,        // & (Borrow)
    AmpAmp,     // &&
    Pipe,       // |
    PipePipe,   // ||
    Bang,       // !
    CustomOperator(String),

    // Delimiters & Symbols
    Arrow,      // ->
    FatArrow,   // =>
    Colon,      // :
    ColonColon, // ::
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    At,         // @

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_custom_operator_char(ch: char) -> bool {
        matches!(
            ch,
            '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' | '&' | '|' | '^' | '~' | '?'
        )
    }

    fn read_custom_operator(&mut self, first: char) -> TokenKind {
        let mut op = String::new();
        op.push(first);
        while let Some(next) = self.peek() {
            if Self::is_custom_operator_char(next) {
                op.push(next);
                self.advance();
            } else {
                break;
            }
        }
        TokenKind::CustomOperator(op)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == Some('/') {
                        self.advance();
                        self.advance();
                        while let Some(c) = self.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else if self.peek_next() == Some('*') {
                        self.advance();
                        self.advance();
                        while let Some(c) = self.peek() {
                            if c == '*' && self.peek_next() == Some('/') {
                                self.advance();
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, (String, Span)> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            let start_pos = self.pos;
            let start_line = self.line;
            let start_col = self.col;

            let ch = match self.advance() {
                Some(c) => c,
                None => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::Eof,
                        span: Span::new(start_pos, start_pos, start_line, start_col),
                    });
                    break;
                }
            };

            let kind = match ch {
                '+' => {
                    if self.match_char('=') {
                        TokenKind::PlusEqual
                    } else {
                        TokenKind::Plus
                    }
                }
                '-' => {
                    if self.match_char('>') {
                        TokenKind::Arrow
                    } else if self.match_char('=') {
                        TokenKind::MinusEqual
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => {
                    if self.match_char('=') {
                        TokenKind::StarEqual
                    } else {
                        TokenKind::Star
                    }
                }
                '/' => {
                    if self.match_char('=') {
                        TokenKind::SlashEqual
                    } else {
                        TokenKind::Slash
                    }
                }
                '%' => TokenKind::Percent,
                '=' => {
                    if self.match_char('=') {
                        TokenKind::EqualEqual
                    } else if self.match_char('>') {
                        TokenKind::FatArrow
                    } else {
                        TokenKind::Equal
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        TokenKind::LessEqual
                    } else if self
                        .peek()
                        .map_or(false, |next| Self::is_custom_operator_char(next))
                    {
                        self.read_custom_operator('<')
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        TokenKind::GreaterEqual
                    } else if self
                        .peek()
                        .map_or(false, |next| Self::is_custom_operator_char(next))
                    {
                        self.read_custom_operator('>')
                    } else {
                        TokenKind::Greater
                    }
                }
                '&' => {
                    if self.match_char('&') {
                        TokenKind::AmpAmp
                    } else {
                        TokenKind::Amp
                    }
                }
                '|' => {
                    if self.match_char('|') {
                        TokenKind::PipePipe
                    } else if self
                        .peek()
                        .map_or(false, |next| Self::is_custom_operator_char(next))
                    {
                        self.read_custom_operator('|')
                    } else {
                        TokenKind::Pipe
                    }
                }
                '^' | '~' | '?' => self.read_custom_operator(ch),
                ':' => {
                    if self.match_char(':') {
                        TokenKind::ColonColon
                    } else {
                        TokenKind::Colon
                    }
                }
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,
                '@' => TokenKind::At,
                '\'' => {
                    let c = match self.advance() {
                        Some('\\') => match self.advance() {
                            Some('n') => '\n',
                            Some('t') => '\t',
                            Some('r') => '\r',
                            Some('\\') => '\\',
                            Some('\'') => '\'',
                            Some(other) => other,
                            None => return Err(("Unterminated char literal".into(), Span::new(start_pos, self.pos, start_line, start_col))),
                        },
                        Some(ch) => ch,
                        None => return Err(("Unterminated char literal".into(), Span::new(start_pos, self.pos, start_line, start_col))),
                    };
                    if !self.match_char('\'') {
                        return Err(("Expected closing single quote for char literal".into(), Span::new(start_pos, self.pos, start_line, start_col)));
                    }
                    TokenKind::CharLit(c)
                }
                '"' => {
                    let mut s = String::new();
                    let mut closed = false;
                    while let Some(c) = self.advance() {
                        if c == '"' {
                            closed = true;
                            break;
                        } else if c == '\\' {
                            match self.advance() {
                                Some('n') => s.push('\n'),
                                Some('t') => s.push('\t'),
                                Some('r') => s.push('\r'),
                                Some('\\') => s.push('\\'),
                                Some('"') => s.push('"'),
                                Some(other) => {
                                    s.push('\\');
                                    s.push(other);
                                }
                                None => break,
                            }
                        } else {
                            s.push(c);
                        }
                    }
                    if !closed {
                        let span = Span::new(start_pos, self.pos, start_line, start_col);
                        return Err(("Unterminated string literal".to_string(), span));
                    }
                    TokenKind::StrLit(s)
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    num_str.push(ch);
                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            if d != '_' {
                                num_str.push(d);
                            }
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
                        num_str.push('.');
                        self.advance();
                        while let Some(d) = self.peek() {
                            if d.is_ascii_digit() || d == '_' {
                                if d != '_' {
                                    num_str.push(d);
                                }
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let val: f64 = num_str.parse().map_err(|e| {
                            (format!("Invalid float literal: {}", e), Span::new(start_pos, self.pos, start_line, start_col))
                        })?;
                        TokenKind::FloatLit(val)
                    } else {
                        let val: i64 = num_str.parse().map_err(|e| {
                            (format!("Invalid integer literal: {}", e), Span::new(start_pos, self.pos, start_line, start_col))
                        })?;
                        TokenKind::IntLit(val)
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    ident.push(ch);
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    match ident.as_str() {
                        "fn" => TokenKind::Fn,
                        "import" => TokenKind::Import,
                        "mod" => TokenKind::Mod,
                        "async" => TokenKind::Async,
                        "await" => TokenKind::Await,
                        "task" => TokenKind::Task,
                        "spawn" => TokenKind::Spawn,
                        "chan" => TokenKind::Chan,
                        "scope" => TokenKind::Scope,
                        "comptime" => TokenKind::Comptime,
                        "extern" => TokenKind::Extern,
                        "test" => TokenKind::Test,
                        "assert" => TokenKind::Assert,
                        "move" => TokenKind::Move,
                        "unsafe" => TokenKind::Unsafe,
                        "let" => TokenKind::Let,
                        "mut" => TokenKind::Mut,
                        "const" => TokenKind::Const,
                        "return" => TokenKind::Return,
                        "print" => TokenKind::Print,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "while" => TokenKind::While,
                        "for" => TokenKind::For,
                        "in" => TokenKind::In,
                        "loop" => TokenKind::Loop,
                        "break" => TokenKind::Break,
                        "continue" => TokenKind::Continue,
                        "struct" => TokenKind::Struct,
                        "enum" => TokenKind::Enum,
                        "type" => TokenKind::Type,
                        "trait" => TokenKind::Trait,
                        "impl" => TokenKind::Impl,
                        "operator" => TokenKind::Operator,
                        "match" => TokenKind::Match,
                        "is" => TokenKind::Is,
                        "true" => TokenKind::True,
                        "false" => TokenKind::False,
                        "null" => TokenKind::Null,
                        "Some" => TokenKind::Some,
                        "None" => TokenKind::None,
                        "Ok" => TokenKind::Ok,
                        "Err" => TokenKind::Err,
                        _ => TokenKind::Ident(ident),
                    }
                }
                _ => {
                    let span = Span::new(start_pos, self.pos, start_line, start_col);
                    return Err((format!("Unexpected character: '{}'", ch), span));
                }
            };

            tokens.push(SpannedToken {
                kind,
                span: Span::new(start_pos, self.pos, start_line, start_col),
            });
        }

        Ok(tokens)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<SpannedToken>, (String, Span)> {
    Lexer::new(source).tokenize()
}