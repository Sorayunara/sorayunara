use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::{Target, emit_llvm_ir_with_target};
use sorayunara::parser::parse;
use sorayunara::wasm_backend::emit_wat;

#[test]
fn test_target_webassembly_wat_codegen() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }
        fn main() -> Int {
            return add(10, 20)
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let wat = emit_wat(&program);

    assert!(wat.contains("(module"));
    assert!(wat.contains("(func $add (export \"add\") (param $a i64) (param $b i64) (result i64)"));
    assert!(wat.contains("i64.add"));
    assert!(wat.contains("(func $main (export \"main\") (result i64)"));
}

#[test]
fn test_target_cross_platform_llvm_triples() {
    let source = r#"
        fn main() -> Int {
            return 100
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let targets = vec![
        (Target::LinuxX64, "x86_64-unknown-linux-gnu"),
        (Target::LinuxArm64, "aarch64-unknown-linux-gnu"),
        (Target::WindowsX64, "x86_64-pc-windows-msvc"),
        (Target::WindowsArm64, "aarch64-pc-windows-msvc"),
        (Target::MacosArm64, "arm64-apple-macosx"),
        (Target::MacosX64, "x86_64-apple-darwin"),
        (Target::Wasm32, "wasm32-unknown-wasi"),
    ];

    for (target, expected_triple) in targets {
        let ir = emit_llvm_ir_with_target(&program, target);
        assert!(ir.contains(&format!("target triple = \"{}\"", expected_triple)));
    }
}
