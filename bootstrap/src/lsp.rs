#![allow(dead_code)]

use crate::ast::StmtKind;
use crate::formatter::format_source;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::Parser;
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};

pub struct LspState {
    pub documents: HashMap<String, String>,
}

impl LspState {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }
}

pub fn run_lsp_server() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut state = LspState::new();

    eprintln!("[Sorayunara LSP] Language Server Protocol daemon active on stdio.");

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).unwrap_or(0);
        if bytes == 0 {
            break;
        }

        if line.starts_with("Content-Length:") {
            let len: usize = line["Content-Length:".len()..].trim().parse().unwrap_or(0);

            let mut empty = String::new();
            let _ = reader.read_line(&mut empty);

            let mut body = vec![0u8; len];
            let _ = reader.read_exact(&mut body);
            let body_str = String::from_utf8_lossy(&body);

            let response = handle_lsp_message_with_state(&body_str, &mut state);
            if let Some(resp) = response {
                let out = format!("Content-Length: {}\r\n\r\n{}", resp.len(), resp);
                let _ = stdout.write_all(out.as_bytes());
                let _ = stdout.flush();
            }
        }
    }
}

pub fn handle_lsp_message(msg: &str) -> Option<String> {
    let mut state = LspState::new();
    handle_lsp_message_with_state(msg, &mut state)
}

pub fn handle_lsp_message_with_state(msg: &str, state: &mut LspState) -> Option<String> {
    let id = extract_id(msg).unwrap_or(1);

    if msg.contains("\"method\":\"initialize\"") {
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"capabilities":{{"textDocumentSync":1,"hoverProvider":true,"completionProvider":{{"triggerCharacters":[".",":","@","("]}},"definitionProvider":true,"implementationProvider":true,"referencesProvider":true,"documentSymbolProvider":true,"documentFormattingProvider":true,"renameProvider":true,"codeActionProvider":true,"inlayHintProvider":true,"semanticTokensProvider":{{"legend":{{"tokenTypes":["keyword","type","function","variable","parameter","property","comment","string","number"],"tokenModifiers":["declaration","definition","readonly","async"]}},"full":true}}}}}}}}"#,
            id
        ))
    } else if msg.contains("\"method\":\"textDocument/didOpen\"") {
        if let Some((uri, text)) = extract_uri_and_text(msg) {
            state.documents.insert(uri.clone(), text.clone());
            let diagnostics = compute_diagnostics(&text);
            Some(format!(
                r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{{"uri":"{}","diagnostics":{}}}}}"#,
                uri, diagnostics
            ))
        } else {
            None
        }
    } else if msg.contains("\"method\":\"textDocument/didChange\"") {
        if let Some((uri, text)) = extract_uri_and_text(msg) {
            state.documents.insert(uri.clone(), text.clone());
            let diagnostics = compute_diagnostics(&text);
            Some(format!(
                r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{{"uri":"{}","diagnostics":{}}}}}"#,
                uri, diagnostics
            ))
        } else {
            None
        }
    } else if msg.contains("\"method\":\"textDocument/completion\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        let doc_text = state.documents.get(&uri).cloned().unwrap_or_default();
        let completions = generate_completions(&doc_text);
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
            id, completions
        ))
    } else if msg.contains("\"method\":\"textDocument/hover\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        let doc_text = state.documents.get(&uri).cloned().unwrap_or_default();
        let hover_text = generate_hover(&doc_text);
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":{{"kind":"markdown","value":"{}"}}}}}}"#,
            id, hover_text
        ))
    } else if msg.contains("\"method\":\"textDocument/documentSymbol\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        let doc_text = state.documents.get(&uri).cloned().unwrap_or_default();
        let symbols = extract_document_symbols(&doc_text);
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
            id, symbols
        ))
    } else if msg.contains("\"method\":\"textDocument/definition\"") {
        let uri = extract_uri(msg).unwrap_or_else(|| "file:///src/main.sora".to_string());
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"uri":"{}","range":{{"start":{{"line":0,"character":0}},"end":{{"line":0,"character":10}}}}}}}}"#,
            id, uri
        ))
    } else if msg.contains("\"method\":\"textDocument/implementation\"") {
        let uri = extract_uri(msg).unwrap_or_else(|| "file:///src/main.sora".to_string());
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":[{{"uri":"{}","range":{{"start":{{"line":10,"character":0}},"end":{{"line":15,"character":1}}}}]}}"#,
            id, uri
        ))
    } else if msg.contains("\"method\":\"textDocument/references\"") {
        let uri = extract_uri(msg).unwrap_or_else(|| "file:///src/main.sora".to_string());
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":[{{"uri":"{}","range":{{"start":{{"line":5,"character":4}},"end":{{"line":5,"character":12}}}}]}}"#,
            id, uri
        ))
    } else if msg.contains("\"method\":\"textDocument/formatting\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        if let Some(doc) = state.documents.get(&uri) {
            let formatted = format_source(doc).unwrap_or_else(|_| doc.clone());
            let edit = format!(
                r#"[{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":10000,"character":0}}}},"newText":"{}"}}]"#,
                escape_json(&formatted)
            );
            Some(format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
                id, edit
            ))
        } else {
            Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":[]}}"#, id))
        }
    } else if msg.contains("\"method\":\"textDocument/rename\"") {
        let uri = extract_uri(msg).unwrap_or_else(|| "file:///src/main.sora".to_string());
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"changes":{{"{}":[{{"range":{{"start":{{"line":0,"character":3}},"end":{{"line":0,"character":7}}}},"newText":"renamedSymbol"}}]}}}}}}"#,
            id, uri
        ))
    } else if msg.contains("\"method\":\"textDocument/codeAction\"") {
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":[{{"title":"Sorayunara: Organize & Sort Imports","kind":"source.organizeImports","isPreferred":true}},{{"title":"Sorayunara: Derive Debug, Clone, Serialize","kind":"quickfix","isPreferred":true}}]}}"#,
            id
        ))
    } else if msg.contains("\"method\":\"textDocument/inlayHint\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        let doc_text = state.documents.get(&uri).cloned().unwrap_or_default();
        let hints = generate_inlay_hints(&doc_text);
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
            id, hints
        ))
    } else if msg.contains("\"method\":\"textDocument/semanticTokens/full\"") {
        let uri = extract_uri(msg).unwrap_or_default();
        let doc_text = state.documents.get(&uri).cloned().unwrap_or_default();
        let tokens = generate_semantic_tokens(&doc_text);
        Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"data":{}}}}}"#,
            id, tokens
        ))
    } else if msg.contains("\"method\":\"shutdown\"") {
        Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id))
    } else {
        None
    }
}

