# 🧠 Sorayunara Memory Model & Ownership Semantics
**Engine**: Zero-GC Deterministic Affine-Linear Resource Management

---

## 1. Core Principles
1. **Zero Garbage Collector (GC)**: No stop-the-world pauses, no tracing overhead.
2. **Deterministic Drop**: Resources (memory buffers, socket descriptors, mutex locks) are deallocated immediately upon exiting their lexical scope.
3. **Move Semantics by Default**: Assigning or passing a non-`Copy` variable transfers ownership.

---

## 2. Linear vs Affine Types
- **Linear Type**: Must be used *exactly once* (cannot be implicitly dropped or duplicated).
- **Affine Type**: Can be used *at most once* (automatically dropped at end of scope if unconsumed).

```sora
// Linear socket ensures explicit graceful shutdown or transfer
fn process_connection(linear socket: SocketSession) {
    socket.send_handshake();
    socket.close(); // Consumes linear token
}
```

---

## 3. Region-Based Lifetimes
References (`&T` and `&mut T`) carry compile-time region tokens that guarantee no reference outlives the owner.
Lifetimes are completely erased during Sora-MIR to LLVM lowering, incurring **0 runtime cost**.
