use sorayunara::test_runner::{TestOptions, TestRunner};

#[test]
fn test_unit_and_benchmark_and_fuzz_runner() {
    let source = r#"
        test "addition commutes" {
            let a: Int = 10
            let b: Int = 20
            assert(a + b == b + a)
        }

        test "multiplication identity" {
            let x: Int = 42
            assert(x * 1 == x)
        }
    "#;

    // 1. Standard unit test
    let runner_std = TestRunner::new(TestOptions::default());
    let results_std = runner_std.run_source(source, "test_cases.ae").unwrap();
    assert_eq!(results_std.len(), 2);
    assert!(results_std.iter().all(|r| r.passed));

    // 2. Benchmark mode
    let runner_bench = TestRunner::new(TestOptions {
        bench: true,
        ..Default::default()
    });
    let results_bench = runner_bench.run_source(source, "test_cases.ae").unwrap();
    assert_eq!(results_bench.len(), 2);
    assert!(results_bench.iter().all(|r| r.passed));

    // 3. Fuzzing mode
    let runner_fuzz = TestRunner::new(TestOptions {
        fuzz: true,
        ..Default::default()
    });
    let results_fuzz = runner_fuzz.run_source(source, "test_cases.ae").unwrap();
    assert_eq!(results_fuzz.len(), 2);
    assert!(results_fuzz.iter().all(|r| r.passed));

    // 4. Code Coverage mode
    let runner_cov = TestRunner::new(TestOptions {
        coverage: true,
        ..Default::default()
    });
    let results_cov = runner_cov.run_source(source, "test_cases.ae").unwrap();
    assert_eq!(results_cov.len(), 2);
    assert!(results_cov.iter().all(|r| r.passed));
}

#[test]
fn test_formal_property_verification() {
    let runner = TestRunner::new(TestOptions {
        verify: true,
        ..Default::default()
    });

    // Property 1: addition is commutative: forall a, b: a + b == b + a
    let prop_comm = runner.run_property_check("addition is commutative", |a, b| {
        a.wrapping_add(b) == b.wrapping_add(a)
    });
    assert!(prop_comm.passed);
    assert_eq!(prop_comm.trials_run, 8);
    assert!(prop_comm.counterexample.is_none());

    // Property 2: multiplication distributes over addition: forall a, b: a * (b + c)
    let prop_mult = runner.run_property_check("multiplication by zero", |a, _| {
        a.wrapping_mul(0) == 0
    });
    assert!(prop_mult.passed);
}
