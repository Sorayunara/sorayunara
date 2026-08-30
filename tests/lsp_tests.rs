use sorayunara::lsp::handle_lsp_message;

#[test]
fn test_lsp_initialize_and_capabilities() {
    let msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    let resp = handle_lsp_message(msg);
    assert!(resp.is_some());
    let r = resp.unwrap();
    assert!(r.contains("completionProvider"));
    assert!(r.contains("hoverProvider"));
    assert!(r.contains("definitionProvider"));
    assert!(r.contains("implementationProvider"));
    assert!(r.contains("semanticTokensProvider"));
}

#[test]
fn test_lsp_completion_and_hover() {
    let comp_msg = r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///main.sora"},"position":{"line":0,"character":0}}}"#;
    let comp_resp = handle_lsp_message(comp_msg).unwrap();
    assert!(comp_resp.contains("async fn"));
    assert!(comp_resp.contains("@derive"));

    let hover_msg = r#"{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{"textDocument":{"uri":"file:///main.sora"},"position":{"line":0,"character":3}}}"#;
    let hover_resp = handle_lsp_message(hover_msg).unwrap();
    assert!(hover_resp.contains("Sorayunara Symbol") || hover_resp.contains("Symbol"));
}

#[test]
fn test_lsp_navigation_and_code_action_and_semantic_tokens() {
    let def_msg = r#"{"jsonrpc":"2.0","id":4,"method":"textDocument/definition","params":{"textDocument":{"uri":"file:///main.sora"}}}"#;
    assert!(handle_lsp_message(def_msg).unwrap().contains("range"));

    let impl_msg = r#"{"jsonrpc":"2.0","id":5,"method":"textDocument/implementation","params":{"textDocument":{"uri":"file:///main.sora"}}}"#;
    assert!(handle_lsp_message(impl_msg).unwrap().contains("range"));

    let ref_msg = r#"{"jsonrpc":"2.0","id":6,"method":"textDocument/references","params":{"textDocument":{"uri":"file:///main.sora"}}}"#;
    assert!(handle_lsp_message(ref_msg).unwrap().contains("range"));

    let action_msg = r#"{"jsonrpc":"2.0","id":9,"method":"textDocument/codeAction","params":{"textDocument":{"uri":"file:///main.sora"}}}"#;
    assert!(
        handle_lsp_message(action_msg)
            .unwrap()
            .contains("source.organizeImports")
    );

    let tokens_msg = r#"{"jsonrpc":"2.0","id":10,"method":"textDocument/semanticTokens/full","params":{"textDocument":{"uri":"file:///main.sora"}}}"#;
    assert!(handle_lsp_message(tokens_msg).unwrap().contains("data"));
}
