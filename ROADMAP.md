# 🗺️ Sorayunara Development Roadmap

This document outlines the strategic milestones and technical roadmap for the **Sorayunara** (`.sora`) programming language ecosystem.

---

## 📍 Current Status: Milestone 2 (v0.2.x)

Sorayunara is currently in active v0.2.x development with a fully functional Rust-based bootstrap compiler, typechecker, borrow checker, VM runtime, multiple codegen backends, and tooling.

---

## 🎯 Strategic Roadmap

```
[Phase 1: Foundations] ➔ [Phase 2: Tooling & Types] ➔ [Phase 3: Native Backends] ➔ [Phase 4: Pure Self-Hosting]
     (COMPLETED)               (CURRENT: v0.2.x)             (IN PROGRESS)                   (PLANNED: v1.0)
```

---

### Phase 1: Core Foundations & Grammar ✅
- [x] Syntax specification with mathematical, type-safe primitives (`.sora`).
- [x] Lexer, recursive descent parser, and Abstract Syntax Tree (AST).
- [x] Three-address intermediate representation (Quad IR).
- [x] Stack-based virtual machine with memory management.
- [x] Basic Standard Library modules (`std.io`, `std.math`, `std.fs`, `std.json`).

---

### Phase 2: Static Safety, Concurrency & Tooling ✅ (v0.2.x)
- [x] **Hindley-Milner Type Inference**: Automatic type unification and parametric polymorphism.
- [x] **Flow-Sensitive Borrow Checker**: Ownership transfer, immutable/mutable borrow rules, no use-after-move.
- [x] **Lightweight Concurrency**: Coroutine scheduler (`spawn async`), MPSC channels (`Channel<T>`).
- [x] **Integrated CLI Toolchain**:
  - Code formatter (`sora fmt`)
  - Linter (`sora lint`)
  - Documentation generator (`sora doc`)
  - Dependency manager (`sora add` / `sora remove` / `sora publish`)
  - Security auditor (`sora audit`)
- [x] **VS Code Extension**: Full LSP integration, syntax highlighting, and custom file icons.

---

### Phase 3: Multi-Target Native Codegen 🟡 (In Progress)
- [x] **LLVM IR Backend**: Direct `.ll` native emission for x86_64 / ARM64 targets.
- [x] **ANSI C Backend**: Transpilation to standard C99 for microcontroller and embedded systems.
- [x] **WebAssembly Backend**: `.wat` / `.wasm` text & binary generation for browser execution.
- [ ] **Direct ELF/Mach-O Linker**: Standalone native binary linker without requiring external clang/gcc.
- [ ] **SIMD & Tensor Intrinsics**: Auto-vectorized matrix operations in `std.simd`.
- [ ] **Cross-compilation Matrix**: Pre-built single-command cross-targets (`sora build --target aarch64-unknown-linux-musl`).

---

### Phase 4: Self-Hosting & 1.0 Production Readiness 🔮
- [x] Initial self-hosting compiler sources in `src/*.sora` (`ast.sora`, `parser.sora`, `codegen.sora`, `main.sora`).
- [ ] Stage 1 -> Stage 2 self-compilation verification.
- [ ] Complete replacement of Rust bootstrap with standalone Sorayunara native compiler binary (`sorac`).
- [ ] Package Registry Web Portal (`packages.sorayunara.org`).
- [ ] Linguistic standardization submission to upstream GitHub Linguist once the community reaches 2,000 public repositories.
- [ ] Long Term Support (LTS) 1.0.0 release candidate.

---

## 🤝 How to Contribute

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines, development setup, and code review standards.
