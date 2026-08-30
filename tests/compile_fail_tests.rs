use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_compile_fail_borrow_immutable_as_mut() {
    let source = r#"
        fn main() {
            let x: Int = 42
            let r: &mut Int = &mut x
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let err = check_semantics(&program).unwrap_err();
    let rendered = err.render_all("main.sora", source);
    assert!(
        rendered.contains("immutable") || rendered.contains("error"),
        "Rendered: {}",
        rendered
    );
}

#[test]
fn test_compile_fail_use_after_move() {
    let source = r#"
        fn main() {
            let s: String = "Sorayunara"
            let moved_s = move s
            let again = s
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let err = check_semantics(&program).unwrap_err();
    let rendered = err.render_all("main.sora", source);
    assert!(
        rendered.contains("moved") || rendered.contains("error"),
        "Rendered: {}",
        rendered
    );
}

#[test]
fn test_compile_fail_type_mismatch_assignment() {
    let source = r#"
        fn main() {
            let x: Int = "This is not an Int"
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let err = check_semantics(&program).unwrap_err();
    let rendered = err.render_all("main.sora", source);
    assert!(
        rendered.contains("mismatch") || rendered.contains("error"),
        "Rendered: {}",
        rendered
    );
}

#[test]
fn test_compile_fail_non_exhaustive_pattern_match() {
    let source = r#"
        fn check_flag(flag: Bool) -> Int {
            match flag {
                true => 1
            }
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let err = check_semantics(&program).unwrap_err();
    let rendered = err.render_all("main.sora", source);
    assert!(
        rendered.contains("non-exhaustive")
            || rendered.contains("pattern")
            || rendered.contains("error"),
        "Rendered: {}",
        rendered
    );
}

#[test]
fn test_compile_fail_undeclared_function_call() {
    let source = r#"
        fn main() {
            non_existent_function(123)
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let err = check_semantics(&program).unwrap_err();
    let rendered = err.render_all("main.sora", source);
    assert!(
        rendered.contains("Unknown")
            || rendered.contains("undeclared")
            || rendered.contains("error"),
        "Rendered: {}",
        rendered
    );
}
