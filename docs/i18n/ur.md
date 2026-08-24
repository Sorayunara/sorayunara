# سورایونارا پروگرامنگ زبان (`.sora`)

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
  **اردو** •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **ہنڈلی ملنر ٹائپ انفرینس، فلو حساس بورو چیکنگ، لاک فری ایکٹر کنکرنسی اور مقامی LLVM/WASM/C کوڈجن کے ساتھ اگلی نسل کی سسٹمز زبان۔**

---

## ⚡ سورایونارا کا مختصر جائزہ

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

## 🏛️ بنیادی معماری ستون

1. **ہنڈلی ملنر ٹائپ سسٹم**: تیز رفتار ٹائپ انفرینس اور پیٹرن میچنگ۔
2. **میموری کی حفاظت**: بغیر گاربیج کلیکٹر کے کمپائل وقت پر تحفظ۔
3. **ایکٹر کنکرنسی**: بیک وقت محفوظ اور تیز ترین کارکردگی۔
4. **ملٹی ٹارگٹ کمپائلر**: LLVM، WASM اور C99 سپورٹ۔
5. **مکمل ٹول چین**: LSP، فارمیٹر، ڈیبگر اور پیکیج مینیجر شامل ہیں۔

---

## 🛠️ ٹول چین کمانڈز

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

*لائسنس: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
