use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_move_expression_parses() {
    let source = r#"
        fn make_user() -> String {
            return "Aether"
        }

        fn main() {
            let user: String = make_user()
            let owner: String = move user
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(program.statements.len() >= 2);
}

#[test]
fn test_unsafe_block_statement_parses() {
    let source = r#"
        fn main() {
            unsafe {
                let x: Int = 42
            }
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(program.statements.len() >= 1);
}

#[test]
fn test_unsafe_block_expression_parses() {
    let source = r#"
        fn main() {
            let ptr_value: Int = unsafe {
                let raw: Int = 7
                raw
            }
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(program.statements.len() >= 1);
}

#[test]
fn test_move_semantics_moves_string_ok() {
    let source = r#"
        fn make_user() -> String {
            return "Aether"
        }

        fn main() {
            let user: String = make_user()
            let owner: String = move user
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    if let Err(engine) = check_semantics(&program) {
        let rendered = engine.render_all("test.ae", source);
        panic!("Unexpected semantic error:\n{}", rendered);
    }
}

#[test]
fn test_unsafe_block_semantics_ok() {
    let source = r#"
        extern "C" {
            fn malloc(size: Int) -> *mut Void
        }

        fn main() {
            unsafe {
                let buf: *mut Void = malloc(1024)
            }
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    if let Err(engine) = check_semantics(&program) {
        let rendered = engine.render_all("test.ae", source);
        panic!("Unexpected unsafe-block semantic error:\n{}", rendered);
    }
}

#[test]
fn test_move_then_use_moved_value_errors() {
    let source = r#"
        fn make_user() -> String {
            return "Aether"
        }

        fn main() {
            let user: String = make_user()
            let owner: String = move user
            let again: String = user
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_err());
}

#[test]
fn test_borrow_instead_of_move_ok() {
    let source = r#"
        fn main() {
            let user = "Aether"
            let borrowed: &String = &user
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    if let Err(engine) = check_semantics(&program) {
        let rendered = engine.render_all("test.ae", source);
        panic!("Unexpected borrow-check error:\n{}", rendered);
    }
}
