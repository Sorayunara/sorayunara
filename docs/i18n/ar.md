# لغة البرمجة سورا يونارا Sorayunara (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  **العربية** •
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


> **لغة برمجية للأنظمة والخدمات الخلفية من الجيل القادم تتميز بنظام استنتاج الأنواع Hindley-Milner، وفحص الاستعارة الحساس للتدفق، وتزامن الممثلين، وتوليد كود أصلي عبر LLVM / WASM / C.**

---

## ⚡ نظرة سريعة على Sorayunara

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

## 🏛️ ركائز الهندسة البرمجية

1. **نظام أنواع هندلي-ميلنر**: استنتاج دقيق للأنواع ومطابقة أنماط شاملة.
2. **أمان الذاكرة دون جامع نفايات**: حماية فائقة من تسريب الذاكرة أثناء الترجمة.
3. **تزامن مبني على الفاعلين (Actors)**: أداء فائق وسرعة معالجة عالية.
4. **دعم منصات متعددة**: توليد كود لـ LLVM و WASM و C99.
5. **بيئة أدوات موحدة**: تتضمن خادم اللغة LSP ومصحح أخطاء DAP ومدير حزم مدمج.

---

## 🛠️ أوامر الأدوات وسطر الأوامر

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

*الترخيص: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
