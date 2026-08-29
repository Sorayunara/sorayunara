# ⚡ Sorayunara Optimization & Performance Guidelines
**Engine Tuning**: LLVM Optimizations, SIMD Auto-Vectorization & Zero-Cost Drops

---

## 1. Zero-Cost Abstractions
- **Traits & Generics**: Monomorphized at compile-time with zero virtual dispatch penalty.
- **Pattern Matching**: Lowered into optimal jump tables and bitwise discriminant checks.
- **Affine Moves**: Registers are moved directly via assembly `mov` with 0 heap allocation.

---

## 2. Link-Time Optimization (LTO)
Compiling with `sorayunara build --release --lto` enables cross-module dead-code elimination, aggressive inlining, and global register allocation across the entire application binary.
