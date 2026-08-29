# ⚡ Sorayunara Concurrency Model: Actors, Fibers & CSP Channels
**Runtime Architecture**: Work-Stealing M:N Green Thread Scheduler

---

## 1. Concurrency Hierarchy
- **Fibers (Green Tasks)**: Ultralightweight userspace execution contexts (~2 KB initial stack).
- **Channels (`Channel[T]`)**: Lock-free SPSC / MPMC bounded message queues.
- **Actors**: Autonomous entities with isolated heaps that communicate exclusively via asynchronous message mailboxes.

---

## 2. Spawning Fibers & Await
```sora
import Std.Async::{Fiber, Channel};

fn worker(task_id: Int, ch: Channel[String]) {
    ch.send(format("Task {} finished", task_id));
}

fn main() -> Int {
    let ch = Channel::bounded(10);
    for i in 0..5 {
        Fiber::spawn(move || {
            worker(i, ch.clone());
        });
    }
    return 0;
}
```
