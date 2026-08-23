use sorayunara::test_runner::{TestOptions, TestRunner};

#[test]
fn test_builtin_test_syntax_passing() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }

        test "addition works" {
            let res: Int = add(2, 3)
            assert(res == 5)
        }

        test "subtraction works" {
            assert(add(10, -5) == 5)
        }
    "#;

    let runner = TestRunner::new(TestOptions::default());
    let results = runner.run_source(source, "test_file.ae").unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.passed));
    assert!(results.iter().any(|r| r.name == "addition works"));
    assert!(results.iter().any(|r| r.name == "subtraction works"));
}

#[test]
fn test_builtin_test_syntax_failing_assertion() {
    let source = r#"
        test "failing condition" {
            assert(2 + 2 == 5)
        }
    "#;

    let runner = TestRunner::new(TestOptions::default());
    let results = runner.run_source(source, "test_fail.ae").unwrap();

    assert_eq!(results.len(), 1);
    assert!(!results[0].passed);
    assert!(results[0].error_message.as_ref().unwrap().contains("Assertion Failed"));
}

#[test]
fn test_builtin_test_options_bench_and_coverage() {
    let source = r#"
        fn multiply(a: Int, b: Int) -> Int {
            return a * b
        }

        test "benchmark multiply" {
            assert(multiply(6, 7) == 42)
        }
    "#;

    let options = TestOptions {
        coverage: true,
        bench: true,
        fuzz: false,
        ..Default::default()
    };
    let runner = TestRunner::new(options);
    let results = runner.run_source(source, "test_bench.ae").unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].passed);
}
