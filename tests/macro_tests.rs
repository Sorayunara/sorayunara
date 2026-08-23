use sorayunara::lexer::tokenize;
use sorayunara::macro_expander::expand_macros;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_macro_derive_debug_and_clone() {
    let source = r#"
        @derive(Debug, Clone)
        struct User {
            id: Int,
            name: String,
        }

        fn main() {
            let str_repr: String = debug_User(10)
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let expanded = expand_macros(program);

    assert!(check_semantics(&expanded).is_ok());
    assert!(expanded.statements.iter().any(|s| match &s.kind {
        sorayunara::ast::StmtKind::Function { name, .. } => name == "debug_User",
        _ => false,
    }));
    assert!(expanded.statements.iter().any(|s| match &s.kind {
        sorayunara::ast::StmtKind::Function { name, .. } => name == "clone_User",
        _ => false,
    }));
}
