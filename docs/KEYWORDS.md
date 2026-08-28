# 📖 Sorayunara (.sora) — 100 Core Keywords Specification

**Document Version**: 1.0.0  
**Language**: Sorayunara (`.sora`)  
**Design Standard**: Strict 100-Keyword Maximum with Tiered Reservation Architecture  

---

## 🏛️ 1. Architecture of the Keyword System

To prevent grammar pollution and keep variable naming effortless, Sorayunara structures its 100 keywords into **three distinct tiers**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                   TOTAL KEYWORD BUDGET: 100 MAX                        │
├──────────────────────────┬───────────────────────┬─────────────────────┤
│   TIER 1: CORE RESERVED  │  TIER 2: FEATURE-OPT  │ TIER 3: CONTEXTUAL  │
│        (45 Keywords)     │     (30 Keywords)     │    (25 Keywords)    │
│  Always reserved in all  │ Reserved in typings,  │ Active only in AST  │
│  scopes & declarations   │ concurrency & memory  │ contextual patterns │
└──────────────────────────┴───────────────────────┴─────────────────────┘
```

- **Core Reserved (Tier 1)**: Universal keywords that cannot be used as identifiers under any circumstance (`fn`, `if`, `else`, `match`, `struct`, `enum`, `mut`, `const`, `return`, `for`, `while`, etc.).
- **Feature Reserved (Tier 2)**: Keywords active within type declarations, concurrency blocks, ownership, and FFI pipelines (`async`, `await`, `actor`, `spawn`, `channel`, `ref`, `borrow`, `own`, `trait`, `impl`, etc.).
- **Contextual (Tier 3)**: Keywords that act as syntax tokens only within specific grammar contexts, remaining valid as variable or function names elsewhere (`as`, `from`, `where`, `derive`, `default`, `target`, `defer`, `guard`, `link`, `abi`, etc.).

---

## 📑 2. Complete 100-Keyword Catalog & Taxonomy

---

### Category 1: Declaration & Binding (10 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 1 | `fn` | Tier 1 | Function declaration | `fn add(a, b) = a + b` |
| 2 | `let` | Tier 1 | Explicit immutable local binding | `let x: Int = 10` |
| 3 | `mut` | Tier 1 | Mutable variable declaration / binding | `mut counter = 0` |
| 4 | `const` | Tier 1 | Compile-time evaluated constant | `const PI = 3.14159` |
| 5 | `static` | Tier 1 | Global static storage allocation | `static INSTANCE: Engine = init()` |
| 6 | `type` | Tier 1 | Custom type alias definition | `type UserId = Int` |
| 7 | `alias` | Tier 3 | Contextual alias mapping | `alias Point = (Float, Float)` |
| 8 | `struct` | Tier 1 | Aggregate record data structure | `struct User { name: String, age: Int }` |
| 9 | `enum` | Tier 1 | Algebraic Data Type (Tagged union) | `enum Status { Active, Idle, Terminated }` |
| 10 | `union` | Tier 2 | Untagged low-level memory union | `union RawData { i: Int, f: Float }` |

---

### Category 2: Control Flow (14 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 11 | `if` | Tier 1 | Conditional expression / statement | `status = if age >= 18 "adult" else "child"` |
| 12 | `else` | Tier 1 | Fallback alternative branch | `if valid { run() } else { abort() }` |
| 13 | `unless` | Tier 2 | Inverted conditional (`if !cond`) | `unless connected { reconnect() }` |
| 14 | `match` | Tier 1 | Pattern matching expression | `match msg { Ok(v) => v, _ => 0 }` |
| 15 | `case` | Tier 3 | Pattern matching case branch | `case 200 => "OK"` |
| 16 | `for` | Tier 1 | Iterator / collection loop | `for item in items { print(item) }` |
| 17 | `while` | Tier 1 | Condition-driven loop | `while count < 10 { count += 1 }` |
| 18 | `loop` | Tier 1 | Infinite loop block | `loop { if done { break } }` |
| 19 | `break` | Tier 1 | Break loop execution | `if found { break }` |
| 20 | `continue`| Tier 1 | Skip to next loop iteration | `if skip_item { continue }` |
| 21 | `return` | Tier 1 | Explicit function return | `return Err("Timeout")` |
| 22 | `yield` | Tier 2 | Generator coroutine yield | `yield current_element` |
| 23 | `defer` | Tier 3 | Execution deferred to scope exit | `defer file.close()` |
| 24 | `guard` | Tier 3 | Early-exit guard assertion | `guard user.is_authenticated else { return }` |

---

### Category 3: Functions & Callables (8 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 25 | `async` | Tier 1 | Asynchronous coroutine modifier | `async fn fetch(url: String)` |
| 26 | `await` | Tier 1 | Asynchronous task synchronization | `response = await fetch(url)` |
| 27 | `move` | Tier 1 | Move semantics / ownership transfer | `spawn move || { process(data) }` |
| 28 | `inline` | Tier 3 | Compiler inline optimization hint | `inline fn square(x: Int) = x * x` |
| 29 | `extern` | Tier 2 | Foreign ABI declaration | `extern "C" fn malloc(s: Int) -> Ptr` |
| 30 | `unsafe` | Tier 1 | Unchecked memory access block | `unsafe { *raw_ptr = 42 }` |
| 31 | `operator`| Tier 2 | Custom operator overloading declaration | `operator + (a: Vec2, b: Vec2) = ...` |
| 32 | `macro` | Tier 2 | Metaprogramming macro definition | `macro trace(msg) { ... }` |

---

### Category 4: Types & Generics (12 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 33 | `trait` | Tier 1 | Interface / behavior contract | `trait Serializable { fn serialize(&self); }` |
| 34 | `impl` | Tier 1 | Implementation block | `impl Serializable for User { ... }` |
| 35 | `where` | Tier 3 | Generic constraint clause | `fn sort<T>(list: [T]) where T: Comparable` |
| 36 | `generic`| Tier 3 | Generic declaration marker | `generic <T, E>` |
| 37 | `self` | Tier 1 | Receiver instance variable | `fn get_name(&self) = self.name` |
| 38 | `Self` | Tier 1 | Receiver implementing type | `fn create() -> Self` |
| 39 | `dyn` | Tier 2 | Dynamic trait dispatch object | `handler: dyn EventHandler` |
| 40 | `typeof` | Tier 2 | Compile-time type reflection query | `t = typeof(user)` |
| 41 | `sizeof` | Tier 2 | Compile-time byte size reflection | `bytes = sizeof(Int)` |
| 42 | `infer` | Tier 3 | Explicit type inference placeholder | `item: infer = compute()` |
| 43 | `opaque` | Tier 3 | Opaque existential type definition | `type Secret = opaque Key` |
| 44 | `never` | Tier 2 | Non-returning bottom type (`!`) | `fn panic(msg: String) -> never` |

---

### Category 5: Modules & Visibility (10 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 45 | `module` | Tier 1 | Module namespace declaration | `module network.http` |
| 46 | `import` | Tier 1 | External module importation | `import std.io` |
| 47 | `export` | Tier 2 | Re-export symbols from module | `export { Client, Server }` |
| 48 | `pub` | Tier 1 | Public symbol visibility | `pub fn serve(port: Int)` |
| 49 | `private`| Tier 2 | Private restricted visibility | `private fn hash()` |
| 50 | `protected`| Tier 3 | Package/subclass visibility | `protected mut state = 0` |
| 51 | `internal`| Tier 3 | Crate/workspace-only visibility | `internal fn bootstrap()` |
| 52 | `use` | Tier 2 | In-scope namespace inclusion | `use std.collections.Map` |
| 53 | `as` | Tier 3 | Alias renaming in imports/casts | `import std.json as j` |
| 54 | `from` | Tier 3 | Sub-item import source | `from std.math import { sqrt, sin }` |

---

### Category 6: Error Handling (8 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 55 | `try` | Tier 2 | Scoped error catch block | `try { execute() } catch e { ... }` |
| 56 | `catch` | Tier 2 | Error handler branch | `catch err { log(err) }` |
| 57 | `finally`| Tier 3 | Scope cleanup guaranteed block | `finally { cleanup() }` |
| 58 | `throw` | Tier 2 | Throw structured error value | `throw NetworkError("Refused")` |
| 59 | `raise` | Tier 3 | Raise exception alias | `raise CustomException` |
| 60 | `recover`| Tier 3 | Panic recovery handler | `recover || { restart_actor() }` |
| 61 | `error` | Tier 2 | Error type declaration | `type Error = error { Timeout, Closed }` |
| 62 | `result` | Tier 2 | Result return sugar declaration | `fn load() -> result<Data, Error>` |

---

### Category 7: Concurrency & Actor Model (10 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 63 | `actor` | Tier 1 | Isolated actor state declaration | `actor WorkerPool { ... }` |
| 64 | `spawn` | Tier 1 | Spawn asynchronous actor / coroutine | `worker = spawn WorkerPool()` |
| 65 | `send` | Tier 2 | Asynchronous message send | `send worker, Message("Compute")` |
| 66 | `receive`| Tier 2 | Block / poll message mailbox | `receive msg -> handle(msg)` |
| 67 | `channel`| Tier 1 | Channel instantiation primitive | `ch = channel<Int>(1024)` |
| 68 | `select` | Tier 2 | Multi-channel multiplexer | `select { msg = ch1.recv() => ... }` |
| 69 | `parallel`| Tier 3 | Structured parallel fork-join block | `parallel { task_a(), task_b() }` |
| 70 | `sync` | Tier 2 | Thread barrier synchronization | `sync { lock.acquire() }` |
| 71 | `atomic` | Tier 2 | Lock-free atomic operation wrapper | `atomic mut counter: Int = 0` |
| 72 | `lock` | Tier 2 | Mutex lock acquisition | `lock mutex { update() }` |

---

### Category 8: Memory & Ownership (8 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 73 | `ref` | Tier 2 | Immutable borrow reference | `fn inspect(data: ref Buffer)` |
| 74 | `mutref` | Tier 2 | Mutable exclusive borrow reference | `fn modify(data: mutref Buffer)` |
| 75 | `borrow` | Tier 3 | Explicit borrow marker | `borrow resource` |
| 76 | `own` | Tier 3 | Explicit ownership requirement | `own item` |
| 77 | `copy` | Tier 3 | Explicit deep/shallow value copy | `cloned = copy item` |
| 78 | `drop` | Tier 2 | Explicit destructor trigger | `drop(handle)` |
| 79 | `lifetime`| Tier 3 | Explicit lifetime annotation | `struct View<'a> { buf: &'a str }` |
| 80 | `scope` | Tier 2 | Structured concurrency scope | `scope { spawn worker() }` |

---

### Category 9: Pattern Matching & Data Inspection (8 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 81 | `in` | Tier 1 | Membership / iterator operator | `if item in list { ... }` |
| 82 | `is` | Tier 1 | Type inspection & narrowing | `if user is Admin { user.grant() }` |
| 83 | `default`| Tier 3 | Default branch in match / enum | `default => println("fallback")` |
| 84 | `some` | Tier 2 | Option constructor / pattern | `let opt = some("value")` |
| 85 | `none` | Tier 2 | Option empty constructor / pattern | `let opt = none` |
| 86 | `ok` | Tier 2 | Result success constructor | `return ok(42)` |
| 87 | `err` | Tier 2 | Result error constructor | `return err("Failed")` |
| 88 | `test` | Tier 2 | Built-in test suite declaration | `test "matrix multiplication" { ... }` |

---

### Category 10: Compile-Time & Metaprogramming (6 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 89 | `comptime`| Tier 1 | Compile-time execution evaluation | `comptime { generate_lookup_table() }` |
| 90 | `compile`| Tier 3 | Sub-target compile directive | `compile target = "wasm32"` |
| 91 | `derive` | Tier 3 | Automated trait derivation attribute | `@derive(Debug, Clone, Serialize)` |
| 92 | `attribute`| Tier 3 | Custom compiler attribute marker | `@attribute inline(always)` |
| 93 | `reflect`| Tier 3 | Type reflection inspection | `reflect(User).fields()` |
| 94 | `assert` | Tier 2 | Compile & runtime assertion | `assert count > 0, "Invalid size"` |

---

### Category 11: FFI, ABI & Platform Target (6 Keywords)

| # | Keyword | Tier | Purpose / Grammar Rule | Code Example in `.sora` |
| :- | :--- | :--- | :--- | :--- |
| 95 | `native` | Tier 3 | Native platform binding | `native fn sys_clock() -> Int` |
| 96 | `platform`| Tier 3 | Target OS conditional branch | `@platform(os = "linux")` |
| 97 | `target` | Tier 3 | Architecture target flag | `@target(arch = "aarch64")` |
| 98 | `link` | Tier 3 | Native C/C++ library linker | `@link("ssl")` |
| 99 | `abi` | Tier 3 | Calling convention specification | `@abi("C")` |
| 100| `root` | Tier 3 | Workspace package root namespace | `import root.compiler.ast` |

---

## ⚖️ 3. Lexer & Parser Implementation Rules

### Rule 1: Zero Ambiguity Lexing
- The Lexer distinguishes **Tier 1 Reserved Keywords** immediately in tokenization:
  ```rust
  match ident_str {
      "fn" => TokenKind::Fn,
      "let" => TokenKind::Let,
      "mut" => TokenKind::Mut,
      "const" => TokenKind::Const,
      "if" => TokenKind::If,
      "else" => TokenKind::Else,
      "for" => TokenKind::For,
      "while" => TokenKind::While,
      "struct" => TokenKind::Struct,
      "enum" => TokenKind::Enum,
      "match" => TokenKind::Match,
      "actor" => TokenKind::Actor,
      "spawn" => TokenKind::Spawn,
      "async" => TokenKind::Async,
      "await" => TokenKind::Await,
      _ => TokenKind::Ident(ident_str),
  }
  ```

### Rule 2: Contextual Keyword Handling
- **Tier 3 Contextual Keywords** (`as`, `from`, `where`, `derive`, `default`, etc.) are parsed as `Ident` during lexing and resolved contextually by the Parser only when appearing at specific grammatical production nodes:
  - Inside `import ... as ...` ➔ token `as` acts as an alias marker.
  - In `x as Int` ➔ token `as` acts as type cast.
  - In `let as = 10` ➔ valid variable identifier `as`.

---

## 🎯 4. Summary: The Golden Balance

```
100 KEYWORDS MAXIMUM
─────────────────────────────────────────────
45 Core Tokens      ➔ Minimum syntax core
30 Feature Tokens   ➔ Safe systems & concurrency
25 Context Tokens   ➔ Non-polluting extensibility
─────────────────────────────────────────────
RESULT: Clean like Python, Safe like Rust, Zero Boilerplate.
```
