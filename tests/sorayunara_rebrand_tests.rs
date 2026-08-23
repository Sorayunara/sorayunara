use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{execute_ir, Value};
use std::fs;
use std::path::Path;

#[test]
fn test_sorayunara_sora_file_extension_execution() {
    let sora_file = Path::new("examples/main.sora");
    assert!(sora_file.exists());

    let source = fs::read_to_string(sora_file).unwrap();
    let tokens = tokenize(&source).unwrap();
    let program = parse(tokens).unwrap();

    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(42));
}

#[test]
fn test_sorayunara_elegant_minimalist_syntax() {
    let source = r#"
        fn increment(x: Int) -> Int {
            return x + 1
        }

        fn compute(n: Int) -> Int {
            let next: Int = increment(n)
            return next * 2
        }

        fn main() -> Int {
            let res: Int = compute(20)
            return res
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(42));
}
