# Bahasa Pemrograman Sorayunara (`.sora`)

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
  **Bahasa Indonesia** •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Bahasa sistem & backend generasi masa depan dengan inferensi tipe Hindley-Milner, borrow checking peka-alur, konkurensi aktor bebas-kunci, dan pembangkitan kode native LLVM / WASM / C.**

---

## ⚡ Sekilas tentang Sorayunara

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

## 🏛️ Pilar Arsitektur Utama

1. **Sistem Tipe Hindley-Milner & Penyempitan Tipe**: Inferensi tipe tanpa boilerplate dan pencocokan pola menyeluruh.
2. **Keamanan Memori Tanpa GC**: Pengecekan peminjaman saat kompilasi untuk mencegah kebocoran memori dan race condition.
3. **Konkurensi Model Aktor**: Task asinkron ringan dengan komunikasi channel MPSC.
4. **Pembangkitan Kode Multi-Target**: Mendukung LLVM native, WebAssembly (WASM), dan transpiler C99.
5. **Toolchain Terpadu**: Terintegrasi langsung dengan server LSP, debugger DAP, formatter, dan package manager.

---

## 🛠️ Perintah Toolchain CLI

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

*Lisensi: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
