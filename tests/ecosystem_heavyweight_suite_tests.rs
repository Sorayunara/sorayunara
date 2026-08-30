use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

fn verify_sora_source(name: &str, content: &str) {
    let tokens =
        tokenize(content).unwrap_or_else(|e| panic!("Lexing failed for {}: {:?}", name, e));
    let ast = parse(tokens).unwrap_or_else(|e| panic!("Parsing failed for {}: {:?}", name, e));
    if let Err(diag) = check_semantics(&ast) {
        panic!(
            "Semantic failed for {}:\n{}",
            name,
            diag.render_all(name, content)
        );
    }
}

#[test]
fn test_algorithms_suite_verification() {
    verify_sora_source(
        "sorting",
        include_str!("../benchmarks/algorithms/sorting.sora"),
    );
    verify_sora_source("graph", include_str!("../benchmarks/algorithms/graph.sora"));
    verify_sora_source(
        "string_search",
        include_str!("../benchmarks/algorithms/string_search.sora"),
    );
}

#[test]
fn test_realworld_http_service_verification() {
    verify_sora_source(
        "http_backend",
        include_str!("../demo-service/realworld_backend.sora"),
    );
}

#[test]
fn test_template_profiles_verification() {
    verify_sora_source("http_template", include_str!("../templates/http/main.sora"));
    verify_sora_source("cli_template", include_str!("../templates/cli/main.sora"));
    verify_sora_source("wasm_template", include_str!("../templates/wasm/main.sora"));
    verify_sora_source(
        "lib_template",
        include_str!("../templates/library/lib.sora"),
    );
}
