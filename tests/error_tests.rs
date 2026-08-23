use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_error_undeclared_variable() {
    let source = "fn main() { let x = unknown_var }";
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_err());
}

#[test]
fn test_error_borrow_checker_mutation_violation() {
    let source = r#"
        fn main() {
            let x: Int = 10
            let r: &mut Int = &mut x
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_err());
}
