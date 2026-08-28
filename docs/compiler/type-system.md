# 🛡️ Compiler: Type System & Inference (`compiler/typeck/`)

The type checking subsystem implements bidirectional Hindley-Milner (HM) type inference with trait resolution.

---

## Core Algorithms
1. **Algorithm W / Bidirectional Inference**: Infers concrete types from usage contexts without requiring explicit variable type annotations.
2. **Type Narrowing**: Refines union types across `if x is Type` and `match` branches.
3. **Exhaustiveness Checking**: Enforces full pattern match coverage on ADT enums.