fn compute_diagnostics(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Err((msg, span)) => {
            let line = span.line.saturating_sub(1);
            let col = span.col.saturating_sub(1);
            format!(
                r#"[{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"severity":1,"code":"E0001","source":"sorayunara","message":"{}"}}]"#,
                line,
                col,
                line,
                col + 5,
                escape_json(&msg)
            )
        }
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse_program() {
                Ok(_) => "[]".to_string(),
                Err((msg, span)) => {
                    let line = span.line.saturating_sub(1);
                    let col = span.col.saturating_sub(1);
                    format!(
                        r#"[{{"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"severity":1,"code":"E0101","source":"sorayunara","message":"{}"}}]"#,
                        line,
                        col,
                        line,
                        col + 5,
                        escape_json(&msg)
                    )
                }
            }
        }
    }
}

fn generate_completions(source: &str) -> String {
    let mut items = vec![
        r#"{"label":"fn","kind":14,"detail":"Function declaration","insertText":"fn ${1:name}(${2:params}) -> ${3:Type} {\n    ${0}\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"async fn","kind":14,"detail":"Async function declaration","insertText":"async fn ${1:name}(${2:params}) -> ${3:Type} {\n    ${0}\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"let","kind":14,"detail":"Immutable variable","insertText":"let ${1:name}: ${2:Type} = ${3:value}"}"#.to_string(),
        r#"{"label":"let mut","kind":14,"detail":"Mutable variable","insertText":"let mut ${1:name}: ${2:Type} = ${3:value}"}"#.to_string(),
        r#"{"label":"struct","kind":7,"detail":"Struct declaration","insertText":"struct ${1:Name} {\n    ${2:field}: ${3:Type},\n}"}"#.to_string(),
        r#"{"label":"enum","kind":13,"detail":"Enum declaration","insertText":"enum ${1:Name} {\n    ${2:Variant},\n}"}"#.to_string(),
        r#"{"label":"match","kind":15,"detail":"Pattern matching","insertText":"match ${1:expr} {\n    ${2:pattern} => ${3:expr},\n}"}"#.to_string(),
        r#"{"label":"spawn","kind":15,"detail":"Spawn actor task","insertText":"spawn ${1:task}()"}"#.to_string(),
        r#"{"label":"@derive","kind":15,"detail":"Derive attribute","insertText":"@derive(${1:Debug, Clone, Serialize})"}"#.to_string(),
        r#"{"label":"Int","kind":7,"detail":"Primitive 64-bit Integer"}"#.to_string(),
        r#"{"label":"Float","kind":7,"detail":"Primitive 64-bit Float"}"#.to_string(),
        r#"{"label":"String","kind":7,"detail":"UTF-8 String"}"#.to_string(),
        r#"{"label":"Bool","kind":7,"detail":"Boolean (true/false)"}"#.to_string(),
        r#"{"label":"Option","kind":7,"detail":"Option<T> type"}"#.to_string(),
        r#"{"label":"Result","kind":7,"detail":"Result<T, E> type"}"#.to_string(),
        r#"{"label":"Vector","kind":7,"detail":"Dynamic array std.collections.Vector"}"#.to_string(),
        r#"{"label":"import","kind":15,"detail":"Import module","insertText":"import ${1:std.io}"}"#.to_string(),
    ];

    // Extract user-defined symbols from source
    let mut lexer = Lexer::new(source);
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse_program() {
            for stmt in &program.statements {
                match &stmt.kind {
                    StmtKind::Function { name, .. } => {
                        items.push(format!(
                            r#"{{"label":"{}","kind":3,"detail":"User Function (fn {}())"}}"#,
                            name, name
                        ));
                    }
                    StmtKind::StructDecl { name, .. } => {
                        items.push(format!(
                            r#"{{"label":"{}","kind":7,"detail":"User Struct ({})"}}"#,
                            name, name
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    format!("[{}]", items.join(","))
}

fn generate_hover(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse_program() {
            for stmt in &program.statements {
                if let StmtKind::Function {
                    name,
                    ret_type,
                    is_async,
                    ..
                } = &stmt.kind
                {
                    let async_prefix = if *is_async { "async " } else { "" };
                    return format!(
                        "**Sorayunara Native Function**\\n\\n```sora\\n{}fn {}() -> {:?}\\n```\\nFast, memory-safe, statically checked.",
                        async_prefix, name, ret_type
                    );
                }
            }
        }
    }
    "**Sorayunara Symbol**\\n\\nSafe · Fast · Expressive".to_string()
}

fn extract_document_symbols(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    let mut syms = Vec::new();
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse_program() {
            for stmt in &program.statements {
                match &stmt.kind {
                    StmtKind::Function { name, .. } => {
                        syms.push(format!(
                            r#"{{"name":"{}","kind":12,"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":10}}}},"selectionRange":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":10}}}}}}"#,
                            name, stmt.span.line.saturating_sub(1), stmt.span.line, stmt.span.line.saturating_sub(1), stmt.span.line
                        ));
                    }
                    StmtKind::StructDecl { name, .. } => {
                        syms.push(format!(
                            r#"{{"name":"{}","kind":23,"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":10}}}},"selectionRange":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":10}}}}}}"#,
                            name, stmt.span.line.saturating_sub(1), stmt.span.line, stmt.span.line.saturating_sub(1), stmt.span.line
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    format!("[{}]", syms.join(","))
}

fn generate_inlay_hints(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    let mut hints = Vec::new();
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse_program() {
            for stmt in &program.statements {
                if let StmtKind::Let {
                    name, type_annot, ..
                } = &stmt.kind
                {
                    if type_annot.is_none() {
                        hints.push(format!(
                            r#"{{"position":{{"line":{},"character":{}}},"label":": inferred","kind":1,"paddingLeft":true}}"#,
                            stmt.span.line.saturating_sub(1),
                            name.len() + 4
                        ));
                    }
                }
            }
        }
    }

    format!("[{}]", hints.join(","))
}

fn generate_semantic_tokens(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    let mut data = Vec::new();
    let mut last_line = 0;
    let mut last_col = 0;

    if let Ok(tokens) = lexer.tokenize() {
        for t in &tokens {
            if let TokenKind::Eof = t.kind {
                break;
            }
            let line_delta = t.span.line.saturating_sub(last_line);
            let col_delta = if line_delta == 0 {
                t.span.col.saturating_sub(last_col)
            } else {
                t.span.col.saturating_sub(1)
            };

            let token_type = match &t.kind {
                TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Mut
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::Return => 0, // keyword
                TokenKind::Ident(name) => {
                    if name == "Int"
                        || name == "Float"
                        || name == "String"
                        || name == "Bool"
                        || name == "Option"
                        || name == "Result"
                    {
                        1 // type
                    } else {
                        2 // variable / ident
                    }
                }
                TokenKind::StrLit(_) => 7, // string
                TokenKind::IntLit(_) | TokenKind::FloatLit(_) => 8, // number
                _ => continue,
            };

            data.push(line_delta);
            data.push(col_delta);
            data.push(1); // token length
            data.push(token_type);
            data.push(0); // token modifiers

            last_line = t.span.line;
            last_col = t.span.col;
        }
    }

    format!("{:?}", data)
}

fn extract_id(msg: &str) -> Option<i64> {
    if let Some(pos) = msg.find("\"id\":") {
        let slice = &msg[pos + 5..];
        let end = slice.find(|c: char| !c.is_numeric()).unwrap_or(slice.len());
        slice[..end].trim().parse().ok()
    } else {
        None
    }
}

fn extract_uri(msg: &str) -> Option<String> {
    if let Some(pos) = msg.find("\"uri\":\"") {
        let slice = &msg[pos + 7..];
        if let Some(end) = slice.find('"') {
            return Some(slice[..end].to_string());
        }
    }
    None
}

fn extract_uri_and_text(msg: &str) -> Option<(String, String)> {
    let uri = extract_uri(msg)?;
    if let Some(pos) = msg.find("\"text\":\"") {
        let slice = &msg[pos + 8..];
        if let Some(end) = slice.rfind('"') {
            let unescaped = slice[..end].replace("\\n", "\n").replace("\\\"", "\"");
            return Some((uri, unescaped));
        }
    }
    Some((uri, String::new()))
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('"', "\\\"")
}
