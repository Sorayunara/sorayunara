# ⚡ Sorayunara Formal Runtime Specification & Systems Model

**Document Version**: 1.0.0  
**Specification Status**: Normative Runtime Standard  
**File Identifier**: `docs/language-spec/runtime-specification.md`

---

## 1. Unified Runtime Architecture

```
                                  Sorayunara Runtime
                                          │
        ┌─────────────────────────────────┼─────────────────────────────────┐
        ▼                                 ▼                                 ▼
[Scheduler & Executor]           [Concurrency Triad]               [Memory & Allocators]
• Work-Stealing M:N Threads      • CSP Channels (MPMC/Buffered)    • Region & Arena Allocator
• Cooperative Green Tasks        • Actor Mailbox Runtime           • Slab / Bump Allocator
• Task Stealing Deque            • Structured Scope Joining        • Zero-Fragmentation Heap
```

---

## 2. Subsystem Specifications

### 2.1 Scheduler & Task Executor (`runtime/scheduler`)
- **Model**: M:N cooperative hybrid threading.
- **Worker Stealing**: Workers yield to global/local FIFO queues upon task blockage (`await`, `recv`, `acquire`).

### 2.2 CSP Typed Channels (`runtime/channel`)
- **Primitives**: Unbounded & Bounded Channels with lock-free atomic rings and condvar backpressure.
- **Invariants**: Sending across channels transfers affine ownership (*Move*), preventing data races.

### 2.3 Actor Model Runtime (`runtime/actor`)
- **Supervision**: Isolated actor mailboxes processing sequential messages with zero shared mutable state.

### 2.4 Region / Arena Allocator (`runtime/allocator`)
- **Operation**: Linear bump allocation with alignment guarantees $\mathcal{O}(1)$ and bulk reclamation $\mathcal{O}(1)$.

---

## 3. Benchmarks & Verification

- **Formal Tests**: [`tests/runtime_concurrency_formal_tests.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/tests/runtime_concurrency_formal_tests.rs)
- **Benchmark Suite**: [`benchmarks/runtime_concurrency_benchmarks.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/benchmarks/runtime_concurrency_benchmarks.rs)
