use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use std::path::Path;

fn verify_sora_module(path: &str, content: &str) {
    let tokens =
        tokenize(content).unwrap_or_else(|e| panic!("Lexing failed for {}: {:?}", path, e));
    let ast = parse(tokens).unwrap_or_else(|e| panic!("Parsing failed for {}: {:?}", path, e));
    if let Err(engine) = check_semantics(&ast) {
        panic!(
            "Semantics failed for {}:\n{}",
            path,
            engine.render_all(path, content)
        );
    }
}

#[test]
fn test_compiler_and_runtime_sora_modules() {
    let modules = [
        (
            "compiler/lexer/token.sora",
            include_str!("../compiler/lexer/token.sora"),
        ),
        (
            "compiler/ast/ast.sora",
            include_str!("../compiler/ast/ast.sora"),
        ),
        (
            "compiler/parser/parser.sora",
            include_str!("../compiler/parser/parser.sora"),
        ),
        (
            "compiler/types/types.sora",
            include_str!("../compiler/types/types.sora"),
        ),
        (
            "compiler/borrow/borrowchecker.sora",
            include_str!("../compiler/borrow/borrowchecker.sora"),
        ),
        (
            "compiler/ir/mir.sora",
            include_str!("../compiler/ir/mir.sora"),
        ),
        (
            "compiler/optimizer/optimizer.sora",
            include_str!("../compiler/optimizer/optimizer.sora"),
        ),
        (
            "compiler/codegen/llvm.sora",
            include_str!("../compiler/codegen/llvm.sora"),
        ),
        (
            "compiler/driver/pipeline.sora",
            include_str!("../compiler/driver/pipeline.sora"),
        ),
        (
            "runtime/allocator/arena.sora",
            include_str!("../runtime/allocator/arena.sora"),
        ),
        (
            "runtime/scheduler/work_stealing.sora",
            include_str!("../runtime/scheduler/work_stealing.sora"),
        ),
        (
            "runtime/async/future.sora",
            include_str!("../runtime/async/future.sora"),
        ),
        (
            "runtime/actor/mailbox.sora",
            include_str!("../runtime/actor/mailbox.sora"),
        ),
        (
            "runtime/channel/mpsc.sora",
            include_str!("../runtime/channel/mpsc.sora"),
        ),
    ];

    for (path, content) in modules {
        verify_sora_module(path, content);
    }
}

#[test]
fn test_specs_hierarchy_exists() {
    let specs = [
        "specs/grammar/grammar.ebnf",
        "specs/type-system/inference.md",
        "specs/ownership/borrowing.md",
        "specs/concurrency/async.md",
        "specs/memory/ABI.md",
        "specs/modules/module-system.md",
        "specs/ffi/c.md",
    ];

    for spec in specs {
        assert!(Path::new(spec).exists(), "Missing required spec: {}", spec);
    }
}
