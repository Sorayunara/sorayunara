#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Self {
        Self { start, end, line, col }
    }

    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 1, col: 1 }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            col: if self.line <= other.line { self.col } else { other.col },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn format_with_source(&self, filename: &str, source: &str) -> String {
        let prefix = match self.level {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Note => "note",
        };

        let mut output = format!("{}: {} at {}:{}:{}\n", prefix, self.message, filename, self.span.line, self.span.col);

        let lines: Vec<&str> = source.lines().collect();
        if self.span.line > 0 && self.span.line <= lines.len() {
            let line_content = lines[self.span.line - 1];
            output.push_str(&format!("{:4} | {}\n", self.span.line, line_content));
            
            let spaces = " ".repeat(self.span.col.saturating_sub(1));
            let underline_len = (self.span.end.saturating_sub(self.span.start)).max(1);
            let carets = "^".repeat(underline_len);
            output.push_str(&format!("     | {}{}\n", spaces, carets));
        }

        if let Some(ref h) = self.hint {
            output.push_str(&format!("     = hint: {}\n", h));
        }

        output
    }
}

pub struct DiagnosticEngine {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }

    pub fn emit(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error)
    }

    pub fn render_all(&self, filename: &str, source: &str) -> String {
        let mut res = String::new();
        for diag in &self.diagnostics {
            res.push_str(&diag.format_with_source(filename, source));
            res.push('\n');
        }
        res
    }
}
