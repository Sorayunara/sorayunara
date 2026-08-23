use std::path::Path;

#[test]
fn test_official_ecosystem_all_pillars_exist() {
    let pillars = vec![
        ("Language Specification", "SPECIFICATION.md"),
        ("Compiler Core Pipeline", "src/optimizer.rs"),
        ("LLVM & Native Backend", "src/llvm_backend.rs"),
        ("Virtual Machine & Runtime", "src/vm.rs"),
        ("Standard Library alloc", "std/alloc.sora"),
        ("Standard Library collections", "std/collections.sora"),
        ("Standard Library crypto", "std/crypto.sora"),
        ("Standard Library http", "std/http.sora"),
        ("Standard Library net", "std/net.sora"),
        ("Standard Library embedded", "std/embedded.sora"),
        ("Standard Library ml", "std/ml.sora"),
        ("Package Registry Client", "src/registry.rs"),
        ("Lockfile Deterministic Engine", "src/lockfile.rs"),
        ("Language Server Protocol Daemon", "src/lsp.rs"),
        ("Formatter Engine", "src/formatter.rs"),
        ("Linter Engine", "src/linter.rs"),
        ("Interactive Debugger (DAP)", "src/debugger.rs"),
        ("Performance Profiler", "src/profiler.rs"),
        ("Documentation Generator", "src/docgen.rs"),
        ("Playground Web App (HTML)", "playground/index.html"),
        ("Playground Web App (JS)", "playground/app.js"),
        ("Playground Web App (CSS)", "playground/style.css"),
        ("VS Code Extension Manifest", "editors/vscode/package.json"),
        ("VS Code Extension Source", "editors/vscode/src/extension.ts"),
    ];

    for (name, rel_path) in pillars {
        let path = Path::new(rel_path);
        assert!(path.exists(), "Ecosystem pillar missing: {} ({})", name, rel_path);
    }
}
