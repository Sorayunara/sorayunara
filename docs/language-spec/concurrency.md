# ⚡ Sorayunara Formal Language Specification: Concurrency & Asynchronous Runtime

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/concurrency.md`

---

## 1. Concurrency Triad

Sorayunara provides three complementary concurrency paradigms unified under compile-time thread safety:

```
                          Sorayunara Concurrency
                                    │
           ┌────────────────────────┼────────────────────────┐
           ▼                        ▼                        ▼
     [Async / Await]         [CSP Channels]           [Actor Model]
  Cooperative Fibers      Typed MPMC Message      Isolated Mailboxes &
  & Non-blocking I/O        Synchronizers          Supervision Trees
```

---

## 2. Async / Await & Cooperative Tasks

### 2.1 Asynchronous Functions
Functions marked with `async` compile into state machines executing on a cooperative green-thread scheduler ([`concurrency_runtime.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/concurrency_runtime.rs)):

```sora
async fn fetch_payload(url: String) -> [Byte]!HttpError {
    resp = await http.get(url)?
    Ok(resp.body())
}
```

### 2.2 Task Spawning (`spawn`)
`spawn` creates a lightweight fiber scheduled across an M:N threadpool:

```sora
handle = spawn fetch_payload("https://api.sorayunara.org/v1/status")
result = await handle?
```

---

## 3. Structured Concurrency (`scope`)

A `scope` block guarantees that all tasks spawned within it are joined before the scope exits:

```sora
results = scope { |s|
    t1 = s.spawn(|| fetch_data_source_a())
    t2 = s.spawn(|| fetch_data_source_b())
    [await t1, await t2]
} // Scope blocks here until all child tasks finish or any task fails
```

---

## 4. CSP Typed Channels (`chan`)

Channels facilitate communication by passing ownership of values between concurrent tasks:

```sora
// Create a buffered typed channel with capacity 10
ch = chan[Int].buffered(10)

spawn move {
    for i in 0..5 {
        ch.send(i) // Or: ch <- i
    }
    ch.close()
}

while val in ch {
    println("Received: ${val}")
}
```

---

## 5. Actor Concurrency Model

Actors are stateful entities that communicate exclusively via asynchronous message queues:

```sora
actor CounterActor {
    mut count: Int = 0

    fn handle_message(mut self, msg: CounterMsg) {
        match msg {
            Increment(amount) => self.count += amount,
            Get(reply_to) => reply_to.send(self.count),
        }
    }
}
```

---

## 6. Thread-Safety Traits (`Send` & `Sync`)

- `Send`: Types whose ownership can safely transfer across thread boundaries.
- `Sync`: Types whose references (`&T`) can safely be shared across threads.
- Compile-time guarantee: **No shared mutable state without synchronization**.
