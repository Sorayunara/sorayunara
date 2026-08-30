use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::{Target, emit_llvm_ir_with_target};
use sorayunara::parser::parse;
use sorayunara::vm::{Value, execute_ir};
use sorayunara::wasm_backend::emit_wat;

#[test]
fn test_multi_target_parsing() {
    assert_eq!(Target::parse("linux-x64"), Some(Target::LinuxX64));
    assert_eq!(Target::parse("windows-x64"), Some(Target::WindowsX64));
    assert_eq!(Target::parse("macos-arm64"), Some(Target::MacosArm64));
    assert_eq!(Target::parse("wasm"), Some(Target::Wasm32));
    assert_eq!(Target::parse("riscv64"), Some(Target::Riscv64));
}

#[test]
fn test_native_targets_llvm_emission() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    // 1. Linux x86_64
    let linux_ir = emit_llvm_ir_with_target(&program, Target::LinuxX64);
    assert!(linux_ir.contains("target triple = \"x86_64-unknown-linux-gnu\""));

    // 2. Windows x86_64
    let win_ir = emit_llvm_ir_with_target(&program, Target::WindowsX64);
    assert!(win_ir.contains("target triple = \"x86_64-pc-windows-msvc\""));

    // 3. macOS ARM64
    let mac_ir = emit_llvm_ir_with_target(&program, Target::MacosArm64);
    assert!(mac_ir.contains("target triple = \"arm64-apple-macosx\""));

    // 4. RISC-V 64
    let riscv_ir = emit_llvm_ir_with_target(&program, Target::Riscv64);
    assert!(riscv_ir.contains("target triple = \"riscv64gc-unknown-linux-gnu\""));
}

#[test]
fn test_wasm_target_emission() {
    let source = r#"
        fn compute(val: Int) -> Int {
            return val * 2
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let wat = emit_wat(&program);
    assert!(wat.contains("(module"));
    assert!(wat.contains("(func $compute (export \"compute\")"));
    assert!(wat.contains("i64.mul"));
}

#[test]
fn test_vm_sandbox_target_execution() {
    let source = r#"
        fn main() -> Int {
            let x: Int = 100
            let y: Int = 200
            return x + y
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir).unwrap();
    assert_eq!(result, Value::Int(300));
}
