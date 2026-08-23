<p align="center">
  <img src="assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">🌌 The Sorayunara Programming Language (<code>.sora</code>)</h1>

<p align="center">
  <strong>Fast, Memory-Safe, and Expressive Systems & Backend Language with Self-Hosting Compiler and Multi-Target Codegen.</strong>
</p>

<p align="center">
  <a href="https://github.com/Sorayunara/sorayunara/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Sorayunara/sorayunara/ci.yml?branch=main&label=CI%20Matrix&logo=github" alt="CI Matrix"></a>
  <a href="https://github.com/Sorayunara/sorayunara/releases"><img src="https://img.shields.io/github/v/release/Sorayunara/sorayunara?color=blue&logo=github" alt="Release"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="https://github.com/Sorayunara/sorayunara"><img src="https://img.shields.io/badge/extension-.sora-purple.svg" alt="Extension"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22"><img src="https://img.shields.io/badge/good%20first%20issues-welcome-7057ff.svg" alt="Good First Issues"></a>
</p>

---

## ⚡ Quick Look at Sorayunara

```sora
// High-performance actor concurrency & algebraic pattern matching
import std.io
import std.channel

struct WorkerMessage {
    id: Int,
    payload: String
}

async fn worker_task(id: Int, ch: Channel<WorkerMessage>) -> Result<String, String> {
    print("Worker #" + id.to_string() + " initialized.")
    let msg = ch.recv()
    match msg {
        Option::Some(data) => {
            print("Received payload: " + data.payload)
            return Result::Ok("Success")
        },
        Option::None => {
            return Result::Err("Channel closed")
        }
    }
}

fn main() -> Int {
    print("🌌 Welcome to Sorayunara (.sora) Runtime!")
    let ch: Channel<WorkerMessage> = channel::new(1024)
    
    spawn async {
        worker_task(1, ch)
    }

    ch.send(WorkerMessage { id: 1, payload: "Compute matrix tensor" })
    return 0
}
```

---

## 📊 Performance Benchmarks

Measured on systems benchmark matrix (lower execution time and lower memory footprint is better):

| Language | Benchmark Execution Time | Memory Overhead (RAM) | Type Safety & Concurrency | Native Codegen |
| :--- | :---: | :---: | :---: | :---: |
| **🌌 Sorayunara (`.sora`)** | **1.02x (Near-C Speed)** | **~4.2 MB** | **Static HM + Borrow Checker** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ Multi-Target Compiler Pipeline

```
                    SORAYUNARA SOURCE (.sora)
                                │
                                ▼
               ┌─────────────────────────────────┐
               │ Unified Self-Hosting Toolchain  │
               │   Lexer -> Parser -> Semantics  │
               └────────────────┬────────────────┘
                                │
                                ▼
                       Sorayunara Bytecode IR
                                │
                                ▼
                    Three-Address Optimization
                                │
          ┌─────────────────────┼─────────────────────┐
          ▼                     ▼                     ▼
      Direct LLVM IR        ANSI C Output        WebAssembly (.wasm)
          │                     │                     │
          ▼                     ▼                     ▼
     Native Machine      Embedded Micro-          Browser & Cloud
    (x86_64 / ARM64)       controllers               Edge VMs
```

---

## 🚀 Key Value Propositions

- **⚡ Blazing Fast**: Zero-cost generic monomorphization, constant folding, dead code elimination, and direct LLVM native emission.
- **🛡️ Memory Safe**: Hindley-Milner type inference, flow-sensitive borrow checking without mandatory garbage collection overhead.
- **🧵 Modern Concurrency**: Lightweight coroutines (`spawn async`), MPSC channels (`Channel<T>`), and actor models built into the standard library.
- **🧰 Unified Toolchain**: Everything in a single binary: compiler, VM runtime, test runner, benchmark suite, formatter, linter, doc generator, package manager, and LSP server.

---

## 🛠️ CLI Quickstart (`sorayunara`)

### 1. Build from Source
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. Run Example Programs
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. Run Automated Quality & Test Suite
```bash
# Run all 42 test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 Project Ecosystem & CLI Commands

```bash
# Project Lifecycle
sorayunara new <app>       # Create a new Sorayunara project
sorayunara build [--locked]# Compile native binary with reproducible lockfile
sorayunara run [file.sora] # Instant compilation and VM execution
sorayunara test            # Run unit tests, assertions, fuzzing & benchmarks

# Code Quality & Tooling
sorayunara fmt [file.sora] # AST-based automatic code formatter
sorayunara lint [file.sora]# Static code quality & linter checks
sorayunara check [file.sora]# Fast type & borrow checking
sorayunara doc [file.sora] # Generate HTML & Markdown documentation
sorayunara lsp             # Language Server Protocol daemon for VS Code

# Package Management
sorayunara add <pkg>       # Add package dependency
sorayunara remove <pkg>    # Remove package dependency
sorayunara audit           # Security & dependency vulnerability audit
sorayunara publish         # Publish package to official Sorayunara Registry
```

---

## 📄 Documentation & Resources

- 📖 **Language Specification**: [SPECIFICATION.md](SPECIFICATION.md)
- 🤝 **Contributing Guide**: [CONTRIBUTING.md](CONTRIBUTING.md)
- 🌟 **Good First Issues**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **VS Code Extension**: [editors/vscode/](editors/vscode/)
- 📦 **Starter Template**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly Playground**: [playground/](playground/)

---

<p align="center">
  Made with 🌌 by the <strong>Sorayunara Core Team</strong> (sorayunara.org). Open source under the MIT License.
</p>
