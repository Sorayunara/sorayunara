use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::optimizer::optimize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{Value, execute_ir};

#[test]
fn test_const_declaration_and_folding() {
    let source = r#"
        const APP_VERSION: String = "1.0.0"
        const MAX_CONNECTIONS: Int = 100 * 10
        fn main() -> Int {
            let limit: Int = MAX_CONNECTIONS
            return limit
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let opt_ir = optimize(ir);
    let result = execute_ir(opt_ir).unwrap();
    assert_eq!(result, Value::Int(1000));
}

#[test]
fn test_comptime_block_evaluation() {
    let source = r#"
        comptime {
            let compile_check: Int = 42
        }
        fn main() -> Int {
            return 42
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(42));
}
