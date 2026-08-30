use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::macro_expander::expand_macros;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{Value, execute_ir};

#[test]
fn test_compile_time_constants_and_comptime_blocks() {
    let source = r#"
        const PORT: Int = 8080
        const TIMEOUT: Int = 30 * 1000

        comptime {
            let api_ready: Bool = true
        }

        fn main() -> Int {
            return PORT
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let val = execute_ir(ir).unwrap();
    assert_eq!(val, Value::Int(8080));
}

#[test]
fn test_compile_time_derive_and_reflection() {
    let source = r#"
        @derive(Debug, Clone, Serialize, PartialEq)
        struct User {
            name: String,
            age: Int
        }

        fn main() -> String {
            let u: User = User { name: "Aether", age: 20 }
            return debug_User(u)
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let expanded = expand_macros(program);

    // Check generated functions
    let fn_names: Vec<String> = expanded
        .statements
        .iter()
        .filter_map(|s| match &s.kind {
            sorayunara::ast::StmtKind::Function { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();

    assert!(fn_names.contains(&"debug_User".to_string()));
    assert!(fn_names.contains(&"clone_User".to_string()));
    assert!(fn_names.contains(&"serialize_User".to_string()));
    assert!(fn_names.contains(&"eq_User".to_string()));
    assert!(fn_names.contains(&"reflect_fields_User".to_string()));
}

#[test]
fn test_compile_time_conditional_compilation_cfg() {
    let source = r#"
        @cfg("disabled")
        fn ignored_fn() -> Int {
            return 999
        }

        fn active_fn() -> Int {
            return 123
        }

        fn main() -> Int {
            return active_fn()
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let expanded = expand_macros(program);

    let fn_names: Vec<String> = expanded
        .statements
        .iter()
        .filter_map(|s| match &s.kind {
            sorayunara::ast::StmtKind::Function { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();

    assert!(!fn_names.contains(&"ignored_fn".to_string()));
    assert!(fn_names.contains(&"active_fn".to_string()));
}
