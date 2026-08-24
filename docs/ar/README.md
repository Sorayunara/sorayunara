<p align="center">
  <img src="../../assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">لغة البرمجة سورا يونارا Sorayunara (`.sora`)</h1>

<p align="center">
  <a href="../../README.md">English</a> &bull;
  <a href="../../README.zh-CN.md">简体中文</a> &bull;
  <a href="../../README.hi.md">हिन्दी</a> &bull;
  <a href="../../README.es.md">Español</a> &bull;
  <a href="../../README.fr.md">Français</a> &bull;
  <b>العربية</b> &bull;
  <a href="../../README.bn.md">বাংলা</a> &bull;
  <a href="../../README.pt-BR.md">Português</a> &bull;
  <a href="../../README.ru.md">Русский</a> &bull;
  <a href="../../README.ur.md">اردو</a> &bull;
  <a href="../../README.id.md">Bahasa Indonesia</a> &bull;
  <a href="../../README.de.md">Deutsch</a> &bull;
  <a href="../../README.ja.md">日本語</a> &bull;
  <a href="../../README.mr.md">मराठी</a> &bull;
  <a href="../../README.tr.md">Türkçe</a>
</p>


<div dir="rtl">

<p align="center">
  <strong>لغة برمجية للأنظمة والخدمات الخلفية من الجيل القادم تتميز بنظام استنتاج الأنواع Hindley-Milner، وفحص الاستعارة الحساس للتدفق، وتزامن الممثلين بدون أقفال، ومحسن Quad IR متعدد المراحل، وتوليد كود أصلي عبر LLVM / WASM / C.</strong>
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
  <img src="../../assets/vscode_preview.svg" alt="Sorayunara VS Code IDE Interface & Syntax Highlighting" width="92%" />
</p>

---

## ⚡ نظرة سريعة على Sorayunara

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

## 📊 اختبارات الأداء القياسية

تم القياس وفق مصفوفة الأنظمة القياسية (انخفاض وقت التنفيذ واستهلاك الذاكرة هو الأفضل):

| اللغة | وقت التنفيذ | استهلاك الذاكرة (RAM) | سلامة الأنواع والتزامن | توليد كود أصلي |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (سرعة تماثل C)** | **~4.2 MB** | **HM ثابت + فاحص الاستعارة** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ مسار مترجم المنصات المتعددة

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

## 🚀 المزايا والخصائص الأساسية

- **⚡ سرعة فائقة**: توحيد قوالب بدون كلفة، وتبسيط الثوابت، وحذف الكود الميت، وتوليد مباشر للغة الآلة عبر LLVM.
- **🛡️ أمان الذاكرة**: استنتاج أنواع هندلي-ميلنر، وفحص استعارة دقيق دون عبء جامع النفايات الإجباري.
- **🧵 تزامن حديث**: روتينات فرعية خفيفة (`spawn async`)، وقنوات MPSC (`Channel<T>`)، ونماذج الفاعلين في المكتبة القياسية.
- **🧰 حزمة أدوات موحدة**: كل شيء في ملف تنفيذي واحد: المترجم، وبيئة التشغيل، ومشغل الاختبارات، ومنسق الكود، ومدير الحزم، وخادم LSP.

---

## 🛠️ البدء السريع عبر سطر الأوامر (`sorayunara`)

<p align="center">
  <img src="../../assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. البناء من المصدر
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. تشغيل البرامج التجريبية
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. تشغيل حزمة الاختبارات وفحص الجودة
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 بيئة المشروع وأوامر CLI

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

## 📄 التوثيق والمصادر

- 📖 **مواصفات اللغة**: [../../SPECIFICATION.md](../../SPECIFICATION.md)
- 🤝 **دليل المساهمة**: [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
- 🌟 **مهام مناسبة للمبتدئين**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **إضافة VS Code**: [../../editors/vscode/](../../editors/vscode/)
- 📦 **قالب البداية**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **منصة WebAssembly التفاعلية**: [../../playground/](../../playground/)

---

<p align="center">
  تم التطوير بواسطة **فريق Sorayunara الأساسي** (sorayunara.org). مفتوحة المصدر تحت رخصة MIT.
</p>
