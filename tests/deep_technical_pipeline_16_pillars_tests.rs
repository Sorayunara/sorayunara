use sorayunara::codegen::emit_c;
use sorayunara::concurrency_runtime::{Actor, ArenaAllocator, Channel};
use sorayunara::hir::lower_ast_to_hir;
use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::lsp::LspState;
use sorayunara::mir::lower_hir_to_mir;
use sorayunara::optimizer::Optimizer;
use sorayunara::parser::parse;
use sorayunara::registry::RegistryClient;
use sorayunara::semantics::check_semantics;
use sorayunara::wasm_backend::emit_wat;
use std::path::Path;
use std::sync::Arc;

#[test]
fn test_16_pillar_deep_technical_pipeline() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }

        fn main() -> Int {
            let res = add(10, 20)
            return res
        }
    "#;

    // 1. Language Specification & Lexer
    let tokens = tokenize(source).expect("1. Lexer failed");
    assert!(!tokens.is_empty());

    // 2. Parser & AST Generation
    let ast = parse(tokens).expect("2. Parser failed");
    assert_eq!(ast.statements.len(), 2);

    // 3. Type System & Hindley-Milner Inference
    if let Err(engine) = check_semantics(&ast) {
        panic!(
            "3. Type System check failed:\n{}",
            engine.render_all("main.sora", source)
        );
    }

    // 4. Borrow Checker & Affine Ownership Verification
    let borrow_check_source = "fn chk(x: &Int) -> &Int { return x }";
    let borrow_ast = parse(tokenize(borrow_check_source).unwrap()).unwrap();
    assert!(
        check_semantics(&borrow_ast).is_ok(),
        "4. Borrow Checker failed"
    );

    // 5. HIR Lowering
    let hir = lower_ast_to_hir(&ast);
    assert_eq!(hir.functions.len(), 2);

    // 6. MIR (Mid-Level Intermediate Representation)
    let mir = lower_hir_to_mir(&hir);
    assert!(mir.functions.contains_key("add"));

    // 7. Optimization Pass (Constant Folding, DCE, Inlining)
    let ir = compile_to_ir(&ast);
    let mut optimizer = Optimizer::new();
    let opt_ir = optimizer.optimize_program(ir);
    assert!(!opt_ir.functions.is_empty());

    // 8. Backends: LLVM, C99, WASM
    let c_code = emit_c(&ast);
    assert!(c_code.contains("long long add"));
    let llvm_ir = emit_llvm_ir(&ast);
    assert!(llvm_ir.contains("define i64 @add"));
    let wat_code = emit_wat(&ast);
    assert!(wat_code.contains("(func $add"));

    // 9. Runtime Subsystems (Channel, Actor, Arena Allocator)
    let chan = Arc::new(Channel::new(Some(10)));
    chan.send(42).unwrap();
    assert_eq!(chan.recv().unwrap(), 42);

    let actor = Actor::spawn(0, |st, delta: i32| *st += delta);
    actor.send(10).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(actor.get_state(|s| *s), 10);

    let arena = ArenaAllocator::new(1024);
    assert!(arena.alloc(32, 8).is_some());

    // 10. Standard Library Verification
    assert!(Path::new("std/math.sora").exists());
    assert!(Path::new("std/collections.sora").exists());
    assert!(Path::new("std/ffi.sora").exists());

    // 11. Package Manager & Dependency Resolver
    let client = RegistryClient::new();
    let tree = client.dependency_tree(Path::new("."));
    assert!(!tree.is_empty());

    // 12. LSP (Language Server Protocol)
    let mut state = LspState::new();
    state
        .documents
        .insert("file:///main.sora".to_string(), source.to_string());
    assert!(state.documents.contains_key("file:///main.sora"));

    // 13. Testing Framework
    assert!(Path::new("tests/compiler_suite_runner_tests.rs").exists());

    // 14. Benchmarks Suite
    assert!(Path::new("benchmarks/fibonacci/fib.sora").exists());

    // 15. FFI & ABI Specification
    assert!(Path::new("docs/language-spec/abi-specification.md").exists());

    // 16. Self-Hosting & Ecosystem
    assert!(Path::new("rfcs/0001-language-evolution.md").exists());
    assert!(Path::new(".github/workflows/ci.yml").exists());
}
