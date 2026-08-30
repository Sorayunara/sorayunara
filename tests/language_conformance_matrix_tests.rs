use sorayunara::codegen::emit_c;
use sorayunara::hir::lower_ast_to_hir;
use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::mir::lower_hir_to_mir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{VM, Value};
use sorayunara::wasm_backend::emit_wat;

#[test]
fn test_foundation_1_grammar_and_parser_conformance() {
    let source = r#"
        struct TransformConfig {
            factor: Int
            threshold: Int
        }

        fn compute(cfg: TransformConfig, val: Int) -> Int {
            let doubled = val * cfg.factor
            if doubled > cfg.threshold {
                return doubled
            } else {
                return 0
            }
        }
    "#;

    let tokens = tokenize(source).expect("1. Lexer conformance failed");
    let ast = parse(tokens).expect("1. Parser conformance failed");
    assert_eq!(ast.statements.len(), 2);
}

#[test]
fn test_foundation_2_type_system_and_hindley_milner_conformance() {
    let source = r#"
        fn infer_add(a: Int, b: Int) -> Int {
            let x = a
            let y = b
            return x + y
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    assert!(
        check_semantics(&ast).is_ok(),
        "2. Type system conformance failed"
    );
}

#[test]
fn test_foundation_3_borrow_checker_and_move_semantics_conformance() {
    // Valid borrowing
    let valid_borrow = r#"
        fn borrow_read(val: &Int) -> &Int {
            return val
        }
    "#;
    let ast_valid = parse(tokenize(valid_borrow).unwrap()).unwrap();
    assert!(check_semantics(&ast_valid).is_ok());

    // Invalid use-after-move must be rejected by compiler
    let invalid_move = r#"
        fn use_after_move() {
            let a: String = "owned_resource"
            let b: String = a
            let c: String = a
        }
    "#;
    let ast_invalid = parse(tokenize(invalid_move).unwrap()).unwrap();
    assert!(
        check_semantics(&ast_invalid).is_err(),
        "3. Borrow checker must reject use-after-move"
    );
}

#[test]
fn test_foundation_4_unified_ir_pipeline_conformance() {
    let source = r#"
        fn multiply_add(a: Int, b: Int, c: Int) -> Int {
            return (a * b) + c
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();

    // AST -> HIR
    let hir = lower_ast_to_hir(&ast);
    assert_eq!(hir.functions.len(), 1);

    // HIR -> MIR
    let mir = lower_hir_to_mir(&hir);
    assert!(mir.functions.contains_key("multiply_add"));

    // AST -> Bytecode IR
    let ir = compile_to_ir(&ast);
    assert!(ir.functions.contains_key("multiply_add"));
}

#[test]
fn test_foundation_5_cross_backend_semantic_conformance() {
    let source = r#"
        fn transform_item(x: Int) -> Int {
            return x * 2
        }

        fn main() -> Int {
            let a = transform_item(3)
            let b = transform_item(4)
            let c = transform_item(5)
            return a + b + c
        }
    "#;

    let tokens = tokenize(source).expect("Lexing failed");
    let ast = parse(tokens).expect("Parsing failed");
    assert!(check_semantics(&ast).is_ok());

    // 1. VM Backend Execution Conformance
    let ir = compile_to_ir(&ast);
    let mut vm = VM::new(ir);
    let vm_output = vm.run().expect("VM execution failed");
    // (3*2) + (4*2) + (5*2) = 6 + 8 + 10 = 24
    assert_eq!(vm_output, Value::Int(24));

    // 2. LLVM IR Backend Conformance
    let llvm_ir = emit_llvm_ir(&ast);
    assert!(llvm_ir.contains("define i64 @transform_item(i64 %x)"));
    assert!(llvm_ir.contains("define i64 @main()"));

    // 3. C99 Backend Conformance
    let c_code = emit_c(&ast);
    assert!(c_code.contains("transform_item"));
    assert!(c_code.contains("main"));

    // 4. WebAssembly WAT Backend Conformance
    let wat = emit_wat(&ast);
    assert!(wat.contains("(func $transform_item"));
    assert!(wat.contains("(func $main"));
}
