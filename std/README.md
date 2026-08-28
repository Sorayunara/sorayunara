# 📦 Sorayunara Standard Library (`std/`)

The `std/` directory is strictly dedicated to the **Sorayunara Standard Library**. It contains only core user-space and runtime libraries. Compiler internals and build tools reside exclusively in `compiler/`, `bootstrap/`, and `tools/`.

---

## 📂 Standard Library Structure

```
std/
├── io/            ── Stream I/O, console buffering, formatted printing
├── fs/            ── File system operations, path handling, file streams
├── net/           ── TCP, UDP, DNS, HTTP, WebSockets, gRPC, QUIC
├── collections/   ── Vec, Map, Set, Queue, Deque, Stack, BTreeMap
├── string/        ── Unicode manipulation, string builder, regex
├── math/          ── Linear algebra, fast math primitives, numeric traits
├── time/          ── System clock, duration, Instant, timers, sleep
├── sync/          ── Mutex, RwLock, Atomic primitives, Semaphores
├── process/       ── Subprocess execution, exit codes, environment variables
└── ...
```

---

## 🏛️ Separation of Concerns
- **`std/`**: Standard Library APIs and user-facing modules only.
- **`compiler/`**: AST, Lexer, Parser, Typechecker, Borrowchecker, Optimizer, Codegen.
- **`runtime/`**: M:N Green-thread task scheduler, Memory Allocators, Actors, CSP Channels.
- **`tools/`**: Formatter (`fmt`), Linter (`lint`), Package Manager (`pkg`), Language Server (`lsp`).
