# A Linguagem de Programação Sorayunara (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  **Português** •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Linguagem de sistemas e backend de última geração com inferência de tipos Hindley-Milner, borrow checker sensível ao fluxo, concorrência de atores e geração de código nativo LLVM / WASM / C.**

---

## ⚡ Visão Rápida de Sorayunara

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

## 🏛️ Pilares da Arquitetura

1. **Sistema de Tipos Hindley-Milner**: Inferência de tipos moderna e pattern matching exaustivo.
2. **Segurança de Memória sem GC**: Verificação de empréstimos em tempo de compilação sem vazamentos.
3. **Concorrência de Atores**: Comunicação segura via canais MPSC e threads leves.
4. **Codegen Multi-Alvo**: Compilação nativa para LLVM, WebAssembly e C99.
5. **Toolchain Integrado**: LSP, formatador, debugger DAP e gerenciador de pacotes inclusos.

---

## 🛠️ Comandos da Toolchain

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

*Licença: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
