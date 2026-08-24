# सोरायुनारा प्रोग्रामिंग भाषा (`.sora`)

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
  [Nigerian Pidgin](pcm.md) •
  **मराठी**
</p>


> **हिंडले-मिलनर टाईप इन्फरन्स, फ्लो-सेन्सिटिव्ह बॉरो चेकिंग, लॉक-फ्री अ‍ॅक्टर समवर्तीता आणि नेटिव्ह LLVM/WASM/C कोड जनरेशनसह पुढील पिढीची सिस्टीम भाषा.**

---

## ⚡ सोरायुनाराचा संक्षिप्त परिचय

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

## 🏛️ मुख्य वास्तुकला आधारस्तंभ

1. **हिंडले-मिलनर टाईप सिस्टीम**: स्वयंचलित प्रकार अनुमान आणि संपूर्ण पॅटर्न मॅचिंग.
2. **मेमरी सुरक्षा**: कंपाइल वेळेस मेमरी सुरक्षितता आणि डेटा रेस प्रतिबंध.
3. **अ‍ॅक्टर समवर्तीता**: सुरक्षित संदेश वहन आणि जलद कार्यक्षमता.
4. **मल्टी-टार्गेट कंपाइलर**: LLVM, WASM आणि C99 साठी नेटिव्ह कोड जनरेशन.
5. **एकात्मिक टूलचेन**: LSP, फॉरमॅटर, डीबगर आणि पॅकेज मॅनेजर समाविष्ट.

---

## 🛠️ टूलचेन कमांड्स

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

*परवाना: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
