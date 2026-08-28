# 🔒 Compiler: Borrow Checker Subsystem (`compiler/borrowck/`)

The borrow checker guarantees compile-time memory safety, preventing use-after-free, double-free, and data races without garbage collection overhead.

---

## Core Rules
1. **Single Owner**: Each resource has exactly one owner at any point in time.
2. **Move Semantics**: Passing a non-copy value transfers ownership (`move`).
3. **Aliasing XOR Mutability**: Any number of immutable references (`&T`) OR exactly one mutable reference (`&mut T`).
4. **Non-Lexical Lifetimes (NLL)**: References expire at their last point of use rather than at scope end.
