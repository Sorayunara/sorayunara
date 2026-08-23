use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::profiler::Profiler;

#[test]
fn test_profiler_cpu_and_table_rendering() {
    let source = r#"
        fn compute_heavy(x: Int) -> Int {
            return x * x + 10
        }

        fn main() -> Int {
            let res: Int = compute_heavy(20)
            return res
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let ir = compile_to_ir(&ast);

    let profiler = Profiler::new(ir);
    let report = profiler.run_full_profile().unwrap();

    // 1. Verify CPU profiling
    assert_eq!(report.functions.len(), 2);
    let total_pct: f64 = report.functions.iter().map(|f| f.time_percentage).sum();
    assert!((total_pct - 100.0).abs() < 0.1);

    // 2. Verify Table format
    let table = Profiler::render_table(&report);
    assert!(table.contains("Function              Time       Calls"));
    assert!(table.contains("Memory & Allocation Analysis"));
    assert!(table.contains("Async & Scheduler Analysis"));
    assert!(table.contains("Flamegraph"));

    // 3. Verify Memory and Allocation profiling
    assert!(report.memory.total_allocations > 0);
    assert!(report.memory.total_bytes_allocated > 0);
    assert!(report.memory.peak_memory_bytes >= report.memory.total_bytes_allocated);

    // 4. Verify Flamegraph
    assert!(!report.flamegraph.is_empty());
}
