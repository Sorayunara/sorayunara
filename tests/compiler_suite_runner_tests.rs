use sorayunara::codegen::emit_c;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::wasm_backend::emit_wat;

fn run_pass(src: &str) {
    let tokens = tokenize(src).expect("Lexing failed");
    let program = parse(tokens).expect("Parsing failed");
    if let Err(engine) = check_semantics(&program) {
        panic!("Semantic check failed for compile-pass fixture:\n{}", engine.render_all("pass.sora", src));
    }
}

fn run_fail(src: &str) {
    let tokens = match tokenize(src) {
        Ok(t) => t,
        Err(_) => return,
    };
    let program = match parse(tokens) {
        Ok(p) => p,
        Err(_) => return,
    };
    assert!(check_semantics(&program).is_err(), "Compile-fail fixture unexpectedly passed semantics");
}

#[test]
fn test_compiler_pass_math_and_logic() {
    let src = include_str!("../compiler-tests/compile-pass/math_and_logic.sora");
    run_pass(src);
}

#[test]
fn test_compiler_pass_traits_and_adt() {
    let src = include_str!("../compiler-tests/compile-pass/traits_and_adt.sora");
    run_pass(src);
}

#[test]
fn test_compiler_fail_type_mismatch() {
    let src = include_str!("../compiler-tests/compile-fail/type_mismatch.sora");
    run_fail(src);
}

#[test]
fn test_compiler_fail_undeclared_var() {
    let src = include_str!("../compiler-tests/compile-fail/undeclared_var.sora");
    run_fail(src);
}

#[test]
fn test_compiler_diagnostics_non_exhaustive_pattern() {
    let src = include_str!("../compiler-tests/diagnostics/non_exhaustive_pattern.sora");
    let tokens = tokenize(src).unwrap();
    let program = parse(tokens).unwrap();
    let res = check_semantics(&program);
    assert!(res.is_err());
    let rendered = res.unwrap_err().render_all("pattern.sora", src);
    assert!(rendered.contains("pattern") || rendered.contains("exhaustive") || rendered.contains("match") || rendered.contains("error"));
}

#[test]
fn test_compiler_codegen_llvm_math() {
    let src = include_str!("../compiler-tests/codegen/llvm_math.sora");
    let tokens = tokenize(src).unwrap();
    let program = parse(tokens).unwrap();
    let llvm = emit_llvm_ir(&program);
    assert!(llvm.contains("compute_hypot"));
    assert!(llvm.contains("define i64 @compute_hypot"));
}

#[test]
fn test_tree_lexer_tokens() {
    let src = include_str!("lexer/tokens.sora");
    let tokens = tokenize(src).unwrap();
    assert!(tokens.len() >= 6);
}

#[test]
fn test_tree_parser_grammar() {
    let src = include_str!("parser/grammar.sora");
    let tokens = tokenize(src).unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.statements.len(), 2);
}

#[test]
fn test_tree_typechecker_inference() {
    let src = include_str!("typechecker/inference.sora");
    run_pass(src);
}

#[test]
fn test_tree_borrowchecker_borrow() {
    let src = include_str!("borrowchecker/borrow.sora");
    run_pass(src);
}

#[test]
fn test_tree_codegen_c_emit() {
    let src = include_str!("codegen/c/emit_c.sora");
    let tokens = tokenize(src).unwrap();
    let ast = parse(tokens).unwrap();
    let c = emit_c(&ast);
    assert!(c.contains("square"));
}

#[test]
fn test_tree_wasm_wat_export() {
    let src = include_str!("wasm/wat_export.sora");
    let tokens = tokenize(src).unwrap();
    let ast = parse(tokens).unwrap();
    let wat = emit_wat(&ast);
    assert!(wat.contains("wasm_add"));
}
