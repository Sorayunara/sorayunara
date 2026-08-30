pub struct BenchmarkStats {
    pub name: String,
    pub iterations: usize,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
}

impl BenchmarkStats {
    pub fn compute(name: &str, mut samples_ns: Vec<f64>) -> Self {
        samples_ns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = samples_ns.len();
        let sum: f64 = samples_ns.iter().sum();
        let mean_ns = sum / (n as f64);
        let median_ns = samples_ns[n / 2];
        let p95_ns = samples_ns[((n as f64) * 0.95) as usize];
        let p99_ns = samples_ns[((n as f64) * 0.99) as usize];

        Self {
            name: name.to_string(),
            iterations: n,
            mean_ns,
            median_ns,
            p95_ns,
            p99_ns,
        }
    }
}
