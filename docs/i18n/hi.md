# सोरायुनारा प्रोग्रामिंग भाषा (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  **हिन्दी** •
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


> **हिंडले-मिलनर टाइप इंफरेंस, फ्लो-सेंसिटिव बॉरो चेकिंग, लॉक-फ्री एक्टर समवर्तीता और मूल LLVM/WASM/C कोडजन के साथ अगली पीढ़ी की सिस्टम और बैकएंड भाषा।**

---

## ⚡ सोरायुनारा का संक्षिप्त विवरण

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

## 🏛️ मुख्य वास्तुकला आधार

1. **हिंडले-मिलनर टाइप सिस्टम**: शून्य बॉयलरप्लेट टाइप इंफरेंस और संपूर्ण पैटर्न मिलान।
2. **मेमोरी सुरक्षा**: कंपाइल-टाइम बॉरो चेकर डेटा रेस और मेमोरी लीक को रोकता है।
3. **एक्टर समवर्तीता**: तेज और सुरक्षित समवर्ती निष्पादन।
4. **मल्टी-टारगेट कंपाइलर**: LLVM, WASM और C99 नेटिव कोड जनरेशन।
5. **एकीकृत टूलचेन**: LSP, फॉर्मेटर, डिबगर और पैकेज मैनेजर शामिल।

---

## 🛠️ टूलचेन और सीएलआई कमांड

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

*लाइसेंस: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
