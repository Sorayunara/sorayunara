# ⚠️ Sorayunara Formal Language Specification: Error Handling Architecture

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/error-handling.md`

---

## 1. Dual Error Model Philosophy

Sorayunara establishes a strict boundary between two classes of errors:

1. **Recoverable Errors**: Expected failure modes (file not found, network timeout, invalid user input). Handled explicitly via `Result[T, E]` (`T!E`) and `Option[T]` (`T?`). **Zero overhead, no hidden exception unwinding**.
2. **Unrecoverable Errors (Panics / Faults)**: Invariant violations, contract violations, out of memory, or assert failures. Trigger a localized task abort or program panic.

---

## 2. Recoverable Errors via `Result[T, E]` (`T!E`)

### 2.1 The `Result` Type Definition
```sora
enum Result[T, E] {
    Ok(T),
    Err(E),
}
```

### 2.2 Question Mark Operator (`?`)
The `?` operator unwraps `Ok(v)` or immediately returns from the enclosing function with `Err(e.into())`:

```sora
fn read_port_from_file(path: String) -> Int!AppError {
    content = fs.read_to_string(path)? // Returns Err if fs fails
    port = content.trim().parse_int()?  // Returns Err if parse fails
    Ok(port)
}
```

---

## 3. Option Coalescing & Safe Navigation

```sora
// Null-coalescing with default fallback
host = config.get("host") ?? "localhost"

// Safe navigation chaining
zip_code = user?.address?.zip_code
```

---

## 4. Unrecoverable Panics & Guarding

### 4.1 Panic Invocations
```sora
fn divide(a: Int, b: Int) -> Int {
    if b == 0 {
        panic("Division by zero in arithmetic kernel")
    }
    a / b
}
```

### 4.2 Assertions
- `assert(condition)`: Verified at runtime; panics on failure.
- `assert_eq(actual, expected)`: Emits formatted mismatch message.
- `comptime assert(...)`: Evaluated at compile time; stops compilation with a diagnostic error if false.

---

## 5. Structured Error Types & Trait `Error`

All standard error types implement the `Error` trait:

```sora
trait Error: Display + Debug {
    fn source(self: &Self) -> &dyn Error? => None
}
```
