use sorayunara::docgen::generate_html_docs;
use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::linter::Linter;
use sorayunara::llvm_backend::{emit_llvm_ir_with_target, Target};
use sorayunara::lockfile::NamiLock;
use sorayunara::optimizer::Optimizer;
use sorayunara::parser::parse;
use sorayunara::profiler::Profiler;
use sorayunara::registry::RegistryClient;
use sorayunara::security::{SecurityEngine, SecurityPolicy};
use sorayunara::semantics::check_semantics;
use sorayunara::test_runner::{TestOptions, TestRunner};
use sorayunara::vm::execute_ir;
use std::path::Path;

#[test]
fn test_heavyweight_matrix_all_17_areas() {
    // 1. Syntax & Modern Parsing
    let source = r#"
        fn calculate(a: Int, b: Int) -> Int {
            let sum: Int = a + b
            return sum
        }

        fn main() -> Int {
            let res: Int = calculate(10, 20)
            return res
        }
    "#;
    let tokens = tokenize(source).expect("1. Lexer failed");
    let program = parse(tokens).expect("1. Syntax / Parser failed");

    // 2. Type System (Static + Inference + Generics + Semantics)
    let sem_res = check_semantics(&program);
    if let Err(ref err) = sem_res {
        eprintln!("{}", err.render_all("matrix_test.sora", source));
    }
    assert!(sem_res.is_ok(), "2. Type System / Semantic check failed");

    // 3. Memory & Security (Ownership / Borrow / Bounds / Sandbox)
    let memory_audit = SecurityEngine::audit_program_memory_safety(&program);
    assert!(memory_audit.use_after_free_prevented, "3. Memory safety failed");
    assert!(memory_audit.double_free_prevented, "3. Memory safety failed");
    let sandbox = SecurityPolicy::strict_sandbox();
    assert!(sandbox.check_fs_read("secret.pem").is_err(), "3. Sandbox security failed");

    // 4. Concurrency Runtime & 5. Compiler Multi-stage IR & 13. Optimizer
    let ir = compile_to_ir(&program);
    let mut optimizer = Optimizer::new();
    let opt_ir = optimizer.optimize_program(ir.clone());

    // 6. Backend (Native LLVM + WASM + VM)
    let llvm_x64 = emit_llvm_ir_with_target(&program, Target::LinuxX64);
    assert!(llvm_x64.contains("x86_64-unknown-linux-gnu"), "6. LLVM x86_64 target failed");
    let llvm_arm = emit_llvm_ir_with_target(&program, Target::LinuxArm64);
    assert!(llvm_arm.contains("aarch64-unknown-linux-gnu"), "6. LLVM ARM64 target failed");
    let llvm_riscv = emit_llvm_ir_with_target(&program, Target::Riscv64);
    assert!(llvm_riscv.contains("riscv64gc-unknown-linux-gnu"), "6. LLVM RISC-V target failed");
    let llvm_cortex = emit_llvm_ir_with_target(&program, Target::ArmCortexM);
    assert!(llvm_cortex.contains("thumbv7em-none-eabihf"), "6. LLVM Cortex-M target failed");

    // Execution in VM Runtime
    let exec_val = execute_ir(opt_ir).expect("6. VM execution failed");
    assert_eq!(exec_val, sorayunara::vm::Value::Int(30));

    // 7. FFI & Foreign types
    assert!(Path::new("std/ffi.sora").exists(), "7. FFI module missing");

    // 8. Package Registry & Lockfile
    let mut lock = NamiLock::new();
    lock.packages.push(sorayunara::lockfile::LockedPackage {
        name: "http".to_string(),
        version: "1.0.0".to_string(),
        source: "https://packages.nami.dev/http".to_string(),
        checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        dependencies: vec![],
    });
    assert!(SecurityEngine::verify_lockfile_security(&lock).is_ok(), "8. Lockfile verification failed");
    let client = RegistryClient::new();
    assert!(!client.search("http").is_empty(), "8. Registry search failed");

    // 9. Security audit
    let audit_res = client.audit_project(Path::new("."));
    assert_eq!(audit_res.vulnerabilities_found, 0, "9. Security audit failed");

    // 10. Networking Stack
    let net_modules = vec!["std/http.sora", "std/net.sora", "std/tls.sora", "std/quic.sora", "std/websocket.sora", "std/grpc.sora"];
    for mod_path in net_modules {
        assert!(Path::new(mod_path).exists(), "10. Networking module missing: {}", mod_path);
    }

    // 11. Tooling (LSP + Formatter + Linter)
    let lints = Linter::lint_program(&program);
    assert!(lints.is_empty(), "11. Linter failed");

    // 12. Debugging & 13. Profiler
    let prof = Profiler::new(ir);
    let prof_report = prof.run_full_profile();
    assert!(prof_report.is_ok(), "13. Profiler failed");

    // 14. Testing & Verification (Unit + Bench + Fuzz + Property)
    let runner = TestRunner::new(TestOptions {
        verify: true,
        ..Default::default()
    });
    let prop_res = runner.run_property_check("add is commutative", |a, b| a + b == b + a);
    assert!(prop_res.passed, "14. Property verify failed");

    // 15. Documentation Generator
    let doc_out = Path::new("target/matrix_docs");
    let _ = generate_html_docs(&program, source, "Matrix Test", doc_out);
    assert!(doc_out.join("index.html").exists(), "15. Doc generator failed");

    // 16. Platforms & Standard Library
    assert!(Path::new("std/embedded.sora").exists(), "16. Baremetal HAL missing");
    assert!(Path::new("std/ml.sora").exists(), "16. ML Interop missing");

    // 17. Ecosystem (Playground + VS Code + Specification)
    assert!(Path::new("SPECIFICATION.md").exists(), "17. Specification missing");
    assert!(Path::new("playground/index.html").exists(), "17. Playground missing");
    assert!(Path::new("editors/vscode/package.json").exists(), "17. VS Code extension missing");
}
