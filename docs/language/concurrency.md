# 🧵 Sorayunara Concurrency Model

Sorayunara incorporates native language-level primitives for asynchronous coroutines, actors, and typed channels.

---

## 1. Async & Coroutines
```sora
async fn fetch_data(id: Int) -> String {
    return "Data for #" + id.to_string()
}

fn main() -> Int {
    let task = spawn fetch_data(1)
    let result = await task
    print(result)
    return 0
}
```

---

## 2. Channels
```sora
import std.channel

fn main() -> Int {
    let ch = make_chan<Int>(10)
    ch.send(42)
    let val = ch.recv()
    print("Received: ", val)
    return 0
}
```
