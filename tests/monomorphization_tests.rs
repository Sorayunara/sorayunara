use sorayunara::ast::*;
use sorayunara::lexer::tokenize;
use sorayunara::monomorphizer::monomorphize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::ir::compile_to_ir;
use sorayunara::vm::{execute_ir, Value};

#[test]
fn test_generic_monomorphization_zero_cost() {
    let source = r#"
        fn compute(x: T) -> T {
            return x
        }

        fn main() -> Int {
            let res: Int = compute_Int(42)
            return res
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let monomorphized = monomorphize(program);

    assert!(monomorphized.statements.iter().any(|s| match &s.kind {
        StmtKind::Function { name, .. } => name == "compute_Int",
        _ => false,
    }));

    assert!(check_semantics(&monomorphized).is_ok());
    let ir = compile_to_ir(&monomorphized);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(42));
}
