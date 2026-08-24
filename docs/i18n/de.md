# Die Sorayunara Programmiersprache (`.sora`)

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
  **Deutsch** •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **System- und Backend-Sprache der nächsten Generation mit Hindley-Milner-Typinferenz, flusssensitivem Borrow-Checker, sperrfreier Actor-Parallelität und nativer LLVM/WASM/C-Codegenerierung.**

---

## ⚡ Kurzüberblick über Sorayunara

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

## 🏛️ Kern-Architekturpfeiler

1. **Hindley-Milner Typsystem**: Typinferenz ohne Boilerplate und erschöpfendes Pattern Matching.
2. **Speichersicherheit ohne GC**: Borrow-Checker zur Vermeidung von Data Races zur Compile-Zeit.
3. **Actor-Nebenläufigkeit**: Leichtgewichtige asynchrone Tasks mit MPSC-Channels.
4. **Multi-Target-Codegen**: Native LLVM-Unterstützung, WebAssembly (WASM) und C99-Transpilierung.
5. **Einheitliche Toolchain**: Integrierter LSP-Server, DAP-Debugger, Formatierer und Paketmanager.

---

## 🛠️ Toolchain-Befehle

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

*Lizenz: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
