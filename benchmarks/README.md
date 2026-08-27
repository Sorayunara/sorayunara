# 📊 Sorayunara Official Benchmarks

This directory contains standalone, reproducible performance benchmarks for the **Sorayunara** (`.sora`) programming language.

## Benchmark Programs

| Benchmark | File | Description | Focus Metric |
| :--- | :--- | :--- | :--- |
| **Fibonacci** | [`fibonacci.sora`](fibonacci.sora) | Iterative & recursive sequence calculation | Call overhead & loop throughput |
| **Matrix Mult** | [`matrix.sora`](matrix.sora) | Triple-nested matrix computation | CPU branch prediction & ALU optimization |
| **Prime Sieve** | [`prime_sieve.sora`](prime_sieve.sora) | Prime number factor determination | Integer arithmetic & memory caching |

## How to Run

```bash
# Run via Sorayunara toolchain
cargo run -- run benchmarks/fibonacci.sora
cargo run -- run benchmarks/matrix.sora
cargo run -- run benchmarks/prime_sieve.sora

# Or execute full benchmark suite
cargo run -- bench
```
