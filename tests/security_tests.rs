use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::security::SecurityPolicy;
use sorayunara::vm::execute_ir;

#[test]
fn test_security_integer_overflow_protection() {
    let source = r#"
        fn main() -> Int {
            let max_val: Int = 9223372036854775807
            let overflowed: Int = max_val + 1
            return overflowed
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let result = execute_ir(ir);

    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("Integer overflow"));
}

#[test]
fn test_security_bounds_checking() {
    let source = r#"
        fn main() -> Int {
            let arr = [10, 20, 30]
            let out_of_bounds = arr[99]
            return out_of_bounds
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);
    let result = execute_ir(ir);

    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("out of bounds"));
}

#[test]
fn test_security_capability_sandbox() {
    let policy = SecurityPolicy::strict_sandbox();
    assert!(policy.check_fs_read("secret.key").is_err());
    assert!(policy.check_fs_write("system.dll").is_err());
    assert!(policy.check_network("https://malicious.site").is_err());
    assert!(policy.check_process_spawn("sh").is_err());
    assert!(policy.check_ffi_invocation("system").is_err());

    let permissive = SecurityPolicy::default();
    assert!(permissive.check_fs_read("allowed.txt").is_ok());
    assert!(permissive.check_network("https://api.aether.dev").is_ok());
    assert!(permissive.check_process_spawn("git").is_ok());
    assert!(permissive.check_ffi_invocation("strlen").is_ok());
}

#[test]
fn test_memory_safety_audit_and_lockfile_security() {
    let source = "fn main() -> Int { return 42 }";
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let audit = sorayunara::security::SecurityEngine::audit_program_memory_safety(&program);
    assert!(audit.use_after_free_prevented);
    assert!(audit.double_free_prevented);
    assert!(audit.buffer_overflow_prevented);
    assert!(audit.data_race_prevented);
    assert!(audit.dangling_pointer_prevented);

    let mut lockfile = sorayunara::lockfile::AetherLock::new();
    lockfile.packages.push(sorayunara::lockfile::LockedPackage {
        name: "http".to_string(),
        version: "2.1.0".to_string(),
        source: "https://pkg.aether.dev/http/2.1.0".to_string(),
        checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            .to_string(),
        dependencies: vec![],
    });
    assert!(sorayunara::security::SecurityEngine::verify_lockfile_security(&lockfile).is_ok());
}
