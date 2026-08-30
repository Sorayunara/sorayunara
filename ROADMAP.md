# Sorayunara Technical Roadmap & Maturity Matrix

## Feature Maturity Status Legend
- 🟢 **Stable**: Production-ready, fully specified, covered by regression test suites.
- 🟡 **Experimental**: Implemented and executable, undergoing API stabilization and stress testing.
- 🔵 **Preview**: Functional prototype available, design subject to evolution.
- ⚪ **Planned**: Documented in RFCs/specifications, scheduled for upcoming release cycles.

---

## Technical Maturity Matrix

| Component | Status | Verification & Test Coverage |
| :--- | :---: | :--- |
| **Lexer, Tokenizer & Parser** | 🟢 | `tests/grammar_verification_tests.rs`, EBNF spec compliance |
| **Hindley-Milner Type Inference** | 🟢 | `tests/type_system_formal_suite_tests.rs`, bidirectional typing |
| **Borrow Checker & Move Semantics** | 🟡 | `tests/borrow_checker_matrix_tests.rs`, flow-sensitive analysis |
| **Rich Compiler Diagnostics** | 🟢 | `tests/rich_diagnostics_tests.rs`, Rustc-style line gutters |
| **Bytecode Virtual Machine & JIT** | 🟢 | `tests/runtime_tests.rs`, multi-target VM runner |
| **LLVM IR Generation** | 🟡 | `tests/multi_target_backend_tests.rs`, SSA emission |
| **C99 & WebAssembly Backends** | 🟡 | `tests/language_conformance_matrix_tests.rs`, WAT & ANSI C |
| **M:N Work-Stealing Runtime** | 🟡 | `tests/runtime_concurrency_formal_tests.rs`, CSP & Actor mailbox |
| **Standard Library 18 Core Modules** | 🟡 | `tests/standard_library_ecosystem_tests.rs`, std/ |
| **Package Manager & Lockfile Engine** | 🔵 | `tests/package_architecture_tests.rs`, SHA-256 integrity |
| **Language Server Protocol (LSP)** | 🔵 | `tests/lsp_tests.rs`, hover, diagnostics, completion |
| **DAP Debugger & Profiler** | 🔵 | `tests/debugger_profiler_tests.rs`, stack tracing |
| **Self-Hosting Stage 2/3 Chain** | ⚪ | RFC 0001, self-hosted compilation pipeline |
