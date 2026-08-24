# El Lenguaje de Programación Sorayunara (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  **Español** •
  [Français](fr.md) •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  [Português](pt.md) •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Lenguaje de sistemas y backend de próxima generación con inferencia de tipos Hindley-Milner, borrow checking sensible al flujo, concurrencia de actores sin bloqueos y generación de código nativo LLVM / WASM / C.**

---

## ⚡ Vistazo Rápido a Sorayunara

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

## 🏛️ Pilares de la Arquitectura

1. **Sistema de Tipos Hindley-Milner**: Inferencia de tipos sin código repetitivo y coincidencia exhaustiva de patrones.
2. **Seguridad de Memoria sin GC**: Borrow checker que previene condiciones de carrera en tiempo de compilación.
3. **Concurrencia de Actores**: Tareas ligeras con paso de mensajes por canales MPSC.
4. **Generación de Código Multi-Destino**: LLVM nativo, WebAssembly (WASM) y C99.
5. **Toolchain Unificado**: LSP, depurador DAP, formateador y gestor de paquetes integrados.

---

## 🛠️ Comandos de Toolchain y CLI

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

*Licencia: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
