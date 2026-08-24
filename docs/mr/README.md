<p align="center">
  <img src="../../assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">सोरायुनारा प्रोग्रामिंग भाषा (`.sora`)</h1>

<p align="center">
  [English](../en/README.md) | [简体中文](../zh-CN/README.md) | [हिन्दी](../hi/README.md) | [Español](../es/README.md) | [Français](../fr/README.md) | [العربية](../ar/README.md) | [বাংলা](../bn/README.md) | [Português](../pt-BR/README.md) | [Русский](../ru/README.md) | [اردو](../ur/README.md) | [Bahasa Indonesia](../id/README.md) | [Deutsch](../de/README.md) | [日本語](../ja/README.md) | **मराठी** | [Türkçe](../tr/README.md)
</p>


<p align="center">
  <strong>हिंडले-मिलनर टाईप इन्फरन्स, फ्लो-सेन्सिटिव्ह बॉरो चेकिंग, लॉक-फ्री अ‍ॅक्टर समवर्तीता, मल्टी-पास क्वाड IR ऑप्टिमाइझर आणि मल्टी-टार्गेट LLVM / WASM / C कोड जनरेशनसह पुढील पिढीची सिस्टीम आणि बॅकएंड भाषा.</strong>
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

## ⚡ सोरायुनाराचा संक्षिप्त परिचय

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

## 📊 कार्यक्षमता बेंचमार्क

सिस्टीम बेंचमार्क मॅट्रिक्सवर मोजले गेले (कमी कार्यान्वयन वेळ आणि कमी मेमरी वापर उत्तम मानला जातो):

| भाषा | बेंचमार्क वेळ | मेमरी वापर (RAM) | टाईप सुरक्षितता आणि समवर्तीता | नेटिव्ह कोडजन |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (C च्या जवळपास गती)** | **~4.2 MB** | **स्टॅटिक HM + बॉरो चेकर** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ मल्टी-टार्गेट कंपाइलर पाइपलाइन

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

## 🚀 प्रमुख वैशिष्ट्ये

- **⚡ अत्यंत वेगवान**: शून्य-किंमत जेनेरिक मोनोमॉर्फिझम, कॉन्स्टंट फोल्डिंग, डेड कोड निर्मूलन आणि थेट LLVM कोड जनरेशन.
- **🛡️ मेमरी सुरक्षा**: सक्तीच्या GC ओव्हरहेडशिवाय हिंडले-मिलनर टाईप इन्फरन्स आणि फ्लो-सेन्सिटिव्ह बॉरो चेकिंग.
- **🧵 आधुनिक समवर्तीता**: हलके कोरूटिन्स (`spawn async`), MPSC चॅनेल्स (`Channel<T>`), आणि स्टँडर्ड लायब्ररीमध्ये अ‍ॅक्टर मॉडेल्स.
- **🧰 एकात्मिक टूलचेन**: एकाच बायनरीमध्ये सर्वकाही: कंपाइलर, VM रनटाइम, टेस्ट रनर, बेंचमार्क संच, फॉरमॅटर, लिंटर, डॉक जनरेटर, पॅकेज मॅनेजर आणि LSP सर्व्हर.

---

## 🛠️ सीएलआय द्रुत सुरुवात (`sorayunara`)

<p align="center">
  <img src="../../assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. सोर्स कोडमधून बिल्ड करा
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. उदाहरण प्रोग्राम चालवा
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. स्वयंचलित गुणवत्ता आणि चाचणी संच चालवा
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 प्रकल्प इकोसिस्टम आणि सीएलआय कमांड्स

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

## 📄 दस्तऐवजीकरण आणि संसाधने

- 📖 **भाषा तपशील**: [../../SPECIFICATION.md](../../SPECIFICATION.md)
- 🤝 **योगदान मार्गदर्शक**: [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
- 🌟 **नवशिक्यांसाठी उपयुक्त समस्या**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **VS Code एक्स्टेंशन**: [../../editors/vscode/](../../editors/vscode/)
- 📦 **स्टार्टर टेम्पलेट**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly प्लेग्राउंड**: [../../playground/](../../playground/)

---

<p align="center">
  **सोरायुनारा कोर टीम** (sorayunara.org) द्वारे विकसित. MIT परवान्याअंतर्गत मुक्त स्रोत.
</p>
