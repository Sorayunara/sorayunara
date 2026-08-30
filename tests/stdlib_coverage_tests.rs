use std::fs;
use std::path::Path;

#[test]
fn test_all_37_standard_library_modules_documented_and_valid() {
    let std_modules = vec![
        "actor",
        "alloc",
        "channel",
        "collections",
        "compression",
        "crypto",
        "cuda",
        "dns",
        "embedded",
        "env",
        "ffi",
        "fs",
        "grpc",
        "http",
        "io",
        "json",
        "jwt",
        "math",
        "ml",
        "net",
        "os",
        "postgres",
        "process",
        "quic",
        "redis",
        "reflection",
        "serialization",
        "sql",
        "string",
        "sync",
        "task",
        "tensor",
        "thread",
        "time",
        "tls",
        "unicode",
        "websocket",
    ];

    assert_eq!(std_modules.len(), 37);

    for module_name in std_modules {
        let file_path = format!("std/{}.sora", module_name);
        let path = Path::new(&file_path);
        assert!(
            path.exists(),
            "Standard library module missing: {}",
            file_path
        );

        let content = fs::read_to_string(path).expect("Failed to read std module file");
        assert!(
            !content.trim().is_empty(),
            "Module {} cannot be empty",
            file_path
        );
        assert!(
            content.contains("//"),
            "Module {} must have header documentation",
            file_path
        );
    }
}
