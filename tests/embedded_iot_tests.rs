use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::{Target, emit_llvm_ir_with_target};
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use std::fs;
use std::path::Path;

#[test]
fn test_embedded_targets_parsing_and_triples() {
    let targets = vec![
        ("arm-cortex-m", Target::ArmCortexM, "thumbv7em-none-eabihf"),
        (
            "arm-cortex-a",
            Target::ArmCortexA,
            "aarch64-unknown-none-elf",
        ),
        ("riscv32", Target::Riscv32, "riscv32imac-unknown-none-elf"),
        ("esp32", Target::Esp32, "xtensa-esp32-none-elf"),
        ("embedded", Target::Embedded, "thumbv7m-none-eabi"),
    ];

    for (name, expected_target, expected_triple) in targets {
        let parsed = Target::parse(name);
        assert_eq!(parsed, Some(expected_target), "Failed to parse {}", name);
        assert_eq!(parsed.unwrap().triple(), expected_triple);
        assert!(!parsed.unwrap().datalayout().is_empty());
    }
}

#[test]
fn test_embedded_llvm_ir_emission() {
    let source = r#"
        fn main() -> Int {
            let led_pin: Int = 13
            return led_pin
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let cortex_ir = emit_llvm_ir_with_target(&program, Target::ArmCortexM);
    assert!(cortex_ir.contains("target triple = \"thumbv7em-none-eabihf\""));

    let esp_ir = emit_llvm_ir_with_target(&program, Target::Esp32);
    assert!(esp_ir.contains("target triple = \"xtensa-esp32-none-elf\""));
}

#[test]
fn test_embedded_hal_std_module_validity() {
    let hal_path = Path::new("std/embedded.sora");
    assert!(hal_path.exists());
    let content = fs::read_to_string(hal_path).unwrap();
    let tokens = tokenize(&content).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());
}
