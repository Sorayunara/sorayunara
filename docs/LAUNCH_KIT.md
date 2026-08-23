# 🚀 Sorayunara Public Launch & Viral Kit

This document contains ready-to-use launch templates formatted specifically for each developer community platform.

---

## 1. 🌐 Hacker News (news.ycombinator.com)

**Post Title**:
```text
Show HN: Sorayunara – Fast, memory-safe systems language with multi-target codegen
```

**Post URL / Text**:
```text
URL: https://github.com/Sorayunara/sorayunara

Body / First Comment:
Hi HN!

We built Sorayunara (.sora) — a modern, memory-safe systems and backend programming language designed to provide near-C performance without garbage collector pauses or complex lifetime annotations.

Key Highlights:
• Unified Toolchain: Everything in a single binary (compiler, VM, package manager, formatter, test runner, LSP).
• Multi-Target Codegen: Emits direct LLVM IR, C99 (for embedded microcontrollers), WebAssembly, and bytecode.
• Memory Model: Flow-sensitive borrow checking & Hindley-Milner type inference with optional unsafe blocks for systems/FFI programming.
• Concurrency: Built-in coroutines (`spawn async`), MPSC channels (`Channel<T>`), and actor primitives.
• 0 External Dependencies: Built purely using Rust stdlib for 100% reproducible bootstrapping.

GitHub Repository: https://github.com/Sorayunara/sorayunara
Specification: https://github.com/Sorayunara/sorayunara/blob/main/SPECIFICATION.md

We’d love to hear your feedback, thoughts on the language syntax, and ideas for the standard library!
```

---

## 2. 🤖 Reddit (r/programming, r/Compilers, r/rust, r/opensource)

**Post Title**:
```text
Introducing Sorayunara (.sora): An elegant, memory-safe systems language with native LLVM and WASM codegen
```

**Post Body**:
```markdown
Hey everyone!

I wanted to share a new open-source project we've been working on: **Sorayunara** ([GitHub: Sorayunara/sorayunara](https://github.com/Sorayunara/sorayunara)).

### Why another systems language?
We wanted a language that feels as clean and expressive as modern TypeScript/Go, but with the raw performance and deterministic memory safety of Rust/C:

```sora
// Concurrency & Pattern Matching in Sorayunara (.sora)
import std.io
import std.channel

fn main() -> Int {
    let ch: Channel<String> = channel::new(1024)
    spawn async {
        ch.send("Hello from worker coroutine!")
    }
    match ch.recv() {
        Option::Some(msg) => print("Received: " + msg),
        Option::None => print("Channel closed")
    }
    return 0
}
```

### Key Technical Specs:
- **Type System**: Static Hindley-Milner type inference with algebraic data types (Enums) and struct implementations.
- **Compiler Pipeline**: AST -> Three-Address Code Quad IR -> Multi-pass Optimizer -> LLVM IR / C / WASM.
- **Developer Experience**: Official VS Code extension with TextMate syntax highlighting and code snippets.
- **Open Source**: MIT licensed, 100% test pass rate across Linux, macOS, and Windows.

We have marked several beginner-friendly tasks under [`good first issue`](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) and welcome any feedback and contributions!

🔗 **GitHub**: https://github.com/Sorayunara/sorayunara
```

---

## 3. 🐦 X / Twitter Thread

**Tweet 1 (Hook)**:
```text
🌌 Introducing Sorayunara (.sora) — a modern, memory-safe systems & backend programming language with native LLVM & WebAssembly codegen!

⚡ Near-C execution speed
🛡️ Zero-cost borrow checking
🧵 Native coroutines & channels
📦 Single binary toolchain

GitHub: https://github.com/Sorayunara/sorayunara 🧵👇
```

**Tweet 2 (Syntax Showcase)**:
```text
Here is what Sorayunara (.sora) looks like in action:
Clean, expressive syntax with algebraic pattern matching and built-in coroutine workers.

Check out the full language specification:
https://github.com/Sorayunara/sorayunara/blob/main/SPECIFICATION.md
```

**Tweet 3 (Call to Action)**:
```text
Sorayunara is 100% open source under MIT.

⭐ Star the repo on GitHub: https://github.com/Sorayunara/sorayunara
🧩 VS Code Extension included
🤝 PRs and Good First Issues welcome!
```

---

## 4. 📝 Dev.to / Hashnode / Medium Article Draft

**Title**:
```text
Why We Built Sorayunara: A Memory-Safe Systems Language from Scratch
```

**Tags**: `#opensource`, `#rust`, `#programming`, `#webdev`
