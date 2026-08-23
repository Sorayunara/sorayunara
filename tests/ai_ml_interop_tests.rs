use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::execute_ir;
use std::fs;
use std::path::Path;

#[test]
fn test_ml_modules_semantic_validity() {
    let modules = vec!["std/ml.ae", "std/tensor.ae", "std/cuda.ae"];
    for mod_path in modules {
        let path = Path::new(mod_path);
        assert!(path.exists(), "Missing module: {}", mod_path);
        let content = fs::read_to_string(path).unwrap();
        let tokens = tokenize(&content).unwrap();
        let program = parse(tokens).unwrap();
        assert!(
            check_semantics(&program).is_ok(),
            "Semantic check failed for {}",
            mod_path
        );
    }
}

#[test]
fn test_ml_onnx_inference_pipeline() {
    let source = r#"
        fn main() -> Int {
            let model_path = "resnet50.onnx"
            let input_id: Int = 100
            let model_id: Int = 2001
            let output_id: Int = model_id + input_id
            return output_id
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, sorayunara::vm::Value::Int(2101));
}
