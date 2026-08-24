# Язык программирования Sorayunara (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  [Português](pt.md) •
  **Русский** •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Системный язык следующего поколения с выводом типов Хиндли-Милнера, потоко-чувствительным borrow checker, акторной многозадачностью и генерацией нативного кода LLVM / WASM / C.**

---

## ⚡ Краткий обзор Sorayunara

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

## 🏛️ Архитектурные основы

1. **Система типов Хиндли-Милнера**: Вывод типов без бойлерплейта и исчерпывающее сопоставление с образцом.
2. **Безопасность памяти без сборщика мусора**: Проверка владения на этапе компиляции.
3. **Акторная модель многозадачности**: Быстрый обмен сообщениями по каналам без блокировок.
4. **Мультиплатформенная кодогенерация**: LLVM IR, WebAssembly (WASM) и C99.
5. **Единый набор инструментов**: Встроенный LSP сервер, отладчик DAP, форматтер и менеджер пакетов.

---

## 🛠️ Команды инструментария CLI

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

*Лицензия: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
