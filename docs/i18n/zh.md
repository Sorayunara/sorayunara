# Sorayunara 编程语言 (`.sora`)

<p align="center">
  [English](../../README.md) •
  **简体中文** •
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
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **具有 Hindley-Milner 类型推断、流敏感借用检查、无锁 Actor 并发和原生 LLVM/WASM/C 代码生成的新一代系统与后端编程语言。**

---

## ⚡ Sorayunara 快速预览

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

## 🏛️ 核心架构体系

1. **Hindley-Milner 类型系统与类型收窄**: 零样板类型推断，支持 ADT 和穷尽模式匹配。
2. **无垃圾回收的流敏感借用检查器**: 在编译时保证内存安全，杜绝数据竞争与悬垂指针。
3. **原生 Actor 并发运行时**: 结合轻量级绿色线程与 MPSC 消息通道。
4. **多后端代码生成引擎**: 支持 LLVM 原生机器码、WebAssembly (WASM) 和标准 C99 转译。
5. **现代化一体化工具链**: 内置编译器、LSP 服务、DAP 调试器、代码格式化器与包管理器。

---

## 🛠️ 工具链与 CLI 命令

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

*开源许可证: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
