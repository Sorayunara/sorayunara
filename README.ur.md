<p align="center">
  <img src="assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">سورایونارا پروگرامنگ زبان (`.sora`)</h1>

<p align="center">
  [English](README.md) | [简体中文](README.zh-CN.md) | [हिन्दी](README.hi.md) | [Español](README.es.md) | [Français](README.fr.md) | [العربية](README.ar.md) | [বাংলা](README.bn.md) | [Português](README.pt-BR.md) | [Русский](README.ru.md) | **اردو** | [Bahasa Indonesia](README.id.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [मराठी](README.mr.md) | [Türkçe](README.tr.md)
</p>


<div dir="rtl">

<p align="center">
  <strong>ہنڈلی ملنر ٹائپ انفرینس، فلو حساس بورو چیکنگ، لاک فری ایکٹر کنکرنسی، ملٹی پاس کواڈ IR آپٹیمائزر اور مقامی LLVM/WASM/C کوڈجن کے ساتھ اگلی نسل کی سسٹمز اور بیک اینڈ زبان۔</strong>
</p>

</div>

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

## ⚡ سورایونارا کا مختصر جائزہ

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

## 📊 کارکردگی کے بینچ مارکس

سسٹمز بینچ مارک میٹرکس پر ناپا گیا (کم وقت اور کم میموری کا استعمال بہتر ہے):

| زبان | بینچ مارک عمل درآمد کا وقت | میموری اوور ہیڈ (RAM) | ٹائپ سیفٹی اور کنکرنسی | مقامی کوڈجن |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (C کے قریب ترین رفتار)** | **~4.2 MB** | **جامد HM + بورو چیکر** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ ملٹی ٹارگٹ کمپائلر پائپ لائن

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

## 🚀 اہم خصوصیات

- **⚡ انتہائی تیز رفتار**: زیرو لاگت جینیاتی مونو مارفائزیشن، کانسٹنٹ فولڈنگ، ڈیڈ کوڈ کا خاتمہ اور براہ راست LLVM کوڈجن۔
- **🛡️ میموری کی حفاظت**: لازمی گاربیج کلیکٹر کے بغیر ہنڈلی ملنر ٹائپ انفرینس اور فلو حساس بورو چیکنگ۔
- **🧵 جدید کنکرنسی**: ہلکے وزن والے کوریوٹنز (`spawn async`)، MPSC چینلز (`Channel<T>`)، اور معیاری لائبریری میں ایکٹر ماڈل۔
- **🧰 متفقہ ٹول چین**: ایک ہی بائنری میں سب کچھ: کمپائلر، VM رن ٹائم، ٹیسٹ رنر، بینچ مارک سویٹ، فارمیٹر، لنٹر، دستاویزات جنریٹر، پیکیج مینیجر اور LSP سرور۔

---

## 🛠️ سی ایل آئی فوری آغاز (`sorayunara`)

<p align="center">
  <img src="assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. سورس کوڈ سے بنائیں
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. نمونہ پروگرام چلائیں
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. خودکار ٹیسٹ اور کوالٹی سویٹ چلائیں
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 پروجیکٹ ایکو سسٹم اور سی ایل آئی کمانڈز

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

## 📄 دستاویزات اور وسائل

- 📖 **زبان کی تفصیلات**: [SPECIFICATION.md](SPECIFICATION.md)
- 🤝 **تعاون کی گائیڈ**: [CONTRIBUTING.md](CONTRIBUTING.md)
- 🌟 **ابتدائی مسائل**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **VS Code ایکسٹینشن**: [editors/vscode/](editors/vscode/)
- 📦 **اسٹارٹر ٹیمپلیٹ**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly پلے گراؤنڈ**: [playground/](playground/)

---

<p align="center">
  **سورایونارا کور ٹیم** (sorayunara.org) نے تیار کیا۔ MIT لائسنس کے تحت اوپن سورس۔
</p>
