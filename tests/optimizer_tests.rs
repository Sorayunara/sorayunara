use sorayunara::ir::{OpCode, compile_to_ir};
use sorayunara::lexer::tokenize;
use sorayunara::optimizer::optimize;
use sorayunara::parser::parse;

#[test]
fn test_optimizer_constant_folding() {
    let source = r#"
        fn main() -> Int {
            return 10 + 20 * 2
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let opt_ir = optimize(ir);

    let main_fn = opt_ir.functions.get("main").unwrap();
    // 10 + 40 -> 50 folded into single PushInt(50)
    assert!(
        main_fn
            .instructions
            .iter()
            .any(|op| matches!(op, OpCode::PushInt(50)))
    );
}

#[test]
fn test_optimizer_dead_code_elimination() {
    let source = r#"
        fn main() -> Int {
            return 42
            let dead = 100
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let opt_ir = optimize(ir);

    let main_fn = opt_ir.functions.get("main").unwrap();
    assert_eq!(main_fn.instructions.last(), Some(&OpCode::Return));
}
