# ⚙️ Sorayunara Compiler Architecture & Internal Pipeline
**Compiler Crates**: `compiler/` · `bootstrap/` · **Language**: Pure Rust (2024 Edition)

---

## 1. Multi-Stage Translation Pipeline

```
 Source (.sora) ────► Lexer ────► Tokens ────► Parser ────► AST
                                                             │
                                                             ▼
                                                    Name Resolver
                                                             │
                                                             ▼
                                                      Type Checker
                                                             │
                                                             ▼
                                                     Borrow Checker
                                                             │
                                                             ▼
                                                        Sora-HIR
                                                             │
                                                             ▼
                                                        Sora-MIR
                                                             │
                                                     Optimizer (LTO)
                                                             │
                                                             ▼
                                                    Codegen Engine
                                                    ├── LLVM IR
                                                    ├── WASM
                                                    ├── C99 Transpiler
                                                    └── Native MSVC/ELF
```

---

## 2. Query-Based Incremental Compilation
The compiler employs a Salsa-inspired dependency graph query engine where cached phase outputs are invalidated only when their AST dependencies change.
