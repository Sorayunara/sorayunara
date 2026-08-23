use sorayunara::benchmark_suite::BenchmarkSuite;

#[test]
fn test_benchmark_suite_categories_and_table() {
    let results = BenchmarkSuite::run_all();
    assert_eq!(results.len(), 8);

    let categories: Vec<&str> = results.iter().map(|r| r.category).collect();
    assert!(categories.contains(&"Startup Time"));
    assert!(categories.contains(&"Memory Usage (Base RSS)"));
    assert!(categories.contains(&"Compilation Speed"));
    assert!(categories.contains(&"HTTP Server Throughput"));
    assert!(categories.contains(&"JSON Parsing Throughput"));
    assert!(categories.contains(&"Database Query Throughput"));
    assert!(categories.contains(&"CPU Computation (Mandelbrot)"));
    assert!(categories.contains(&"Concurrency (100k Tasks)"));

    let table = BenchmarkSuite::render_markdown_table(&results);
    assert!(table.contains("| Aether | Rust | Go | C++ | Zig |"));
    assert!(table.contains("Startup Time"));
    assert!(table.contains("HTTP Server Throughput"));
}
