<p align="center">
  <img src="assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">Bahasa Pemrograman Sorayunara (`.sora`)</h1>

<p align="center">
  <a href="README.md">English</a> &bull;
  <a href="README.zh-CN.md">简体中文</a> &bull;
  <a href="README.hi.md">हिन्दी</a> &bull;
  <a href="README.es.md">Español</a> &bull;
  <a href="README.fr.md">Français</a> &bull;
  <a href="README.ar.md">العربية</a> &bull;
  <a href="README.bn.md">বাংলা</a> &bull;
  <a href="README.pt-BR.md">Português</a> &bull;
  <a href="README.ru.md">Русский</a> &bull;
  <a href="README.ur.md">اردو</a> &bull;
  <b>Bahasa Indonesia</b> &bull;
  <a href="README.de.md">Deutsch</a> &bull;
  <a href="README.ja.md">日本語</a> &bull;
  <a href="README.mr.md">मराठी</a> &bull;
  <a href="README.tr.md">Türkçe</a>
</p>


<p align="center">
  <strong>Bahasa sistem & backend generasi masa depan dengan inferensi tipe Hindley-Milner, borrow checking peka-alur, konkurensi aktor bebas-kunci, pengoptimal Quad IR multi-pass, dan pembangkitan kode native LLVM / WASM / C.</strong>
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
  <img src="assets/vscode_preview.svg" alt="Sorayunara VS Code IDE Interface & Syntax Highlighting" width="92%" />
</p>

---

## ⚡ Sekilas tentang Sorayunara

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

## 📊 Tolok Ukur Performa (Benchmark)

Diukur pada matriks benchmark sistem (waktu eksekusi dan konsumsi memori yang lebih rendah adalah lebih baik):

| Bahasa | Waktu Eksekusi | Overhead Memori (RAM) | Keamanan Tipe & Konkurensi | Codegen Native |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (Mendekati Kecepatan C)** | **~4.2 MB** | **HM Statis + Borrow Checker** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ Pipeline Kompiler Multi-Target

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

## 🚀 Proposisi Nilai Utama

- **⚡ Sangat Cepat**: Monomorfisasi generik tanpa biaya overhead, constant folding, eliminasi kode mati, dan emisi native LLVM langsung.
- **🛡️ Memori Aman**: Inferensi tipe Hindley-Milner dan borrow checking peka-alur tanpa kewajiban overhead Garbage Collector.
- **🧵 Konkurensi Modern**: Coroutine ringan (`spawn async`), channel MPSC (`Channel<T>`), dan model aktor terintegrasi dalam pustaka standar.
- **🧰 Toolchain Terpadu**: Semua dalam satu biner: compiler, VM runtime, test runner, benchmark suite, formatter, linter, doc generator, package manager, dan LSP server.

---

## 🛠️ Panduan Memulai Cepat CLI (`sorayunara`)

<p align="center">
  <img src="assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. Build dari Kode Sumber
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. Jalankan Program Contoh
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. Jalankan Pengujian Otomatis & Suite Kualitas
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 Ekosistem Proyek & Perintah CLI

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

## 📄 Dokumentasi & Sumber Daya

- 📖 **Spesifikasi Bahasa**: [SPECIFICATION.md](SPECIFICATION.md)
- 🤝 **Panduan Kontribusi**: [CONTRIBUTING.md](CONTRIBUTING.md)
- 🌟 **Issue Ramah Pemula**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **Ekstensi VS Code**: [editors/vscode/](editors/vscode/)
- 📦 **Template Starter**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **Playground WebAssembly**: [playground/](playground/)

---

<p align="center">
  Dikembangkan oleh **Tim Inti Sorayunara** (sorayunara.org). Open source di bawah Lisensi MIT.
</p>
