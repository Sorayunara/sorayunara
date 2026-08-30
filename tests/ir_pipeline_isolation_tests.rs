use sorayunara::codegen::emit_c;
use sorayunara::hir::lower_ast_to_hir;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::mir::lower_hir_to_mir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::wasm_backend::emit_wat;

#[test]
fn test_isolated_pipeline_ast_to_hir() {
    let src = r#"
        fn multiply(a: Int, b: Int) -> Int {
            return a * b
        }
    "#;
    let tokens = tokenize(src).expect("Lexer failed");
    let ast = parse(tokens).expect("Parser failed");
    assert_eq!(ast.statements.len(), 1);

    let hir = lower_ast_to_hir(&ast);
    assert_eq!(hir.functions.len(), 1);
    assert_eq!(hir.functions[0].name, "multiply");
    assert_eq!(hir.functions[0].params.len(), 2);
}

#[test]
fn test_isolated_pipeline_hir_to_mir() {
    let src = r#"
        fn branch_check(x: Int) -> Int {
            if x > 10 {
                return 1
            } else {
                return 0
            }
        }
    "#;
    let tokens = tokenize(src).expect("Lexer failed");
    let ast = parse(tokens).expect("Parser failed");
    let hir = lower_ast_to_hir(&ast);
    let mir = lower_hir_to_mir(&hir);

    assert!(mir.functions.contains_key("branch_check"));
    let body = mir.functions.get("branch_check").unwrap();
    assert!(!body.basic_blocks.is_empty());
}

#[test]
fn test_isolated_pipeline_semantic_and_borrow_analysis() {
    let src = r#"
        fn compute(val: Int) -> &Int {
            let x: Int = val
            let r: &Int = &x
            return r
        }
    "#;
    let tokens = tokenize(src).expect("Lexer failed");
    let ast = parse(tokens).expect("Parser failed");
    assert!(check_semantics(&ast).is_ok());
}

#[test]
fn test_isolated_pipeline_multi_backend_emission() {
    let src = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }
    "#;
    let tokens = tokenize(src).expect("Lexer failed");
    let ast = parse(tokens).expect("Parser failed");

    // 1. C99 Emission
    let c_code = emit_c(&ast);
    assert!(c_code.contains("long long add("));

    // 2. LLVM IR Emission
    let llvm_ir = emit_llvm_ir(&ast);
    assert!(llvm_ir.contains("define i64 @add("));

    // 3. WebAssembly WAT Emission
    let wat_code = emit_wat(&ast);
    assert!(wat_code.contains("(func $add"));
}
