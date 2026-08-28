# 🌌 Sorayunara Master Engineering Roadmap & Architecture Matrix

This document defines the official 10-pillar architecture and phase milestones for the **Sorayunara Programming Language Project**.

---

## 🏛️ The 10 Pillars of Sorayunara

```
                                  SORAYUNARA TOOLCHAIN
                                           │
  ┌───────────────┬────────────────────────┼────────────────────────┬───────────────┐
  ▼               ▼                        ▼                        ▼               ▼
1. Compiler     2. Runtime               3. Stdlib                4. Tooling      5. Package
   Pipeline        M:N Actor Scheduler      37 Pure Modules          LSP, Fmt,       Registry,
   Lexer..Codegen  Allocators & VM          IO, Net, Crypto          Lint, REPL      Resolver, Lock
  │               │                        │                        │               │
  └───────────────┼────────────────────────┼────────────────────────┼───────────────┘
                  ▼                        ▼                        ▼
               6. Tests                 7. Platforms             8. Benchmarks
                  Compiler Stages          x86_64, ARM64,           Reproducible
                  & Integration            RISC-V, WASM, MCU        Workloads
```

---

## 1. ⚙️ Complete Compiler Pipeline (`compiler/`)

```
.sora Source Code
      │
      ▼
   Lexer               (compiler/lexer/)
      │
      ▼
   Parser & AST        (compiler/parser/, compiler/ast/, compiler/syntax/)
      │
      ▼
   Name Resolution     (compiler/resolver/)
      │
      ▼
   Type Checking       (compiler/typeck/)
      │
      ▼
   HM Type Inference   (compiler/inference/)
      │
      ▼
   Borrow & Ownership  (compiler/borrowck/, compiler/ownership/)
      │
      ▼
   Control Flow (CFG)  (compiler/hir/, compiler/mir/)
      │
      ▼
   Multi-Pass Optimizer(compiler/optimizer/)
      │
      ▼
   Multi-Target Codegen(compiler/codegen/)
      ├── LLVM Native IR (.ll)
      ├── ANSI C99 (.c)
      ├── WebAssembly Text (.wat) & Binary (.wasm)
      └── Bytecode VM Engine
```

---

## 2. 📦 Comprehensive Standard Library (`std/`)

All modules in `std/` are 100% pure Sorayunara or safe C-ABI wrappers:

| Subsystem | Modules |
|---|---|
| **Core & Memory** | `std/alloc/`, `std/collections/`, `std/strings/`, `std/reflection/`, `std/ffi/` |
| **I/O & OS** | `std/io/`, `std/fs/`, `std/process/`, `std/time/`, `std/logging/` |
| **Networking & Web** | `std/net/`, `std/http/`, `std/tcp/`, `std/udp/`, `std/websocket/` |
| **Concurrency & Async** | `std/async/`, `std/concurrency/`, `std/channel/`, `std/sync/`, `std/atomics/`, `std/thread/` |
| **Data & Encodings** | `std/encoding/`, `std/json/`, `std/crypto/`, `std/regex/`, `std/compression/`, `std/database/` |
| **Quality & Assurance**| `std/testing/` |

---

## 3. ⚡ Dedicated Systems Runtime (`runtime/`)

- **Memory**: Lock-free bump allocators, arena allocators, thread-local heaps, zero-pause ownership cleanup.
- **Concurrency Scheduler**: Work-stealing M:N green thread scheduler running thousands of micro-tasks across CPU cores.
- **Actor & Channels**: CSP bounded and unbounded ring-buffer channels with non-blocking send/recv.
- **Platform Abstraction**: OS-level syscall multiplexer for Linux (`epoll`), Windows (`IOCP`), macOS (`kqueue`), and WebAssembly (`WASI`).

---

## 4. 🛠️ Developer Tooling & Language Server Protocol

- **Command Line**: `sora` (single-binary orchestrator for new, build, run, test, check, fmt, lint, doc, bench, doctor).
- **Language Server (`sora-lsp`)**: Complete LSP 3.17 server providing diagnostics, auto-completion, hover signatures, jump-to-definition, and rename refactoring.
- **Editor Extensions**: First-class support for VS Code, Neovim, Emacs, and JetBrains.

---

## 5. 🧪 Pipeline-Mapped Test Suite (`tests/`)

Tests are categorized strictly by compiler pipeline stage:
`tests/lexer/`, `tests/parser/`, `tests/typecheck/`, `tests/inference/`, `tests/borrowck/`, `tests/ownership/`, `tests/generics/`, `tests/traits/`, `tests/async/`, `tests/actors/`, `tests/concurrency/`, `tests/optimization/`, `tests/codegen/` (`llvm/`, `wasm/`, `c/`), `tests/runtime/`, `tests/std/`, `tests/integration/`.

---

## 6. 📦 Package Manager & Lockfile Engine (`package/`)

- Declarative project manifests (`sora.toml`).
- Cryptographic SHA-256 lockfile verification (`sora.lock`).
- Zero-drift semantic version resolution (SemVer 2.0.0).

---

## 7. 🎯 Self-Hosting Roadmap

- **Stage 0**: Bootstrap compiler written in pure Rust (edition 2024, zero external dependencies).
- **Stage 1**: Sorayunara compiler written in `.sora` and compiled by Stage 0.
- **Stage 2**: Sorayunara compiler compiled by Stage 1 (`sora compile sora.sora`).
- **Stage 3**: 100% self-hosted, independent compiler toolchain.

---

## 📈 Sizing & Growth Milestones

- **Phase 1**: Core Language + Pratt Parser + AST + Diagnostics (**Completed ✅**)
- **Phase 2**: Hindley-Milner Typeck + Borrowck + VM + src/ Entry Points (**Completed ✅**)
- **Phase 3**: Multi-Target Codegen (LLVM, C99, WASM) + Runtime Scheduler + LSP Engine (**In Progress 🚀**)
- **Phase 4**: Expanded Standard Library (37 modules) + 100+ Real Pipeline Tests + Package Manager
- **Phase 5**: Self-Hosting Bootstrap + Full Platform Matrix
