# 🔬 Sorayunara Intermediate Representations (HIR & MIR)
**Specification**: High-Level (HIR) and Control-Flow Mid-Level (MIR) SSA Form

---

## 1. High-Level Intermediate Representation (Sora-HIR)
- Preserves high-level constructs: pattern matching, algebraic effects, trait bounds, and HKTs.
- Target of bidirectional type inference and trait resolution.

---

## 2. Mid-Level Intermediate Representation (Sora-MIR)
- **Control Flow Graph (CFG)**: Composed of Basic Blocks (`bb0`, `bb1`, ...) terminated by jumps, branches, or returns.
- **Explicit Drops**: Clean deallocation instructions (`StorageDead`, `Drop(x)`) inserted by the borrow checker.
- **SSA Register Form**: Instructions operate on virtual registers (`_1`, `_2`, `_3`) for dead-code elimination, constant folding, and vectorization passes.
