# 🧬 Sorayunara Static Type System & Category-Theoretic Inference
**Type Framework**: System $F_\omega$ + Hindley-Milner + Higher-Kinded Types (HKT)

---

## 1. Type Inference & Unification
Sorayunara employs a bidirectional type checking algorithm with full Damas-Milner type inference. Local variable bindings do not require manual annotations unless resolving ambiguous generic constraints.

$$ \frac{\Gamma \vdash e_1 : \tau_1 \to \tau_2 \quad \Gamma \vdash e_2 : \tau_1}{\Gamma \vdash e_1(e_2) : \tau_2} $$

---

## 2. Higher-Kinded Types (HKT)
Traits can abstract over type constructors (kinds of shape `* -> *`):

```sora
pub trait Functor[F[_]] {
    fn fmap[A, B](self: F[A], transform: fn(A) -> B) -> F[B];
}

pub trait Monad[M[_]] : Functor[M] {
    fn pure[A](val: A) -> M[A];
    fn flat_map[A, B](self: M[A], transform: fn(A) -> M[B]) -> M[B];
}
```

---

## 3. Algebraic Data Types (ADT) & Exhaustiveness
Every `enum` variant in Sorayunara is statically checked for complete exhaustiveness during pattern matching.
Missing branches emit compilation error `E0312: Non-exhaustive pattern match`.
