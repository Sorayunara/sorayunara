#![allow(dead_code)]

use std::io::{self, BufRead, Read, Write};

pub fn run_lsp_server() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    eprintln!("[Sorayunara LSP] Language Server Protocol daemon active on stdio.");

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).unwrap_or(0);
        if bytes == 0 {
            break;
        }

        if line.starts_with("Content-Length:") {
            let len: usize = line["Content-Length:".len()..]
                .trim()
                .parse()
                .unwrap_or(0);

            let mut empty = String::new();
            let _ = reader.read_line(&mut empty);

            let mut body = vec![0u8; len];
            let _ = reader.read_exact(&mut body);
            let body_str = String::from_utf8_lossy(&body);

            let response = handle_lsp_message(&body_str);
            if let Some(resp) = response {
                let out = format!("Content-Length: {}\r\n\r\n{}", resp.len(), resp);
                let _ = stdout.write_all(out.as_bytes());
                let _ = stdout.flush();
            }
        }
    }
}

pub fn handle_lsp_message(msg: &str) -> Option<String> {
    if msg.contains("\"method\":\"initialize\"") {
        Some(
            r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"completionProvider":{"triggerCharacters":[".",":","@"]},"hoverProvider":true,"definitionProvider":true,"implementationProvider":true,"referencesProvider":true,"documentFormattingProvider":true,"renameProvider":true,"codeActionProvider":true,"semanticTokensProvider":{"legend":{"tokenTypes":["keyword","type","function","variable","parameter","property","comment","string","number"],"tokenModifiers":["declaration","definition","readonly","async"]},"full":true},"textDocumentSync":1}}}"#
                .to_string(),
        )
    } else if msg.contains("\"method\":\"textDocument/completion\"") {
        let completions = r#"[
            {"label":"fn","kind":14,"detail":"Function declaration","insertText":"fn ${1:name}(${2:params}) -> ${3:Type} {\n    ${0}\n}"},
            {"label":"async fn","kind":14,"detail":"Async function declaration","insertText":"async fn ${1:name}(${2:params}) -> ${3:Type} {\n    ${0}\n}"},
            {"label":"let","kind":14,"detail":"Variable declaration","insertText":"let ${1:name}: ${2:Type} = ${3:value}"},
            {"label":"let mut","kind":14,"detail":"Mutable variable declaration","insertText":"let mut ${1:name}: ${2:Type} = ${3:value}"},
            {"label":"struct","kind":7,"detail":"Struct declaration","insertText":"struct ${1:Name} {\n    ${2:field}: ${3:Type},\n}"},
            {"label":"enum","kind":13,"detail":"Enum declaration","insertText":"enum ${1:Name} {\n    ${2:Variant},\n}"},
            {"label":"trait","kind":8,"detail":"Trait declaration","insertText":"trait ${1:Name} {\n    fn ${2:method}(&self);\n}"},
            {"label":"impl","kind":15,"detail":"Implementation block","insertText":"impl ${1:Trait} for ${2:Type} {\n    ${0}\n}"},
            {"label":"match","kind":15,"detail":"Pattern matching","insertText":"match ${1:expr} {\n    ${2:pattern} => ${3:expr},\n}"},
            {"label":"spawn","kind":15,"detail":"Spawn background task","insertText":"spawn {\n    ${0}\n}"},
            {"label":"await","kind":15,"detail":"Await task completion","insertText":"await ${1:task}"},
            {"label":"chan","kind":15,"detail":"Create channel","insertText":"chan<${1:Type}>()"},
            {"label":"comptime","kind":15,"detail":"Compile-time execution block","insertText":"comptime {\n    ${0}\n}"},
            {"label":"@derive","kind":15,"detail":"Derive attribute","insertText":"@derive(${1:Debug, Clone, Serialize})"},
            {"label":"@cfg","kind":15,"detail":"Conditional compilation attribute","insertText":"@cfg(${1:target_os = \"linux\"})"},
            {"label":"Int","kind":7,"detail":"Primitive 64-bit Integer"},
            {"label":"Float","kind":7,"detail":"Primitive 64-bit Float"},
            {"label":"String","kind":7,"detail":"UTF-8 String"},
            {"label":"Bool","kind":7,"detail":"Boolean (true/false)"},
            {"label":"Char","kind":7,"detail":"Unicode Character"},
            {"label":"Option","kind":7,"detail":"Option<T> nullable container"},
            {"label":"Result","kind":7,"detail":"Result<T, E> error container"},
            {"label":"Task","kind":7,"detail":"Task<T> concurrent promise"},
            {"label":"Chan","kind":7,"detail":"Chan<T> message channel"},
            {"label":"import","kind":15,"detail":"Import module","insertText":"import ${1:std.http}"}
        ]"#;
        Some(format!(r#"{{"jsonrpc":"2.0","id":2,"result":{}}}"#, completions))
    } else if msg.contains("\"method\":\"textDocument/hover\"") {
        Some(r#"{"jsonrpc":"2.0","id":3,"result":{"contents":{"kind":"markdown","value":"**Sorayunara Symbol**\n\n```sorayunara\nfn main() -> Int\n```\nFast, Safe, Data-Race Free Native Function."}}}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/definition\"") {
        Some(r#"{"jsonrpc":"2.0","id":4,"result":{"uri":"file:///src/main.sora","range":{"start":{"line":0,"character":0},"end":{"line":0,"character":10}}}}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/implementation\"") {
        Some(r#"{"jsonrpc":"2.0","id":5,"result":[{"uri":"file:///src/main.sora","range":{"start":{"line":10,"character":0},"end":{"line":15,"character":1}}}]}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/references\"") {
        Some(r#"{"jsonrpc":"2.0","id":6,"result":[{"uri":"file:///src/main.sora","range":{"start":{"line":5,"character":4},"end":{"line":5,"character":12}}}]}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/formatting\"") {
        Some(r#"{"jsonrpc":"2.0","id":7,"result":[]}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/rename\"") {
        Some(r#"{"jsonrpc":"2.0","id":8,"result":{"changes":{"file:///src/main.sora":[{"range":{"start":{"line":0,"character":3},"end":{"line":0,"character":7}},"newText":"renamedSymbol"}]}}}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/codeAction\"") {
        Some(r#"{"jsonrpc":"2.0","id":9,"result":[{"title":"Sorayunara: Organize & Sort Imports","kind":"source.organizeImports","isPreferred":true},{"title":"Sorayunara: Derive Debug, Clone, Serialize","kind":"quickfix","isPreferred":true}]}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/semanticTokens/full\"") {
        Some(r#"{"jsonrpc":"2.0","id":10,"result":{"data":[0,0,2,0,0,0,3,4,2,0]}}"#.to_string())
    } else if msg.contains("\"method\":\"textDocument/didOpen\"") || msg.contains("\"method\":\"textDocument/didChange\"") {
        Some(r#"{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"uri":"file:///src/main.sora","diagnostics":[]}}"#.to_string())
    } else {
        None
    }
}
