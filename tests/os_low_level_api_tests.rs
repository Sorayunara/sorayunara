use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use std::fs;
use std::path::Path;

#[test]
fn test_all_standard_library_modules_validity() {
    let std_modules = vec![
        "alloc.sora",
        "collections.sora",
        "crypto.sora",
        "env.sora",
        "fs.sora",
        "io.sora",
        "net.sora",
        "os.sora",
        "process.sora",
        "sync.sora",
        "thread.sora",
        "time.sora",
        "unicode.sora",
        "compression.sora",
        "serialization.sora",
        "reflection.sora",
    ];

    let std_dir = Path::new("std");
    for mod_name in std_modules {
        let mod_path = std_dir.join(mod_name);
        assert!(mod_path.exists(), "Missing std module: {}", mod_name);

        let content = fs::read_to_string(&mod_path).unwrap();
        let tokens = tokenize(&content).unwrap();
        let program = parse(tokens).unwrap();
        match check_semantics(&program) {
            Ok(_) => {}
            Err(diag) => {
                let rendered = diag.render_all(mod_name, &content);
                panic!("Semantic analysis failed for std/{}:\n{}", mod_name, rendered);
            }
        }
    }
}
