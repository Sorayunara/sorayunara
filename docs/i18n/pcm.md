# Di Sorayunara Programming Language (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  [Português](pt.md) •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  **Nigerian Pidgin** •
  [मराठी](mr.md)
</p>


> **Next-generation systems and backend language wey get Hindley-Milner type inference, flow-sensitive borrow checking, lock-free actor concurrency, and fast LLVM/WASM/C native machine code generation.**

---

## ⚡ Quick Look for Sorayunara

```sora
import std.io
import std.channel

struct WorkerMessage {
    id: Int,
    payload: String
}

async fn worker_task(id: Int, ch: Channel<WorkerMessage>) -> Result<String, String> {
    print("Worker #" + id.to_string() + " running.")
    let msg = ch.recv()
    match msg {
        Option::Some(data) => {
            print("Received: " + data.payload)
            return Result::Ok("Success")
        },
        Option::None => {
            return Result::Err("Channel closed")
        }
    }
}

fn main() -> Int {
    print("Sorayunara (.sora) Runtime Active!")
    let ch: Channel<WorkerMessage> = channel::new(1024)
    spawn async {
        worker_task(1, ch)
    }
    ch.send(WorkerMessage { id: 1, payload: "Compute task" })
    return 0
}
```

---

## 🏛️ Di Main Power Pillars

1. **Hindley-Milner Type System**: Fast type inference without any long talk, plus complete pattern matching.
2. **Memory Safety without Garbage Collector**: Borrow checker wey dey catch memory issues and data race for compile time.
3. **Actor Concurrency Model**: Fast background tasks wey dey share messages safely with channels.
4. **Multi-Target Compiler**: Fast code generation for LLVM native, WebAssembly (WASM), and C99.
5. **All-in-One Toolchain**: Full LSP server, DAP debugger, code formatter, and package manager.

---

## 🛠️ Toolchain CLI Commands

```bash
sorayunara compile main.sora    # Compile to native target
sorayunara run main.sora        # Compile & run
sorayunara build main.sora      # Build optimized release binary
sorayunara test                 # Run test suites & assert blocks
sorayunara fmt main.sora        # Format source code
sorayunara check main.sora      # Type check & borrow check
sorayunara debug main.sora      # Interactive DAP debug session
sorayunara lsp                  # Language Server Protocol daemon
```

---

*License: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
