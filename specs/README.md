# Sorayunara Formal Specifications (`specs/`)

This directory contains the formal semantic and syntactic specifications for the Sorayunara programming language.

## Modules:
- [language/](language/): Syntax, grammar, lexical rules, and expressions.
- [type-system/](type-system/): Hindley-Milner type inference, subtyping, and trait systems.
- [borrow-checker/](borrow-checker/): Static ownership, affine types, lifetime constraints.
- [concurrency/](concurrency/): Actor mailboxes, M:N scheduler, channels, async/await semantics.
- [codegen/](codegen/): ABI layout, LLVM lowering, WASM memory mapping, ANSI C transpilation.
