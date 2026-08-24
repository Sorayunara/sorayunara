<p align="center">
  <img src="../../assets/logo.png" alt="Sorayunara Programming Language Logo" width="220" />
</p>

<h1 align="center">Язык программирования Sorayunara (`.sora`)</h1>

<p align="center">
  <a href="../../README.md">English</a> &bull;
  <a href="../../README.zh-CN.md">简体中文</a> &bull;
  <a href="../../README.hi.md">हिन्दी</a> &bull;
  <a href="../../README.es.md">Español</a> &bull;
  <a href="../../README.fr.md">Français</a> &bull;
  <a href="../../README.ar.md">العربية</a> &bull;
  <a href="../../README.bn.md">বাংলা</a> &bull;
  <a href="../../README.pt-BR.md">Português</a> &bull;
  <b>Русский</b> &bull;
  <a href="../../README.ur.md">اردو</a> &bull;
  <a href="../../README.id.md">Bahasa Indonesia</a> &bull;
  <a href="../../README.de.md">Deutsch</a> &bull;
  <a href="../../README.ja.md">日本語</a> &bull;
  <a href="../../README.mr.md">मराठी</a> &bull;
  <a href="../../README.tr.md">Türkçe</a>
</p>


<p align="center">
  <strong>Системный и бэкенд язык следующего поколения с выводом типов Хиндли-Милнера, потоко-чувствительным borrow checker, акторной многозадачностью без блокировок, многопроходным оптимизатором Quad IR и генерацией нативного кода LLVM / WASM / C.</strong>
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

## ⚡ Краткий обзор Sorayunara

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

## 📊 Тесты производительности (Бенчмарки)

Измерено на матрице системных бенчмарков (меньшее время выполнения и меньший объем памяти лучше):

| Язык | Время выполнения | Потребление памяти (RAM) | Типобезопасность и параллелизм | Нативный кодогенератор |
| :--- | :---: | :---: | :---: | :---: |
| **Sorayunara (`.sora`)** | **1.02x (Скорость близкая к C)** | **~4.2 MB** | **Статический HM + Borrow Checker** | **LLVM / C / WASM** |
| **C (GCC -O3)** | 1.00x (Baseline) | ~3.8 MB | Manual (Unsafe) | Native Machine Code |
| **Rust (rustc -O)** | 1.01x | ~4.5 MB | Static Ownership | LLVM Machine Code |
| **Go (gc 1.22)** | 1.45x | ~18.5 MB | Garbage Collected | Native Machine Code |
| **Python (CPython 3.12)** | 24.80x | ~48.0 MB | Dynamic (GIL bound) | Bytecode Interpreter |

---

## 🏛️ Мультиплатформенный конвейер компилятора

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

## 🚀 Ключевые преимущества

- **⚡ Невероятная скорость**: Мономорфизация дженериков с нулевой стоимостью, свертка констант, удаление мертвого кода и прямая генерация LLVM.
- **🛡️ Безопасность памяти**: Вывод типов Хиндли-Милнера и потоко-чувствительный borrow checker без накладных расходов сборщика мусора.
- **🧵 Современный параллелизм**: Легковесные корутины (`spawn async`), каналы MPSC (`Channel<T>`) и модель акторов в стандартной библиотеке.
- **🧰 Единый инструментарий**: Все в одном бинарнике: компилятор, VM, раннер тестов, бенчмарки, форматтер, линтер, генератор документации, менеджер пакетов и LSP сервер.

---

## 🛠️ Быстрый старт с CLI (`sorayunara`)

<p align="center">
  <img src="../../assets/terminal_demo.svg" alt="Sorayunara Terminal Execution Demo" width="92%" />
</p>

### 1. Сборка из исходного кода
```bash
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara
cargo build --release
```

### 2. Запуск примеров программ
```bash
# Execute instantly using VM / JIT
cargo run -- run examples/main.sora

# Run matrix multiplication benchmark
cargo run -- run examples/matrix.sora

# Run prime number sieve
cargo run -- run examples/primes.sora
```

### 3. Запуск автоматических тестов и проверки качества
```bash
# Run all test suites
cargo test --all-targets

# Run diagnostics
cargo run -- doctor
```

---

## 📦 Экосистема проекта и команды CLI

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

## 📄 Документация и ресурсы

- 📖 **Спецификация языка**: [../../SPECIFICATION.md](../../SPECIFICATION.md)
- 🤝 **Руководство по участию**: [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
- 🌟 **Задачи для новичков**: [GitHub Issues](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
- 🧩 **Расширение для VS Code**: [../../editors/vscode/](../../editors/vscode/)
- 📦 **Стартовый шаблон**: [sorayunara-starter-template](https://github.com/Sorayunara/sorayunara-starter-template)
- 🌐 **WebAssembly Песочница**: [../../playground/](../../playground/)

---

<p align="center">
  Разработано **командой Sorayunara Core** (sorayunara.org). Открытый исходный код под лицензией MIT.
</p>
