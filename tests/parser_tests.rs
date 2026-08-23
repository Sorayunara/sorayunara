use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;

#[test]
fn test_parse_functions_and_structs() {
    let source = r#"
        struct User {
            id: Int,
            name: String,
        }

        fn get_user(id: Int) -> User {
            return User
        }
    "#;
    let tokens = tokenize(source).expect("Tokenization should succeed");
    let program = parse(tokens).expect("Parsing should succeed");
    assert_eq!(program.statements.len(), 2);
}

#[test]
fn test_parse_pattern_matching() {
    let source = r#"
        fn check_opt(opt: Option<Int>) -> Int {
            let res = match opt {
                Some(v) => v,
                None => 0,
            }
            return res
        }
    "#;
    let tokens = tokenize(source).expect("Tokenization should succeed");
    let program = parse(tokens).expect("Parsing should succeed");
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_parse_concurrency() {
    let source = r#"
        async fn background() -> String {
            return "done"
        }
        fn main() {
            let t: Task<String> = spawn background()
            let res: String = await t
        }
    "#;
    let tokens = tokenize(source).expect("Tokenization should succeed");
    let program = parse(tokens).expect("Parsing should succeed");
    assert_eq!(program.statements.len(), 2);
}
