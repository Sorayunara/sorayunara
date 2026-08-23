# 🌌 Sorayunara Programming Language (`.sora`)
> **Sorayunara — Fast, Safe, Elegant, and Expressive Language for Systems, Backend, Embedded, & AI/ML.**

**Organization**: Sorayunara  
**Institution/Business**: Sorayunara  
**Official Extension**: `.sora`  
**Registry**: `https://registry.sorayunara.org`  

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
         Linux/Win/Mac Linux/Mac/Win  Linux/Baremetal
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
- HTML Docs: `docs/index.html` (via `sorayunara doc`)
- VS Code Extension: `editors/vscode/`
- WebAssembly Playground: `playground/`

---
*Maintained by the Sorayunara Core Team (sorayunara.org).*
