# 📊 Sorayunara Performance Profiling & Flamegraph Analysis
**Tools**: Sampling CPU Profiler, Allocation Tracker, and Chrome Tracing JSON

---

## 1. Profiling Commands
```powershell
sorayunara bench --profile
# Emits profile_trace.json compatible with speedscope.app and Chrome Tracing
```

---

## 2. High-Precision Benchmarks
```sora
benchmark "string concatenation throughput" {
    let mut s = String::new();
    for _ in 0..10_000 {
        s.push_str("data");
    }
}
```
Emits nanosecond-precision timings, throughput (ops/sec), CPU cycles, and cache miss rates.
