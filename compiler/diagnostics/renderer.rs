#![allow(dead_code)]

use super::diagnostic::Diagnostic;
use super::severity::Severity;

pub struct DiagnosticRenderer;

impl DiagnosticRenderer {
    pub fn render(diag: &Diagnostic, filename: &str, source: &str) -> String {
        let prefix = diag.severity.as_str();
        let mut output = format!("{}: {} at {}:{}:{}\n", prefix, diag.message, filename, diag.span.line, diag.span.col);

        let lines: Vec<&str> = source.lines().collect();
        if diag.span.line > 0 && diag.span.line <= lines.len() {
            let line_content = lines[diag.span.line - 1];
            output.push_str(&format!("{:4} | {}\n", diag.span.line, line_content));

            let spaces = " ".repeat(diag.span.col.saturating_sub(1));
            let underline_len = (diag.span.end.saturating_sub(diag.span.start)).max(1);
            let carets = "^".repeat(underline_len);
            output.push_str(&format!("     | {}{}\n", spaces, carets));
        }

        if let Some(ref h) = diag.hint {
            output.push_str(&format!("     = hint: {}\n", h));
        }

        output
    }
}
