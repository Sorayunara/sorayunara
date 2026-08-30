# 🧬 Sorayunara Static Type System & Category-Theoretic Inference

**Document Status**: Official Architecture & Systems Reference Standard  
**Theoretical Basis**: System $F_\omega$ + Hindley-Milner Algorithm $\mathcal{W}$ + Affine Type Subtyping  
**Compiler Modules**: [`bootstrap/src/semantics.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/semantics.rs), [`bootstrap/src/monomorphizer.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/monomorphizer.rs)

---

## 1. Type Inference & Unification Rules

Sorayunara employs a bidirectional type checking algorithm with Damas-Milner principal type inference:

$$ \frac{\Gamma \vdash e_1 : \tau_1 \to \tau_2 \quad \Gamma \vdash e_2 : \tau_1}{\Gamma \vdash e_1(e_2) : \tau_2} \quad [\text{T-App}] $$

$$ \frac{\Gamma, x : \tau_1 \vdash e : \tau_2}{\Gamma \vdash \lambda x. e : \tau_1 \to \tau_2} \quad [\text{T-Abs}] $$

---

## 2. Complete Type Hierarchy

```
                                  Top Type / Any
                                        │
        ┌───────────────────────────────┼───────────────────────────────┐
        ▼                               ▼                               ▼
 [Primitive Scalars]             [Compound Types]              [Algebraic Data Types]
 • Int8..Int64, UInt8..UInt64    • Tuples: (A, B, C)           • Enums / Sum Types
 • Float32, Float64              • Arrays: [T; N]              • Structs / Product Types
 • Bool, Char                    • Slices: &[T]                • Option[T] (T?)
 • Byte (u8)                     • Fat Pointers                • Result[T, E] (T!E)
        │                               │                               │
        └───────────────────────────────┼───────────────────────────────┘
                                        ▼
                                [Generics & Traits]
                                • Trait Bounds (T: Trait)
                                • Associated Types (type Item)
                                • Monomorphized Instances
                                        │
                                        ▼
                                   Never (!)
```

---

## 3. Systems-Level Memory & Type Rules

| Construct | Representation | Size / Layout | Systems Invariant |
|---|---|---|---|
| **Primitives** | Scalar Binary | 1, 2, 4, 8 bytes | Strict IEEE-754 / Two's complement |
| **Tuples** | Contiguous Stack | Sum of sizes + padding | Zero pointer indirection |
| **Arrays `[T; N]`** | Flat Buffer | $N \times \text{sizeof}(T)$ | Stack-allocated inline memory |
| **Slices `&[T]`** | Fat Pointer | 16 bytes `(ptr, len)` | Memory-safe bounds checking |
| **Structs** | Named Product | Field alignment / `@repr(C)` | C-ABI compatible layout |
| **Enums** | Tagged Union | `Tag + max(Payload)` | Null-pointer niche optimization |
| **Generics** | Parametric | Monomorphized at compile-time | Zero runtime overhead |
| **`T?` (Option)** | Sum Type | Niche-optimized pointer | No runtime overhead for `&T?` |
| **`T!E` (Result)**| Sum Type | Success/Error union | No hidden exception unwinding |

---

## 4. Formal Type Specification Details

For the complete technical specification of each type rule, refer to:
👉 **[`docs/language-spec/types.md`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/docs/language-spec/types.md)**
