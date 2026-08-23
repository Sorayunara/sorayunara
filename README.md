<p align="center">
  <img src="assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">🌌 Sorayunara Programming Language (<code>.sora</code>)</h1>

<p align="center">
  <strong>Fast, Safe, Elegant, and Expressive Language for Systems, Backend, Embedded, & AI/ML.</strong>
</p>

<p align="center">
  <a href="https://github.com/Sorayunara/sorayunara/actions"><img src="https://img.shields.io/badge/build-passing-brightgreen.svg" alt="Build Status"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="https://github.com/Sorayunara/sorayunara"><img src="https://img.shields.io/badge/extension-.sora-purple.svg" alt="Extension"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22"><img src="https://img.shields.io/badge/good%20first%20issues-welcome-7057ff.svg" alt="Good First Issues"></a>
</p>

---

## 🏛️ Architecture & Self-Hosting Multi-Arch Pipeline

```
                    SORAYUNARA
                       .sora
                         │
                         ▼
              ┌─────────────────────┐
              │ Sorayunara Compiler │
              │   written in Sora   │
              └──────────┬──────────┘
                         │
                         ▼
                    Sora Native IR
                         │
                         ▼
              Direct Machine Code
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
            x86-64      ARM64      RISC-V
              │          │          │
              ▼          ▼          ▼
            Linux      Windows     Linux
```

---

## 🚀 Key Features

- **Fast**: Direct native code generation via LLVM IR & WebAssembly, zero-cost generic monomorphization, multi-pass IR optimizer (constant folding, dead code elimination, peephole optimizations).
- **Safe**: HM static type inference, flow-sensitive borrow checking, three explicit memory modes (Managed GC, Owned move semantics, Unsafe raw pointer blocks), integer overflow prevention, and capability sandbox.
- **Unified Toolchain**: Everything in one single binary `sorayunara` — compiler, runtime VM, package manager, test runner, linter, formatter, doc generator, profiler, and LSP daemon.

---

## 🛠️ CLI Toolchain Commands (`sorayunara`)

```bash
# Project Lifecycle
sorayunara new <app>       # Create a new Sorayunara project
sorayunara init [app]      # Initialize project in current directory
sorayunara build [--locked]# Compile native binary with reproducible lockfile
sorayunara run [file.sora] # Instant compilation and VM execution
sorayunara test            # Run unit tests, assertions, fuzzing & benchmarks

# Code Quality & Diagnostics
sorayunara fmt [file.sora] # Format source code
sorayunara lint [file.sora]# Static analysis and linter
sorayunara check [file.sora]# Fast type & borrow checking
sorayunara doctor          # Environment & toolchain diagnostics
sorayunara doc [file.sora] # Generate HTML & Markdown documentation
sorayunara lsp             # Language Server Protocol daemon (VS Code ready)

# Package Registry & Security
sorayunara add <pkg>       # Install package from official registry
sorayunara remove <pkg>    # Remove package dependency
sorayunara audit           # Security & dependency vulnerability audit
sorayunara publish         # Publish package to official Sorayunara Registry
```

---

## 📄 Documentation & Specification
- Language Specification: [SPECIFICATION.md](SPECIFICATION.md)
- Contributing Guide: [CONTRIBUTING.md](CONTRIBUTING.md)
- Community Good First Issues: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- Starter Project Template: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- HTML Docs: `docs/index.html` (via `sorayunara doc`)
- VS Code Extension: `editors/vscode/`
- WebAssembly Playground: `playground/`

---
*Maintained by the Sorayunara Core Team (sorayunara.org).*
