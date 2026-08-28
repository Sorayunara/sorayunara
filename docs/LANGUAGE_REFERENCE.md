# 📖 Sorayunara (.sora) Complete Language Reference & Dictionary

**Document Version**: 2.0.0  
**Language**: Sorayunara (`.sora`)  
**Design Philosophy**: **"Python Brevity + Rust Safety + Functional Pipeline + Built-in Concurrency"**  

---

## 📑 Table of Contents

1. [Variable & Value](#1-variable--value)
2. [Function & Callables](#2-function--callables)
3. [Condition & Branching](#3-condition--branching)
4. [Loops & Iterations](#4-loops--iterations)
5. [Struct & Object Orientation](#5-struct--object-orientation)
6. [Visibility & Access Control](#6-visibility--access-control)
7. [Modules & Packaging](#7-modules--packaging)
8. [Error Handling & Propagation](#8-error-handling--propagation)
9. [Concurrency & Actor Model](#9-concurrency--actor-model)
10. [Memory & Ownership Model](#10-memory--ownership-model)
11. [Collections & Data Structures](#11-collections--data-structures)
12. [Pipeline & Functional Operators](#12-pipeline--functional-operators)
13. [Testing & Benchmarking](#13-testing--benchmarking)
14. [Web & Backend Standard DSL](#14-web--backend-standard-dsl)
15. [Database & Query Operations](#15-database--query-operations)
16. [CLI & System APIs](#16-cli--system-apis)
17. [Metaprogramming & Compile-Time Evaluation](#17-metaprogramming--compile-time-evaluation)
18. [Core Operator Reference Table](#18-core-operator-reference-table)

---

## 1. Variable & Value

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `let` | `let x = 10` | Explicit local variable binding |
| `mut` | `mut count = 0` / `let mut x = 10` | Mutable variable that can be updated |
| `const` | `const PI = 3.14159` | Compile-time evaluated constant |
| `val` | `val name = "Lutfi"` | Immutable value binding alias |
| `ref` | `ref data` | Immutable reference |
| `move` | `move data` | Explicit ownership transfer |
| `copy` | `copy buffer` | Explicit value copy |
| `type` | `type UserId = Int` | Custom type definition / alias |
| `alias` | `alias Point = (Float, Float)` | Type name alias |
| `default`| `default timeout = 5000` | Default parameter value |

```sora
// Short declaration
name := "Lutfi"
age := 20
mut count := 0
count += 1
```

---

## 2. Function & Callables

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `fn` | `fn add(a, b) => a + b` | Function declaration |
| `return` | `return x` | Explicit early value return |
| `async` | `async fn load()` | Asynchronous coroutine modifier |
| `await` | `await load()` | Await asynchronous promise |
| `yield` | `yield item` | Generator coroutine yield |
| `inline` | `inline fn square(x) => x * x` | Compiler inlining directive |
| `pure` | `pure fn add(a, b) => a + b` | Side-effect-free pure function |
| `extern` | `extern "C" fn puts(s: String)`| Foreign ABI symbol binding |
| `native` | `native fn sys_clock() -> Int` | Platform native implementation |

---

## 3. Condition & Branching

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `if` | `if x > 0 => print(x)` | Conditional expression |
| `else` | `else => print("zero/neg")` | Fallback alternative branch |
| `elif` | `elif x == 0 => print("zero")` | Secondary condition branch |
| `unless` | `unless ok => retry()` | Inverted condition (`if !ok`) |
| `when` | `when x > 10 => handle_overflow()` | Guard expression |
| `match` | `match x { 0 => "zero", _ => "other" }` | Pattern matching expression |
| `case` | `case 200 => "OK"` | Branch pattern arm |
| `default`| `default => "fallback"` | Fallback pattern arm |

---

## 4. Loops & Iterations

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `for` | `for x in xs { print(x) }` | Standard iterator loop |
| `each` | `each x in xs => print(x)` | Concise element iterator |
| `while` | `while x < 10 => x += 1` | Condition loop |
| `loop` | `loop { work() }` | Infinite loop |
| `break` | `if done => break` | Exit loop immediately |
| `continue`| `if skip_item => continue` | Skip to next loop cycle |
| `in` | `x in xs` | Collection membership test |
| `step` | `0..10 step 2` | Range step interval |
| `until` | `until x == 10 => x += 1` | Loop until condition is met |

---

## 5. Struct & Object Orientation

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `struct` | `struct User { name String, age Int }` | Aggregate data structure |
| `class` | `class Engine { ... }` | Object class definition |
| `object` | `object Config { port = 8080 }` | Singleton object namespace |
| `enum` | `enum State { On, Off }` | Tagged union / ADT |
| `trait` | `trait Show { fn show(&self) }` | Interface contract |
| `impl` | `impl Show for User { ... }` | Trait implementation |
| `new` | `new User("Lutfi", 20)` | Object constructor |
| `self` | `self.name` | Instance receiver |
| `base` | `base.init()` | Superclass delegator |
| `field` | `field count Int` | Explicit struct field |

---

## 6. Visibility & Access Control

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `pub` | `pub fn serve()` | Public visibility |
| `priv` | `priv fn secret()` | Private module visibility |
| `internal` | `internal fn link()` | Workspace-internal visibility |
| `protected`| `protected mut state = 0` | Subclass/package visibility |
| `export` | `export { Client, Server }` | Re-export symbols |
| `hidden` | `hidden fn debug_hook()` | Hide from generated documentation |

---

## 7. Modules & Packaging

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `import` | `import std.io` | Import external module |
| `use` | `use std.math.sqrt` | Bring symbol into local scope |
| `from` | `from std.math use { sqrt, sin }` | Sub-item import source |
| `as` | `import std.json as j` | In-scope symbol alias |
| `module` | `module network.http` | Declare module namespace |
| `package`| `package web_service` | Declare package boundary |
| `include`| `include "utils.sora"` | Literal file inclusion |

---

## 8. Error Handling & Propagation

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `try` | `try do_task() catch e => log(e)` | Scoped error attempt |
| `catch` | `catch err => print(err)` | Error exception handler |
| `throw` | `throw Error("Invalid")` | Raise structured error |
| `raise` | `raise CustomException` | Exception raise alias |
| `finally`| `finally => file.close()` | Guaranteed cleanup block |
| `recover`| `recover e => restart()` | Panic recovery handler |
| `assert` | `assert x > 0, "Non-positive"` | Validation assertion |
| `panic` | `panic("Fatal state")` | Abort process with diagnostic |
| `?` | `data := read_file()?` | Propagate error early if `Err`/`None` |

---

## 9. Concurrency & Actor Model

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `spawn` | `spawn worker_task(1, ch)` | Spawn concurrent task/actor |
| `async` | `async fn fetch()` | Asynchronous function |
| `await` | `await fetch()` | Await async task resolution |
| `actor` | `actor Server { ... }` | Isolated state actor |
| `send` | `send ch, msg` / `ch <- msg` | Send message to channel/actor |
| `recv` | `msg := recv ch` | Receive message from channel |
| `channel`| `ch := channel<Int>(1024)` | Create message channel |
| `select` | `select { msg = ch.recv() => ... }` | Channel multiplexer |
| `parallel`| `a, b := parallel { task_a(), task_b() }` | Structured fork-join parallel |
| `sync` | `sync { lock.acquire() }` | Thread barrier synchronization |
| `lock` | `lock mutex { update() }` | Mutex lock acquisition |
| `unlock` | `unlock mutex` | Release mutex lock |
| `atomic` | `atomic mut counter = 0` | Lock-free atomic variable |
| `task` | `task job()` | Task descriptor |

---

## 10. Memory & Ownership Model

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `own` | `own buffer` | Explicit ownership constraint |
| `borrow` | `borrow buffer` | Explicit immutable borrow |
| `move` | `move resource` | Transfer ownership |
| `ref` | `ref data` | Immutable reference (`&T`) |
| `mut` | `mut data` | Mutable reference (`&mut T`) |
| `weak` | `weak parent_ref` | Non-owning weak reference |
| `drop` | `drop(handle)` | Explicit destructor invocation |
| `free` | `free(raw_ptr)` | Release allocated heap memory |
| `scope` | `scope { ... }` | Structured lifetime scope |
| `unsafe` | `unsafe { *ptr = 1 }` | Unchecked memory access block |

---

## 11. Collections & Data Structures

- `list`: Array / Slice (`list<Int>` or `[Int]`)
- `map`: Key-Value dictionary (`map<String, Int>` or `Map[String, Int]`)
- `set`: Unique set (`set<Int>` or `Set[Int]`)
- `vec`: Dynamic growable vector (`vec<Float>`)
- `queue`: FIFO queue (`queue<Task>`)
- `stack`: LIFO stack (`stack<Frame>`)
- `range`: Numerical sequence (`0..10`)

---

## 12. Pipeline & Functional Operators (`|>`)

```sora
// Data transformation pipeline
data := users
    |> filter(x => x.age >= 18)
    |> map(x => x.name)
    |> sort()
    |> take(10)
```

Supported pipeline transformers:
`pipe`, `map`, `filter`, `reduce`, `fold`, `find`, `some`, `every`, `none`, `take`, `skip`, `first`, `last`, `reverse`, `sort`, `group`, `flat`, `flatmap`, `unique`, `count`, `sum`, `min`, `max`.

---

## 13. Testing & Benchmarking

| Keyword | Grammar Example | Function / Meaning |
| :--- | :--- | :--- |
| `test` | `test "calculator add" { ... }` | Unit test suite |
| `expect` | `expect result == 42` | Test expectation assertion |
| `assert` | `assert x > 0` | Runtime validation |
| `bench` | `bench "matrix multiply" { ... }` | Performance benchmark suite |
| `mock` | `mock db` | Mock dependency generator |
| `fake` | `fake User()` | Test fixture factory |
| `fixture`| `fixture sample_data { ... }` | Test fixture setup |
| `skip` | `skip test "flaky network"` | Skip test case |

---

## 14. Web & Backend Standard DSL

```sora
// Declarative routing & HTTP verbs
get "/users" => users()
post "/user" => create_user(body)
put "/user/:id" => update_user(param("id"), body)
delete "/user/:id" => delete_user(param("id"))

// Server instantiation
serve(port: 8080) {
    middleware: [auth_guard, cors_middleware, logger]
}
```

Keywords & verbs: `route`, `get`, `post`, `put`, `patch`, `delete`, `head`, `options`, `middleware`, `request`, `response`, `header`, `cookie`, `session`, `auth`, `guard`, `query`, `param`, `body`, `json`, `form`, `file`, `upload`, `download`, `serve`, `listen`, `connect`, `socket`, `http`, `https`, `ws`, `websocket`.

---

## 15. Database & Query Operations

```sora
// Type-safe query pipeline
users := db.query("users")
    .where(age > 18)
    .order("created_at", desc)
    .limit(20)
    .all()

// Transaction block
transaction {
    db.insert("orders", new_order)
    db.update("inventory", item_id, dec(1))
    commit
}
```

Keywords: `db`, `sql`, `query`, `insert`, `update`, `delete`, `select`, `where`, `join`, `group`, `order`, `limit`, `offset`, `transaction`, `commit`, `rollback`, `migrate`, `model`, `schema`, `index`, `relation`.

---

## 16. CLI & System APIs

```sora
// CLI Argument Parsing
arg_port := env.get("PORT") ?? "8080"
path := dir.current() + "/config.json"
if file.exists(path) => print("Found config")
```

Keywords: `cmd`, `arg`, `args`, `env`, `stdin`, `stdout`, `stderr`, `exit`, `exec`, `process`, `signal`, `path`, `file`, `dir`, `read`, `write`, `open`, `close`, `watch`, `copy`, `move`, `delete`, `exists`, `system`, `platform`, `cpu`, `memory`, `time`, `clock`.

---

## 17. Metaprogramming & Compile-Time Evaluation

```sora
// Compile-time table generation
comptime {
    generate_sine_lookup_table()
}

// Automatic trait derivation
@derive(Debug, Clone, Serialize)
struct User {
    name String
    age Int
}
```

Keywords: `macro`, `meta`, `derive`, `gen`, `generate`, `compile`, `comptime`, `consteval`, `reflect`, `typeinfo`, `attribute`, `annotation`, `pragma`, `template`, `generic`, `where`, `constraint`.

---

## 18. Core Operator Reference Table

| Operator | Function / Name | Example in `.sora` |
| :--- | :--- | :--- |
| `:=` | Fast declaration & inference | `name := "Lutfi"` |
| `=` | Assignment | `count = 10` |
| `=>` | Fat arrow (functions / lambdas) | `fn add(a, b) => a + b` |
| `->` | Return type flow | `fn id() -> Int` |
| `\|>` | Functional pipeline | `data \|> filter(fn) \|> sort()` |
| `?.` | Safe navigation | `user?.profile?.email` |
| `??` | Fallback default (null coalesce) | `user?.name ?? "Unknown"` |
| `?` | Early error propagation | `data := fs::open("app.json")?` |
| `..` | Half-open numerical range | `0..10` |
| `..=` | Inclusive numerical range | `1..=10` |
| `::` | Namespace resolution | `std::io::print` |
| `<-` | Asynchronous actor send | `worker <- Task("Compute")` |
| `==` | Equality test | `if score == 100` |
| `!=` | Inequality test | `if status != 0` |
| `>=` / `<=` | Relational comparisons | `if age >= 18` |
| `&&` / `\|\|` / `!` | Logical AND, OR, NOT | `if is_admin && is_active` |
| `++` / `--` | Increment / Decrement | `count++` |
| `+=` / `-=` / `*=` / `/=` | Compound arithmetic | `total += price` |
| `**` | Exponentiation | `area = PI * r ** 2` |

---

## 🎯 Target Syntax Showcase

```sora
fn main() {
    users := db.users()
        |> filter(x => x.age >= 18)
        |> map(x => x.name)
        |> sort()

    each user in users => print(user)
}
```
