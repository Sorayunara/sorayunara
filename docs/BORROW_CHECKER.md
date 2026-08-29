# 🛡️ Sorayunara Borrow Checker & Alias Analysis
**Compiler Phase**: `compiler/borrowck/` · **Algorithm**: Non-Lexical Lifetimes (NLL) CFG Flow

---

## 1. The Borrowing Rules
1. Any resource may have **any number of immutable references (`&T`)**, OR
2. Exactly **one mutable reference (`&mut T`)**, but NEVER both concurrently.
3. References must always point to valid, initialized memory within the active CFG path.

---

## 2. Compile-Time Data Race Freedom
Because mutable references are strictly exclusive, cross-thread data races are physically impossible at the type level.
Sending data across actors/fibers requires `Send` ownership transfer (*move*), preventing shared mutable state.
