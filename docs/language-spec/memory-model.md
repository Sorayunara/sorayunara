# 🧠 Sorayunara Formal Language Specification: Memory Model & Unsafe Subsystem

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/memory-model.md`

---

## 1. Abstract Machine & Memory Architecture

Sorayunara programs execute on an abstract machine with three memory zones:

1. **Stack Memory**: Fixed-size, statically managed stack frames holding local scalar primitives, pointers, and values with known compile-time size ($Sized$). Allocation and deallocation cost is $\mathcal{O}(1)$ with zero fragmentation.
2. **Heap Memory**: Dynamically allocated memory via the unified allocator subsystem (Region/Arena, Slab, or Jemalloc). Managed strictly by affine ownership and RAII destructors without tracing garbage collection pauses.
3. **Static / Data Segment**: Holds read-only constants, string literals (`&'static str`), and static variables alive for the program's lifecycle.

---

## 2. Memory Allocators & Region Scopes

Sorayunara supports pluggable memory allocation strategies:

```sora
// Standard Heap Allocation
buffer = Buffer.new(1024)

// Arena / Region Allocation for high-throughput batch operations
region temp_arena = Arena.new(64 * 1024) {
    for i in 0..10_000 {
        node = temp_arena.alloc(Node { value: i })
        process_node(node)
    }
} // Entire 64KB arena wiped in a single O(1) bulk deallocation
```

---

## 3. The `unsafe` Subsystem & Raw Pointers

For operating system drivers, embedded kernels, and high-performance FFI, Sorayunara provides an isolated `unsafe` boundary.

### 3.1 Raw Pointers (`*const T`, `*mut T`)
Raw pointers have no borrow-checking or lifetime guarantees:
- May be `null`.
- May alias freely.
- Dereferencing requires an explicit `unsafe` block.

```sora
address: UInt64 = 0x4000_1000
raw_ptr = address as *mut UInt32

unsafe {
    // Direct MMIO register write
    *raw_ptr = 0x01
    value = *raw_ptr
}
```

### 3.2 Unsafe Invariants
Inside `unsafe` blocks, the developer assumes responsibility for:
1. Ensuring pointers are aligned and non-null prior to dereferencing.
2. Preventing data races on shared mutable memory.
3. Maintaining valid initialized memory for references cast from pointers.
