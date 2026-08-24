<p align="center">
  <img src="assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">सोरायुनारा प्रोग्रामिंग भाषा (`.sora`)</h1>

<p align="center">
  [English](README.md) | [简体中文](README.zh-CN.md) | **हिन्दी** | [Español](README.es.md) | [Français](README.fr.md) | [العربية](README.ar.md) | [বাংলা](README.bn.md) | [Português](README.pt-BR.md) | [Русский](README.ru.md) | [اردو](README.ur.md) | [Bahasa Indonesia](README.id.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [मराठी](README.mr.md) | [Türkçe](README.tr.md)
</p>


<p align="center">
  <strong>हिंडले-मिलनर टाइप इंफरेंस, फ्लो-सेंसिटिव बॉरो चेकिंग, लॉक-फ्री एक्टर समवर्तीता, मल्टी-पास क्वाड IR ऑप्टिमाइज़र और मल्टी-टारगेट LLVM / WASM / C कोडजन के साथ अगली पीढ़ी की सिस्टम और बैकएंड भाषा।</strong>
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

## ⚡ सोरायुनारा का संक्षिप्त विवरण

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

## 📊 प्रदर्शन बेंचमार्क

सिस्टम बेंचमार्क मैट्रिक्स पर मापा गया (कम निष्पादन समय और कम मेमोरी पदचिह्न बेहतर है):

| भाषा | बेंचमार्क निष्पादन समय | मेमोरी ओवरहेड (RAM) | टाइप सुरक्षा और समवर्तीता | नेटिव कोडजन |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (C के करीब गति)** | **~4.2 MB** | **स्टैटिक HM + बॉरो चेकर** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ मल्टी-टारगेट कंपाइलर पाइपलाइन

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

## 🚀 मुख्य विशेषताएं

- **⚡ अत्यधिक तेज**: शून्य-लागत जेनेरिक मोनोमॉर्फिज़्म, कॉन्स्टेंट फोल्डिंग, डेड कोड एलिमिनेशन और सीधा LLVM कोडजन।
- **🛡️ मेमोरी सुरक्षा**: हिंडले-मिलनर टाइप इंफरेंस, फ्लो-सेंसिटिव बॉरो चेकिंग बिना अनिवार्य GC ओवरहेड।
- **🧵 आधुनिक समवर्तीता**: हल्के कोरूटीन्स (`spawn async`), MPSC चैनल्स (`Channel<T>`), और स्टैंडर्ड लाइब्रेरी में निर्मित एक्टर मॉडल।
- **🧰 एकीकृत टूलचेन**: एक ही बाइनरी में सब कुछ: कंपाइलर, VM रनटाइम, टेस्ट रनर, बेंचमार्क सूट, फॉर्मेटर, लिंटर, डॉक्यूमेंट जेनरेटर, पैकेज मैनेजर और LSP सर्वर।

---

## 🛠️ सीएलआई त्वरित शुरुआत (`sorayunara`)

<p align="center">
  <img src="assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. सोर्स कोड से निर्माण करें
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. उदाहरण प्रोग्राम चलाएं
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. स्वचालित गुणवत्ता और टेस्ट सूट चलाएं
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 प्रोजेक्ट इकोसिस्टम और सीएलआई कमांड्स

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

## 📄 प्रलेखन और संसाधन

- 📖 **भाषा विशिष्टता**: [SPECIFICATION.md](SPECIFICATION.md)
- 🤝 **योगदान गाइड**: [CONTRIBUTING.md](CONTRIBUTING.md)
- 🌟 **शुरुआती मुद्दे**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **VS Code एक्सटेंशन**: [editors/vscode/](editors/vscode/)
- 📦 **स्टार्टर टेम्पलेट**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly प्लेग्राउंड**: [playground/](playground/)

---

<p align="center">
  **सोरायुनारा कोर टीम** (sorayunara.org) द्वारा विकसित। MIT लाइसेंस के तहत ओपन सोर्स।
</p>
