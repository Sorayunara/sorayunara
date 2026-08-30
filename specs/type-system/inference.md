# Sorayunara Type System & Hindley-Milner Inference Specification

## 1. Type Inference Rules
- Bidirectional typing: Expression AST nodes infer principal types via Algorithm W.
- Unification: Structural equality between type terms with occurs-check.

## 2. Generics & Monomorphization
- Parametric types: `T`, `Option<T>`, `Result<T, E>`.
- Monomorphization: Concrete instantiation at compile time with zero runtime overhead.
