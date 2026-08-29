# 📦 Sorayunara Standard Library Architecture (`std.*`)
**Standard Library Organization**: Modular, Zero-Overhead, Headless Ready

---

## 1. Core Modules Overview
- `std.core`: Primitives, tuples, traits (`Clone`, `Debug`, `Default`, `Display`, `Hash`, `Eq`, `Ord`).
- `std.collections`: `Vector[T]`, `HashMap[K, V]`, `HashSet[T]`, `BTreeMap[K, V]`, `RingBuffer[T]`.
- `std.io`: Stdio, buffered readers/writers, binary serialization, paths.
- `std.fs`: Filesystem operations, asynchronous file streaming, directory traversal.
- `std.net`: TCP, UDP, TLS 1.3 socket streams, HTTP/1.1 and HTTP/2 clients/servers.
- `std.async`: Fibers, channels, task executors, timers, select loops.
- `std.math`: Pure arithmetic, trigonometry, geometry, linear algebra, SIMD vector primitives.
- `std.testing`: Native assertion macros, benchmark harnesses, test runner utilities.
- `std.windows`: Service Control Manager, Event Viewer, Registry, DPAPI, Named Pipes.
