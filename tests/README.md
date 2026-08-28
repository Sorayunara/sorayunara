# 🧪 Sorayunara Pipeline-Based Test Architecture

Tests in Sorayunara are organized strictly according to the **Compiler & Runtime Pipeline Stages**. This enables rapid bug localization and isolated regression verification.

---

## 🏛️ Pipeline Test Taxonomy

```
tests/
├── lexer/            ── Tokenization, UTF-8 handling, keywords, literals
├── parser/           ── Pratt precedence, syntax errors, AST constructs
├── typechecker/      ── Hindley-Milner inference, trait bounds, ADTs
├── borrowchecker/    ── Static ownership, move semantics, lifetime validation
├── ir/               ── High-level IR (HIR), Mid-level IR (MIR / CFG)
├── optimizer/        ── Dead code elimination (DCE), inlining, constant folding
├── codegen/
│   ├── llvm/         ── LLVM IR generation, native machine targets
│   ├── wasm/         ── WebAssembly text (.wat) & binary (.wasm) emission
│   └── c/            ── ANSI C transpilation and embedded C targets
├── runtime/          ── Virtual Machine execution, M:N scheduler, memory arena
└── integration/      ── End-to-end full-program compilation and execution
```

---

## 🎯 Bug Localization Mapping

When investigating or reproducing issues:

| Bug Category | Pipeline Stage | Target Test Suite |
|---|---|---|
| Lexing / Token Bug | Tokenizer | `tests/lexer/` |
| Syntax / Grammar Bug | Parser | `tests/parser/` |
| Type Inference / Narrowing Bug | Typechecker | `tests/typechecker/` |
| Ownership / Use-after-move Bug | Borrow Checker | `tests/borrowchecker/` |
| Control Flow / SSA Bug | IR Engine | `tests/ir/` |
| Over-optimization / DCE Bug | Multi-pass Optimizer | `tests/optimizer/` |
| Native Emission Bug | LLVM Backend | `tests/codegen/llvm/` |
| WASM Output Bug | WASM Backend | `tests/codegen/wasm/` |
| C Transpiler Bug | C Backend | `tests/codegen/c/` |
| VM / Stack Underflow Bug | Runtime VM | `tests/runtime/` |
| Multi-module / End-to-end Bug | Full Pipeline | `tests/integration/` |

---

## 🚀 Running Pipeline Tests

```bash
# Run all tests across the entire pipeline
cargo test --all-targets

# Run tests targeting a specific pipeline stage
cargo test --test parser_tests
cargo test --test typechecker_tests
cargo test --test memory_model_tests
cargo test --test optimizer_tests
cargo test --test runtime_tests
cargo test --test multi_target_backend_tests
```
