# ⚠️ Sorayunara Error Handling & Resumable Effects
**Model**: `Result[T, E]`, `Option[T]`, and Resumable Algebraic Effects

---

## 1. Unrecoverable vs Recoverable Errors
- **Recoverable Errors**: Represented by the sum type `Result[T, E] = Ok(T) | Err(E)` and the `?` try-propagation operator.
- **Unrecoverable Errors (Panics)**: Trigger immediate stack unwinding or abort, releasing resources via RAII drop guards.

---

## 2. Algebraic Effect Error Handling
Instead of global try-catch blocks that discard stack context, algebraic effects allow resumable handlers:

```sora
effect FailureHandler {
    fn on_timeout(retry_count: U32) -> Action;
}

fn fetch_resource() -> Result[Data, NetError] with [FailureHandler] {
    // Delimited continuation can resume computation after external resolution
}
```
