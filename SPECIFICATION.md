# 🌌 Sorayunara Core Language Specification (v1.0)

**Organization**: Sorayunara  
**Language**: Sorayunara (`.sora`)  
**Design Philosophy**: **"Python simplicity + Rust safety + Zero boilerplate"**  
**Compiler**: `sora` / `sorayunara`  

---

## 🎯 The "Syntax Budget" Philosophy

Sorayunara adheres to an official design principle:
> **"If a concept can be written shorter without sacrificing readability or type safety, use the shorter syntax."**

| Feature | Python | Rust | Sorayunara (`.sora`) |
| :--- | :--- | :--- | :--- |
| **Variable** | `x = 10` | `let x = 10;` | `x = 10` (immutable by default) |
| **Mutable Variable** | `x = 10` | `let mut x = 10;` | `mut x = 10` |
| **Constant** | `PI = 3.14` | `const PI: f64 = 3.14;` | `const PI = 3.14` |
| **Simple Function** | `def add(a, b): return a + b` | `fn add(a: i64, b: i64) -> i64 { a + b }` | `fn add(a, b) = a + b` |
| **Function Block** | `def add(a, b):` | `fn add(a: i64, b: i64) -> i64 {` | `fn add(a, b) { a + b }` (implicit return) |
| **If Expression** | `status = "adult" if age >= 18 else "child"` | `let status = if age >= 18 { "adult" } else { "child" };` | `status = if age >= 18 "adult" else "child"` |
| **Loop** | `for item in items:` | `for item in items {` | `for item in items { print(item) }` |
| **Comprehension** | `[x*2 for x in nums if x>10]` | `nums.iter().filter(...).map(...)` | `[x * 2 for x in nums if x > 10]` |
| **Pipeline Chaining** | `map(filter(...))` | `nums.into_iter().filter().map()` | `nums.filter(x > 10).map(x * 2)` |
| **Safe Navigation** | `user.name if user else None` | `user.as_ref().map(\|u\| &u.name)` | `user?.name` |
| **Null/Option Fallback**| `user.name if user else "Anon"` | `user.map(\|u\| u.name).unwrap_or("Anon")` | `user?.name ?? "Anon"` |
| **Error Propagation** | `try / except` | `divide(10, 2)?` | `divide(10, 2)?` |
| **Struct Creation** | `User(name="Lutfi", age=21)` | `User { name: "Lutfi".into(), age: 21 }` | `user = User(name: "Lutfi", age: 21)` |

---

## 📑 Core Syntax Breakdown

### 1. Variables & Mutability
By default, variables are immutable and inferred:
```sora
// Immutable (no 'let' required)
name = "Lutfi"
age = 21

// Mutable
mut counter = 0
counter = counter + 1

// Compile-Time Constant
const PI = 3.14159265359

// Explicit Type Annotations (optional)
port: Int = 8080
title: String = "Sorayunara Core"
```

---

### 2. Functions & Expressions
Functions support single-line `=` expressions and multi-line blocks with implicit last-expression returns:
```sora
// Single-line expression function
fn add(a, b) = a + b
fn double(x: Int) -> Int = x * 2

// Multi-line function (last expression is returned automatically)
fn compute_metrics(base: Int, multiplier: Int) -> Int {
    normalized = base * 10
    normalized * multiplier
}
```

---

### 3. Control Flow as Expressions
`if`/`else` and `match` are always first-class expressions:
```sora
// Single-line if-expression
status = if age >= 18 "adult" else "child"

// Multi-line block if-expression
access_level = if role == "admin" {
    "full_access"
} else if role == "editor" {
    "write_access"
} else {
    "read_only"
}

// Expression pattern matching
grade = match score {
    90..=100 => "A"
    80..=89  => "B"
    70..=79  => "C"
    _        => "F"
}
```

---

### 4. Loops & Comprehensions
Clean, colon-free iteration and collection comprehensions:
```sora
// For loop
for user in users {
    print("Hello " + user.name)
}

// While loop
while counter > 0 {
    counter = counter - 1
}

// List Comprehension
even_squares = [x * x for x in numbers if x % 2 == 0]

// Pipeline Method Chaining
top_scores = scores
    .filter(x => x >= 80)
    .map(x => x + 5)
```

---

### 5. Structs & Object Instantiation
Concise aggregate data types without boilerplate:
```sora
struct User {
    name: String,
    age: Int,
    is_active: Bool = true // default value
}

// Instantiate with clean constructor syntax
user = User(name: "Lutfi", age: 21)
print(user.name)
```

---

### 6. Safe Navigation (`?.`), Fallback (`??`), and Error Propagation (`?`)
Eliminate null-pointer exceptions and deep try-catch nesting:
```sora
// Safe field access
company_name = employee?.company?.name

// Safe fallback / coalescing
display_name = user?.name ?? "Guest"

// Error propagation (returns early if Err/None)
fn load_config(path: String) -> Result<Config, IOError> {
    file = fs::open(path)?
    content = file.read_all()?
    config = json::parse(content)?
    Ok(config)
}
```

---

### 7. Algebraic Enums & Pattern Matching
Type-safe tagged unions with zero memory overhead:
```sora
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

value = match result {
    Result::Ok(val) => val
    Result::Err(err) => {
        print("Encountered error: " + err)
        0
    }
}
```

---

### 8. Concurrency: Coroutines & Lock-Free Channels
Lightweight actors and asynchronous pipelines built into the standard library:
```sora
struct TaskMessage {
    id: Int,
    payload: String
}

async fn worker(ch: Channel<TaskMessage>) {
    let msg = ch.recv()
    match msg {
        Option::Some(data) => print("Processed: " + data.payload),
        Option::None => print("Channel closed")
    }
}

fn main() -> Int {
    ch: Channel<TaskMessage> = channel::new(1024)
    
    spawn async {
        worker(ch)
    }

    ch.send(TaskMessage(id: 1, payload: "Run LLVM Pipeline"))
    return 0
}
```

---

## 🏛️ Summary of Syntax Rules

1. **Clean Declarations**: No mandatory `let` keyword for declarations (`x = 10`, `mut y = 20`, `const Z = 30`).
2. **Expression First**: Blocks and control structures resolve to their trailing expression.
3. **Smart Inferred Types**: Write types only when annotating public APIs or resolving ambiguity.
4. **Safety by Default**: Immutable by default, safe navigation `?.`, fallback `??`, and `?` error propagation.
5. **No Colon/Semicolon Noise**: Semicolons are optional and colons are only used for explicit type annotations.
