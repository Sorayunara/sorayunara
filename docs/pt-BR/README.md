<p align="center">
  <img src="../../assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">A Linguagem de Programação Sorayunara (`.sora`)</h1>

<p align="center">
  [English](../en/README.md) | [简体中文](../zh-CN/README.md) | [हिन्दी](../hi/README.md) | [Español](../es/README.md) | [Français](../fr/README.md) | [العربية](../ar/README.md) | [বাংলা](../bn/README.md) | **Português** | [Русский](../ru/README.md) | [اردو](../ur/README.md) | [Bahasa Indonesia](../id/README.md) | [Deutsch](../de/README.md) | [日本語](../ja/README.md) | [मराठी](../mr/README.md) | [Türkçe](../tr/README.md)
</p>


<p align="center">
  <strong>Linguagem de sistemas e backend de última geração com inferência de tipos Hindley-Milner, borrow checking sensível ao fluxo, concorrência de atores sem travas, otimizador Quad IR multi-pass e geração de código nativo LLVM / WASM / C.</strong>
</p>

<p align="center">
  <a href="https://github.com/Sorayunara/sorayunara/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Sorayunara/sorayunara/ci.yml?branch=main&label=CI%20Matrix&logo=github" alt="CI Matrix"></a>
  <a href="https://github.com/Sorayunara/sorayunara/releases"><img src="https://img.shields.io/github/v/release/Sorayunara/sorayunara?color=blue&logo=github" alt="Release"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="https://github.com/Sorayunara/sorayunara"><img src="https://img.shields.io/badge/extension-.sora-purple.svg" alt="Extension"></a>
  <a href="https://github.com/Sorayunara/sorayunara/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22"><img src="https://img.shields.io/badge/good%20first%20issues-welcome-7057ff.svg" alt="Good First Issues"></a>
</p>

<p align="center">
  <img src="../../assets/vscode_preview.svg" alt="Sorayunara VS Code IDE Interface & Syntax Highlighting" width="92%" />
</p>

---

## ⚡ Visão Rápida de Sorayunara

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
    print("Welcome to Sorayunara (.sora) Runtime!")
    let ch: Channel<WorkerMessage> = channel::new(1024)
    
    spawn async {
        worker_task(1, ch)
    }

    ch.send(WorkerMessage { id: 1, payload: "Compute matrix tensor" })
    return 0
}
```

---

## 📊 Benchmarks de Performance

Medido na matriz de benchmarks de sistemas (menor tempo de execução e menor consumo de memória é melhor):

| Linguagem | Tempo de Execução | Consumo de Memória (RAM) | Segurança de Tipos & Concorrência | Codegen Nativo |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (Velocidade Quase-C)** | **~4.2 MB** | **HM Estático + Borrow Checker** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ Pipeline do Compilador Multi-Alvo

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

## 🚀 Principais Propostas de Valor

- **⚡ Extremamente Rápido**: Monomorfização genérica de custo zero, constant folding, eliminação de código morto e emissão LLVM nativa direta.
- **🛡️ Segurança de Memória**: Inferência de tipos Hindley-Milner e borrow checking sensível ao fluxo sem sobrecarga obrigatória de Garbage Collector.
- **🧵 Concorrência Moderna**: Corrotinas leves (`spawn async`), canais MPSC (`Channel<T>`) e modelos de atores integrados à biblioteca padrão.
- **🧰 Toolchain Unificado**: Tudo em um único binário: compilador, runtime VM, executor de testes, suite de benchmarks, formatador, linter, gerador de docs, gerenciador de pacotes e servidor LSP.

---

## 🛠️ Início Rápido via CLI (`sorayunara`)

<p align="center">
  <img src="../../assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. Compilar a partir do Código-Fonte
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. Executar Programas de Exemplo
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. Executar Suite Automatizada de Qualidade e Testes
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 Ecossistema do Projeto & Comandos CLI

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

## 📄 Documentação & Recursos

- 📖 **Especificação da Linguagem**: [../../SPECIFICATION.md](../../SPECIFICATION.md)
- 🤝 **Guia de Contribuição**: [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
- 🌟 **Issues para Iniciantes**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **Extensão para VS Code**: [../../editors/vscode/](../../editors/vscode/)
- 📦 **Template Inicial**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **Playground WebAssembly**: [../../playground/](../../playground/)

---

<p align="center">
  Desenvolvido pelo **Time Central Sorayunara** (sorayunara.org). Código aberto sob Licença MIT.
</p>
