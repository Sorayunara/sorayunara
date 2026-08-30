#![allow(dead_code)]

use std::fmt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    E0308, // Mismatched Types
    E0382, // Use of moved value
    E0425, // Cannot find value / undeclared identifier
    E0502, // Cannot borrow as mutable because it is also borrowed as immutable
    E0506, // Cannot assign / mutate while borrowed
    E0312, // Non-exhaustive pattern match
    E0106, // Missing lifetime specifier
    E0001, // Custom / generic error
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::E0308 => write!(f, "E0308"),
            ErrorCode::E0382 => write!(f, "E0382"),
            ErrorCode::E0425 => write!(f, "E0425"),
            ErrorCode::E0502 => write!(f, "E0502"),
            ErrorCode::E0506 => write!(f, "E0506"),
            ErrorCode::E0312 => write!(f, "E0312"),
            ErrorCode::E0106 => write!(f, "E0106"),
            ErrorCode::E0001 => write!(f, "E0001"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: Option<ErrorCode>,
    pub message: String,
    pub span: Span,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub suggestions: Vec<String>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(level: DiagnosticLevel, message: impl Into<String>, span: Span) -> Self {
        Self {
            level,
            code: None,
            message: message.into(),
            span,
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
            hint: None,
        }
    }

    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self::new(DiagnosticLevel::Error, message, span)
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self::new(DiagnosticLevel::Warning, message, span)
    }

    pub fn note(message: impl Into<String>, span: Span) -> Self {
        Self::new(DiagnosticLevel::Note, message, span)
    }

    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_label(mut self, span: Span, message: impl Into<String>, is_primary: bool) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            is_primary,
        });
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn format_with_source(&self, filename: &str, source: &str) -> String {
        let mut output = String::new();

        // 1. Header: error[E0308]: message
        let level_str = match self.level {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Note => "note",
            DiagnosticLevel::Help => "help",
        };

        if let Some(code) = self.code {
            output.push_str(&format!("{}[{}]: {}\n", level_str, code, self.message));
        } else {
            output.push_str(&format!("{}: {}\n", level_str, self.message));
        }

        // 2. Location pointer: --> filename:line:col
        output.push_str(&format!("  --> {}:{}:{}\n", filename, self.span.line, self.span.col));
        output.push_str("   |\n");

        let lines: Vec<&str> = source.lines().collect();
        if self.span.line > 0 && self.span.line <= lines.len() {
            let line_content = lines[self.span.line - 1];
            output.push_str(&format!("{:4} | {}\n", self.span.line, line_content));

            let spaces = " ".repeat(self.span.col.saturating_sub(1));
            let underline_len = (self.span.end.saturating_sub(self.span.start)).max(1);
            let carets = "^".repeat(underline_len);
            output.push_str(&format!("   | {}{}", spaces, carets));

            if !self.labels.is_empty() {
                let label_text = &self.labels[0].message;
                output.push_str(&format!(" {}\n", label_text));
            } else {
                output.push('\n');
            }
        }

        output.push_str("   |\n");

        // 3. Notes
        for note in &self.notes {
            output.push_str(&format!("   = note: {}\n", note));
        }

        // 4. Suggestions & Help
        for sugg in &self.suggestions {
            output.push_str(&format!("   = help: {}\n", sugg));
        }

        if let Some(ref h) = self.hint {
            output.push_str(&format!("   = hint: {}\n", h));
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
