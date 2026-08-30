use sorayunara::hir::lower_ast_to_hir;
use sorayunara::ir::{OpCode, compile_to_ir};
use sorayunara::lexer::tokenize;
use sorayunara::mir::lower_hir_to_mir;
use sorayunara::optimizer::Optimizer;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{Value, execute_ir};

#[test]
fn test_end_to_end_compiler_pipeline() {
    let source = r#"
        fn square(x: Int) -> Int {
            return x * x
        }

        fn main() -> Int {
            let val: Int = square(5)
            return val + 10
        }
    "#;

    // 1. Lexer
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());

    // 2. Parser -> AST
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.statements.len(), 2);

    // 3. Name Resolution & Type Checking
    assert!(check_semantics(&ast).is_ok());

    // 4. HIR Lowering
    let hir = lower_ast_to_hir(&ast);
    assert_eq!(hir.functions.len(), 2);

    // 5. MIR Lowering (Control Flow Graph & Basic Blocks)
    let mir = lower_hir_to_mir(&hir);
    assert_eq!(mir.functions.len(), 2);

    // 6. Backend IR Generation
    let ir = compile_to_ir(&ast);

    // 7. Multi-Pass Optimization Pipeline
    let mut optimizer = Optimizer::new();
    let opt_ir = optimizer.optimize_program(ir);

    // 8. Execution
    let result = execute_ir(opt_ir).unwrap();
    assert_eq!(result, Value::Int(35));
}

#[test]
fn test_optimizer_passes_constant_folding_and_dce_and_lto() {
    let source = r#"
        fn unused_dead_function() -> Int {
            return 999
        }

        fn main() -> Int {
            let folded: Int = 10 + 20 * 2
            return folded
            let unreachable_code: Int = 100
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let ir = compile_to_ir(&ast);

    let mut optimizer = Optimizer::new();
    let opt_ir = optimizer.optimize_program(ir);

    // Verify LTO removed unused_dead_function
    assert!(!opt_ir.functions.contains_key("unused_dead_function"));
    assert!(opt_ir.functions.contains_key("main"));

    // Verify Constant Folding folded operations into PushInt(50)
    let main_func = opt_ir.functions.get("main").unwrap();
    assert!(main_func.instructions.contains(&OpCode::PushInt(50)));

    let result = execute_ir(opt_ir).unwrap();
    assert_eq!(result, Value::Int(50));
}
