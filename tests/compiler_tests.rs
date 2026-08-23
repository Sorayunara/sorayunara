use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;

#[test]
fn test_ir_generation_and_opcodes() {
    let source = r#"
        fn main() {
            let mut i: Int = 0
            while i < 5 {
                i = i + 1
            }
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    assert!(ir.functions.contains_key("main"));
}
