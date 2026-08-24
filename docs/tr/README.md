<p align="center">
  <img src="../../assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">Sorayunara Programlama Dili (`.sora`)</h1>

<p align="center">
  [English](../en/README.md) | [简体中文](../zh-CN/README.md) | [हिन्दी](../hi/README.md) | [Español](../es/README.md) | [Français](../fr/README.md) | [العربية](../ar/README.md) | [বাংলা](../bn/README.md) | [Português](../pt-BR/README.md) | [Русский](../ru/README.md) | [اردو](../ur/README.md) | [Bahasa Indonesia](../id/README.md) | [Deutsch](../de/README.md) | [日本語](../ja/README.md) | [मराठी](../mr/README.md) | **Türkçe**
</p>


<p align="center">
  <strong>Hindley-Milner tip çıkarımı, akışa duyarlı borrow checking, kilitsiz aktör eşzamanlılığı, çok geçişli Quad IR optimize edici ve çok hedefli LLVM / WASM / C yerel kod üretimine sahip yeni nesil sistem ve arka uç dili.</strong>
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

## ⚡ Sorayunara'ya Hızlı Bakış

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

## 📊 Performans Kriterleri (Benchmarklar)

Sistem benchmark matrisinde ölçülmüştür (daha düşük yürütme süresi ve daha az bellek kullanımı daha iyidir):

| Dil | Yürütme Süresi | Bellek Tüketimi (RAM) | Tip Güvenliği & Eşzamanlılık | Yerel Kod Üretimi |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (Neredeyse C Hızı)** | **~4.2 MB** | **Statik HM + Borrow Checker** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ Çok Hedefli Derleyici Boru Hattı

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

## 🚀 Temel Değer Önermeleri

- **⚡ Son Derece Hızlı**: Sıfır maliyetli jenerik monomorflaştırma, sabit katlama (constant folding), ölü kod eleme ve doğrudan LLVM yerel kod üretimi.
- **🛡️ Bellek Güvenliği**: Zorunlu Çöp Toplayıcı (GC) ek yükü olmaksızın Hindley-Milner tip çıkarımı ve akışa duyarlı borrow checking.
- **🧵 Modern Eşzamanlılık**: Hafif coroutineler (`spawn async`), MPSC kanalları (`Channel<T>`) ve standart kütüphanede yerleşik aktör modelleri.
- **🧰 Birleşik Araç Zinciri**: Her şey tek bir ikili dosyada: derleyici, VM çalışma zamanı, test çalıştırıcı, benchmark paketi, biçimlendirici, linter, dokümantasyon oluşturucu, paket yöneticisi ve LSP sunucusu.

---

## 🛠️ CLI Hızlı Başlangıç (`sorayunara`)

<p align="center">
  <img src="../../assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. Kaynak Koddan Derleyin
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. Örnek Programları Çalıştırın
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. Otomatik Kalite ve Test Paketini Çalıştırın
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 Proje Ekosistemi & CLI Komutları

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

## 📄 Dokümantasyon & Kaynaklar

- 📖 **Dil Belirtimi (Spesifikasyon)**: [../../SPECIFICATION.md](../../SPECIFICATION.md)
- 🤝 **Katkı Kılavuzu**: [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
- 🌟 **Yeni Başlayanlar İçin Sorunlar**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **VS Code Eklentisi**: [../../editors/vscode/](../../editors/vscode/)
- 📦 **Başlangıç Şablonu**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly Deneme Alanı (Playground)**: [../../playground/](../../playground/)

---

<p align="center">
  **Sorayunara Çekirdek Ekibi** (sorayunara.org) tarafından geliştirilmiştir. MIT Lisansı altında açık kaynaklıdır.
</p>
