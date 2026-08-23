use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::native_builder::NativeBuilder;
use sorayunara::codegen::emit_c;
use sorayunara::vm::{execute_ir, Value};
use std::fs;
use std::path::Path;

#[test]
fn test_self_hosting_compiler_source_verification() {
    let self_host_file = Path::new("compiler/main.sora");
    assert!(self_host_file.exists(), "compiler/main.sora must exist");

    let source = fs::read_to_string(self_host_file).unwrap();
    let tokens = tokenize(&source).unwrap();
    let program = parse(tokens).unwrap();

    assert!(check_semantics(&program).is_ok());

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(0));
}

#[test]
fn test_direct_native_machine_code_emission() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }

        fn main() -> Int {
            let res: Int = add(20, 22)
            return res
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let c_code = emit_c(&program);

    let output_target = "target/test_direct_native_app.exe";
    let build_result = NativeBuilder::build_executable(&c_code, output_target);
    assert!(build_result.is_ok());
}
