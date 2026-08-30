use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{Value, execute_ir};
use std::fs;
use std::path::Path;

#[test]
fn test_networking_modules_semantic_validity() {
    let modules = vec![
        "http.sora",
        "net.sora",
        "tls.sora",
        "dns.sora",
        "quic.sora",
        "websocket.sora",
        "grpc.sora",
    ];

    let std_dir = Path::new("std");
    for mod_name in modules {
        let mod_path = std_dir.join(mod_name);
        assert!(mod_path.exists(), "Missing module std/{}", mod_name);
        let content = fs::read_to_string(&mod_path).unwrap();
        let tokens = tokenize(&content).unwrap();
        let program = parse(tokens).unwrap();
        assert!(
            check_semantics(&program).is_ok(),
            "Semantic check failed on std/{}",
            mod_name
        );
    }
}

#[test]
fn test_http_server_execution_pipeline() {
    let source = r#"
        fn server_new() -> Int {
            return 8080
        }

        fn server_get(server_id: Int, route: String) -> Bool {
            return true
        }

        fn server_post(server_id: Int, route: String) -> Bool {
            return true
        }

        fn server_listen(server_id: Int, addr: String) {
        }

        fn main() -> Int {
            let srv: Int = server_new()
            let ok1: Bool = server_get(srv, "/hello")
            let ok2: Bool = server_post(srv, "/users")
            server_listen(srv, ":8080")
            return srv
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let val = execute_ir(ir).unwrap();
    assert_eq!(val, Value::Int(8080));
}
