# 🛠️ Sorayunara Unified Toolchain Architecture

The Sorayunara toolchain (`sora` / `sorayunara`) is a single binary that integrates every aspect of the developer workflow.

---

## 🏛️ Integrated Tooling Ecosystem

```
sora
├── run <file.sora>       ── Fast execution via Virtual Machine
├── build <file.sora>     ── Ahead-of-time compilation to Native / WASM / C
├── check <file.sora>     ── Typecheck & borrowcheck without compilation
├── test                  ── Integrated unit, integration, and fuzz test runner
├── bench                 ── Nanosecond-precision performance benchmarking
├── fmt <file.sora>       ── AST-driven source code formatter
├── lint <file.sora>      ── Idiom and anti-slop rule enforcement
├── pkg                   ── Package manager, dependency resolver & lockfiles
├── lsp                   ── Language Server Protocol (LSP) daemon
└── doctor                ── Environment, toolchain, and dependency diagnostics
```
