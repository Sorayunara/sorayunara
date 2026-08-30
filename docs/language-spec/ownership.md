# 🔐 Sorayunara Formal Language Specification: Ownership & Move Semantics

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/ownership.md`

---

## 1. Ownership Axioms

Sorayunara manages memory statically at compile-time without a runtime Garbage Collector (GC) via three fundamental ownership axioms:

1. **Every resource in Sorayunara has a single owner variable.**
2. **There can only be ONE owner at any given point in program execution.**
3. **When the owner variable goes out of scope, the resource is automatically deallocated (RAII).**

---

## 2. Value Move Semantics

### 2.1 The Move Operation
When a non-`Copy` value is assigned to another binding, passed by value into a function, or returned from a scope, ownership is **moved**. The source binding becomes invalid:

```sora
s1 = String.from("Sorayunara")
s2 = s1 // Ownership transferred (moved) to s2

// Compile Error: E0382 - Use of moved value `s1`
// println(s1) 

println(s2) // Valid
```

### 2.2 Explicit `move` Keyword
The `move` keyword forces capture of variables by value into closures, actors, or concurrency tasks:

```sora
buffer = Buffer.allocate(4096)
task = spawn move {
    // buffer ownership moved entirely into the concurrent worker
    process_buffer(buffer)
}
```

---

## 3. The `Copy` vs `Clone` Distinction

### 3.1 `Copy` Semantics
Types whose data resides entirely on the stack (e.g., `Int`, `Float`, `Bool`, `Char`, fixed-size primitive arrays) implement `Copy`. Assignment performs a bitwise copy (memcpy) without invalidating the source:

```sora
x = 42
y = x // x is copied, not moved
println(x) // Valid: 42
```

### 3.2 `Clone` Semantics
Types managing heap allocations or external handles (`String`, `[T]`, `Map[K, V]`) implement `Clone`. Duplication requires an explicit `.clone()` call to prevent accidental deep copies:

```sora
s1 = String.from("Sorayunara")
s2 = s1.clone() // Deep copy
println(s1) // Valid
```

---

## 4. RAII & Automatic Resource Cleanup

When an owner leaves scope, the compiler synthesizes a call to its destructor (`Drop` trait):

```sora
{
    socket = TcpStream.connect("127.0.0.1:8080")?
    socket.write(b"PING")
} // socket exits scope -> automatic socket close & kernel handle release
```
