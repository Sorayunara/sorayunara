# সোরায়ুনারা প্রোগ্রামিং ভাষা (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  [العربية](ar.md) •
  **বাংলা** •
  [Português](pt.md) •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **হিন্ডলে-মিলনার টাইপ ইনফারেন্স, মেমরি-নিরাপদ বরো চেকিং, লক-মুক্ত অ্যাক্টর কনকারেন্সি এবং এলএলভিএম / ওয়াসম / সি নেটিভ কোডজেন সহ পরবর্তী প্রজন্মের সিস্টেম ল্যাঙ্গুয়েজ।**

---

## ⚡ সোরায়ুনারার এক ঝলক

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

## 🏛️ মূল আর্কিটেকচার স্তম্ভ

1. **হিন্ডলে-মিলনার টাইপ সিস্টেম**: চমৎকার টাইপ ইনফারেন্স ও প্যাটার্ন ম্যাচিং।
2. **মেমরি নিরাপত্তা**: কম্পাইল টাইমে ডেটা রেস ও মেমরি ত্রুটি প্রতিরোধ করে।
3. **অ্যাক্টর কনকারেন্সি**: দ্রুত এবং নির্ভরযোগ্য সমান্তরাল প্রক্রিয়াকরণ।
4. **মাল্টি-টার্গেট কম্পাইলার**: LLVM, WASM এবং C99 সমর্থন।
5. **একীভূত টুলচেন**: এলএসপি, ডিবাগার, ফরম্যাটার এবং প্যাকেজ ম্যানেজার।

---

## 🛠️ টুলচেন কমান্ডসমূহ

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

*লাইসেন্স: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
