# Changelog

All notable changes to the **Sorayunara** (`.sora`) programming language toolchain, compiler, standard library, and runtime ecosystem are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Multi-Target Codegen Suite**: Expanded backend emission targets including LLVM IR (`.ll`), ANSI C (`.c`), WebAssembly (`.wat` / `.wasm`), and VM bytecode.
- **Formal Snapshot & Compile-Fail Tests**: Added compile-time failure diagnostics suite verifying borrow checker, Hindley-Milner type inference, and AST parser error reporting.
- **Unified Binary CLI**: Dual executable entrypoints (`sorayunara` and `sora`) supporting all development workflows.

---

## [0.2.2] - 2026-08-25

### Added
- **VS Code Extension Rebranding & Assets**:
  - Official high-resolution light and dark file icon themes for `.sora` files.
  - TextMate grammar highlighting keywords, types, traits, functions, and string interpolations.
  - Language Server Protocol (LSP) auto-trigger on `.sora` file activation.
  - Native Windows `.ico`, Linux MIME `.xml`, and Android MIME icon assets in `assets/`.
- **Linguist Upstream Submission Bundle**:
  - Full bundle in `linguist-submission/` ready for global GitHub recognition.

### Changed
- Re-architected repository structure into clean sub-tiers: `compiler/`, `runtime/`, `std/`, `bootstrap/`, `src/`, `tests/`, `examples/`, `benchmarks/`, and `tools/`.

---

## [0.2.0] - 2026-08-20

### Added
- **Hindley-Milner Type Inference**: Automatic type unification for variables, closures, tuples, and algebraic data types.
- **Flow-Sensitive Borrow Checker**: Ownership tracking, compile-time borrow exclusivity (`&` vs `&mut`), and use-after-move prevention.
- **Lightweight Actor Concurrency**:
  - `spawn async` coroutine task scheduling.
  - Lock-free MPSC channels (`Channel<T>`) for cross-thread message passing.
- **Built-in Developer Tooling**:
  - AST-based code formatter (`sora fmt`).
  - Static analysis linter (`sora lint`).
  - Documentation generator (`sora doc`).
  - Security auditor (`sora audit`).

---

## [0.1.0] - 2026-08-10

### Added
- Initial release of the Sorayunara unified toolchain.
- Lexer, recursive descent parser, and AST definition for `.sora`.
- Three-address code Intermediate Representation (IR) with constant folding and dead code elimination.
- Portable Virtual Machine (VM) and bytecode execution engine.
- Standard library base modules: `std.io`, `std.fs`, `std.net`, `std.math`, `std.json`, `std.time`.
