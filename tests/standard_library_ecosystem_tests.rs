use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

fn verify_std_module(name: &str, content: &str) {
    let tokens = tokenize(content)
        .unwrap_or_else(|e| panic!("Lexing failed for std/{}.sora: {:?}", name, e));
    let ast =
        parse(tokens).unwrap_or_else(|e| panic!("Parsing failed for std/{}.sora: {:?}", name, e));
    if let Err(engine) = check_semantics(&ast) {
        panic!(
            "Semantics failed for std/{}.sora:\n{}",
            name,
            engine.render_all(&format!("{}.sora", name), content)
        );
    }
}

#[test]
fn test_all_18_core_standard_library_modules() {
    let modules = [
        ("alloc", include_str!("../std/alloc.sora")),
        ("collections", include_str!("../std/collections.sora")),
        ("convert", include_str!("../std/convert.sora")),
        ("env", include_str!("../std/env.sora")),
        ("fs", include_str!("../std/fs.sora")),
        ("io", include_str!("../std/io.sora")),
        ("iter", include_str!("../std/iter.sora")),
        ("math", include_str!("../std/math.sora")),
        ("net", include_str!("../std/net.sora")),
        ("path", include_str!("../std/path.sora")),
        ("process", include_str!("../std/process.sora")),
        ("sync", include_str!("../std/sync.sora")),
        ("thread", include_str!("../std/thread.sora")),
        ("time", include_str!("../std/time.sora")),
        ("crypto", include_str!("../std/crypto.sora")),
        ("encoding", include_str!("../std/encoding.sora")),
        ("testing", include_str!("../std/testing.sora")),
        ("ffi", include_str!("../std/ffi.sora")),
    ];

    for (name, content) in modules {
        verify_std_module(name, content);
    }
}
