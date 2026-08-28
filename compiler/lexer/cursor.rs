#![allow(dead_code)]

pub struct Cursor<'a> {
    pub chars: std::str::Chars<'a>,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }
}
