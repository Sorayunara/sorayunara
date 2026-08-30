use sorayunara::codegen::emit_c;
use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::{VM, Value};
use sorayunara::wasm_backend::emit_wat;

#[test]
fn test_end_to_end_compile_and_run_sora_program() {
    let source = r#"
        fn fibonacci(n: Int) -> Int {
            if n <= 1 {
                return n
            } else {
                return fibonacci(n - 1) + fibonacci(n - 2)
            }
        }

        fn main() -> Int {
            let res = fibonacci(10)
            return res
        }
    "#;

    // 1. Tokenize
    let tokens = tokenize(source).expect("Lexing failed");

    // 2. Parse into AST
    let ast = parse(tokens).expect("Parsing failed");

    // 3. Semantics & Type Check
    let sem_res = check_semantics(&ast);
    assert!(sem_res.is_ok(), "Semantics failed");

    // 4. Lowers to Multi-Backend Targets
    let c_code = emit_c(&ast);
    assert!(c_code.contains("fibonacci"));
    assert!(c_code.contains("main"));

    let llvm_ir = emit_llvm_ir(&ast);
    assert!(llvm_ir.contains("define i64 @fibonacci(i64 %n)"));
    assert!(llvm_ir.contains("define i64 @main()"));

    let wat = emit_wat(&ast);
    assert!(wat.contains("(func $fibonacci"));
    assert!(wat.contains("(func $main"));

    // 5. Compiles to Bytecode IR & Executes in VM
    let ir = compile_to_ir(&ast);
    let mut vm = VM::new(ir);
    let result = vm.run().expect("VM execution failed");

    // Fib(10) == 55
    assert_eq!(result, Value::Int(55));
}
