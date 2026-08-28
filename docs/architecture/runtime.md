# 🚀 Sorayunara Runtime Architecture

The Sorayunara Runtime Engine provides high-throughput, low-latency execution for both compiled native binaries and standalone bytecode execution in the Sorayunara Virtual Machine (VM).

---

## 🏛️ Core Runtime Subsystems

### 1. Memory Subsystem (`runtime/memory/`)
- **Arena & Region Allocator**: Fast bump allocation for request-scoped workloads.
- **Reference Counting**: Deterministic reference counting (`Rc` / `Arc`) for shared resources.
- **Zero-GC Architecture**: No garbage collection pauses or background GC thread overhead.

### 2. Concurrency & M:N Task Scheduler (`runtime/concurrency/`)
- **Work-Stealing Scheduler**: Distributes lightweight coroutine tasks across available CPU worker threads.
- **Micro-Tasks (`spawn` / `await`)**: Sub-microsecond task creation with minimal stack footprint.

### 3. Actor Engine (`runtime/actors/`)
- Isolated mailboxes with lock-free queues.
- Asynchronous message passing with fault-tolerant supervision hierarchies.

### 4. CSP Channels (`runtime/channels/`)
- Synchronous (unbuffered) and asynchronous (buffered) multi-producer multi-consumer (MPMC) typed channels.

### 5. FFI Bridge (`runtime/ffi/`)
- Direct C-ABI bridge providing zero-overhead interop with OS syscalls and native C libraries.
