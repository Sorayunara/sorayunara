use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::vm::{execute_ir, Value};

#[test]
fn test_vm_arithmetic_execution() {
    let source = r#"
        fn compute() -> Int {
            return 10 * 5 + 2
        }
        fn main() -> Int {
            return compute()
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(52));
}

#[test]
fn test_vm_concurrency_execution() {
    let source = r#"
        async fn worker() -> Int {
            return 999
        }
        fn main() -> Int {
            let t = spawn worker()
            let res = await t
            return res
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(999));
}
