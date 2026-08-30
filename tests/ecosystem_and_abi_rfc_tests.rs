use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use std::path::Path;

fn verify_benchmark_file(name: &str, content: &str) {
    let tokens = tokenize(content).unwrap_or_else(|e| panic!("Lexing failed for benchmark {}: {:?}", name, e));
    let ast = parse(tokens).unwrap_or_else(|e| panic!("Parsing failed for benchmark {}: {:?}", name, e));
    assert!(check_semantics(&ast).is_ok(), "Semantics failed for benchmark {}", name);
}

#[test]
fn test_all_9_benchmark_fixtures() {
    let benches = [
        ("fibonacci", include_str!("../benchmarks/fibonacci/fib.sora")),
        ("matrix", include_str!("../benchmarks/matrix/matrix_mult.sora")),
        ("json", include_str!("../benchmarks/json/json_bench.sora")),
        ("http", include_str!("../benchmarks/http/http_bench.sora")),
        ("hashmap", include_str!("../benchmarks/hashmap/hashmap_bench.sora")),
        ("sorting", include_str!("../benchmarks/sorting/sort_bench.sora")),
        ("concurrency", include_str!("../benchmarks/concurrency/actor_bench.sora")),
        ("memory", include_str!("../benchmarks/memory/alloc_bench.sora")),
        ("compiler", include_str!("../benchmarks/compiler/lex_bench.sora")),
    ];

    for (name, content) in benches {
        verify_benchmark_file(name, content);
    }
}

#[test]
fn test_rfc_system_integrity() {
    let rfcs = [
        "rfcs/0001-language-evolution.md",
        "rfcs/0002-generics.md",
        "rfcs/0003-traits.md",
        "rfcs/0004-async.md",
        "rfcs/0005-ffi.md",
        "rfcs/0006-package-registry.md",
    ];

    for rfc in rfcs {
        assert!(Path::new(rfc).exists(), "Missing required RFC document: {}", rfc);
    }
}

#[test]
fn test_abi_spec_document_exists() {
    assert!(Path::new("docs/language-spec/abi-specification.md").exists());
}
