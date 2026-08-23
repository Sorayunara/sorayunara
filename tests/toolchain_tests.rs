use sorayunara::formatter::format_source;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::test_runner::{TestOptions, TestRunner};

#[test]
fn test_toolchain_compiler_and_semantics() {
    let source = "fn main() -> Int { return 42 }";
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());
}

#[test]
fn test_toolchain_formatter_subsystem() {
    let unformatted = "fn main()->Int{return 42;}";
    let formatted = format_source(unformatted);
    assert!(formatted.is_ok());
}

#[test]
fn test_toolchain_test_and_verification_subsystems() {
    let test_src = r#"
        test "basic math" {
            assert(2 + 2 == 4)
        }
    "#;
    let runner = TestRunner::new(TestOptions {
        coverage: true,
        bench: true,
        fuzz: true,
        verify: true,
        snapshot: true,
    });
    let results = runner.run_source(test_src, "test_suite.sora").unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].passed);
}
