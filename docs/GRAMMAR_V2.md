# 🌌 Sora Grammar v2 — Ultra-Ergonomic Systems & Backend Language

**Specification Status**: Approved Reference Standard  
**File Extension**: `.sora`  
**Philosophy**: **"Shorter than Python, Safer than Rust, Zero Boilerplate"**  

---

## 🎯 1. Core Design Axioms

1. **Grammar Compression**: Express complex systems logic in fewer characters without degrading semantic clarity.
2. **50–80 Focused Keywords**: Keep the reserved keyword vocabulary compact, composable, and clean.
3. **Rich Operator Ergonomics**: Leverage powerful operators (`|>`, `?.`, `??`, `?`, `:=`, `=>`, `<-`) to eliminate boilerplate method calls.
4. **Frictionless Type Inference**: Let the compiler handle Hindley-Milner typing while allowing optional explicit annotations.

---

## 📑 2. Detailed Syntax Categories

---

### Category A: Variables & Declarations (`:=`, `=`, `mut`, `const`)

```sora
// Short declaration syntax (Go/Sora style)
name := "Lutfi"
age := 20

// Standard declaration
title = "Sorayunara Core"

// Mutable binding
mut counter := 0
counter += 1

// Compile-Time Constant
const MAX_BUFFER = 4096
const PI = 3.14159265
```

---

### Category B: Functions & Arrow Expressions (`=>`)

```sora
// Single-line fat-arrow function
fn add(a, b) => a + b
fn square(x) => x * x
fn greet(name) => "Hello, " + name

// Explicit typed signature
fn multiply(a: Int, b: Int) -> Int => a * b

// Multi-line block with implicit trailing return
fn calculate(x) {
    y := x * 2
    y + 10
}
```

---

### Category C: Control Flow & Arrow Conditionals

```sora
// Single-line arrow condition
if age >= 18 => print("adult") else => print("young")

// Multi-line block if-else
status := if age >= 18 {
    "adult"
} else {
    "minor"
}

// Guard clause with early return
guard user.is_authenticated else => return Err("Unauthorized")
```

---

### Category D: Loops & Ranges (`for`, `each`, `..`, `while`)

```sora
// For-in iteration
for x in nums {
    print(x)
}

// Single-line arrow loop
for x in nums => print(x)

// Range loop (half-open 0..10 and inclusive 0..=10)
for i in 0..10 => print(i)

// Shorthand each iteration
each x nums => print(x)

// While loop
while x < 10 => x += 1

// Infinite loop with break
loop {
    if is_done() => break
}
```

---

### Category E: Collections, Maps & Pipelines (`|>`)

```sora
// Lists & Arrays
nums := [1, 2, 3, 4, 5]

// Map literals
user := { name: "Lutfi", age: 20 }
print(user.name)

// Data Transformation Pipeline
result := nums
    |> filter(x => x > 2)
    |> map(x => x * 2)
    |> sort()

// Compact pipeline processing
data := users
    |> filter(u => u.age >= 18)
    |> map(u => u.name)
    |> join(", ")
```

---

### Category F: Structs & Methods

```sora
// Clean struct without redundant punctuation
struct User {
    name: String
    age: Int
    is_admin: Bool = false
}

// Instantiation (Named or Positional)
u1 := User(name: "Lutfi", age: 20)
u2 := User("Lutfi", 20)

// Methods
impl User {
    fn display(&self) => print("{self.name} ({self.age})")
}
```

---

### Category G: Pattern Matching & Error Handling (`match`, `?`, `??`, `try/catch`)

```sora
// Clean pattern match
label := match status {
    0 => "zero"
    1 => "one"
    x => "other: {x}"
}

// Safe Field Access & Fallback
display_name := user?.name ?? "Unknown"

// Error Propagation Operator (?)
data := get_data()?

// Shorthand Try-Catch
try fetch_remote() catch err => print("Failed: {err}")
```

---

### Category H: Asynchronous Coroutines & Parallel Execution

```sora
// Spawn async task directly
spawn worker_task(1, ch)

// Await async task
data := await fetch(url)

// Structured parallel execution block
a, b := parallel {
    fetch_user_profile(id),
    fetch_user_metrics(id)
}
```

---

### Category I: Actor Model & Message Channels

```sora
actor Counter {
    mut value := 0

    on increment(n: Int) {
        value += n
    }

    on get -> Int {
        reply value
    }
}

// Spawn and send asynchronous message
c := spawn Counter()
c <- increment(5)
```

---

## ⚡ 3. Sora Official Operator Dictionary

| Operator | Name | Purpose | Example |
| :--- | :--- | :--- | :--- |
| `:=` | **Short Decl** | Declare and infer variable | `name := "Lutfi"` |
| `=>` | **Fat Arrow** | Expression function / Lambda / Match arm | `fn add(a, b) => a + b` |
| `\|>` | **Pipeline** | Pipe preceding value as first argument | `data \|> filter(fn) \|> sort()` |
| `?.` | **Safe Access**| Safe optional/nullable field navigation | `user?.profile?.email` |
| `??` | **Null Coalesce**| Provide fallback default value | `user?.name ?? "Guest"` |
| `?` | **Error Try** | Propagate error early if `Err`/`None` | `file := fs::open("app.json")?` |
| `<-` | **Actor Send** | Send asynchronous actor message | `actor_ref <- Message("ping")` |
| `..` | **Range** | Half-open integer range | `0..10` (0 to 9) |
| `..=` | **Range Inc**| Inclusive integer range | `1..=10` (1 to 10) |
| `::` | **Namespace**| Module / Enum variant path | `std::http::Client` |
| `->` | **Return Arrow**| Function return type annotation | `fn total() -> Int` |
| `&` | **Borrow** | Immutable borrow reference | `&data` |
| `mutref` | **Mut Borrow**| Exclusive mutable borrow reference | `mutref data` |

---

## 📊 4. Syntax Comparison: Python vs Rust vs Sora v2

```sora
// 1. Python:
// numbers = [1, 2, 3, 4, 5]
// result = [x * 2 for x in numbers if x > 2]

// 2. Rust:
// let numbers = vec![1, 2, 3, 4, 5];
// let result: Vec<i32> = numbers.into_iter().filter(|&x| x > 2).map(|x| x * 2).collect();

// 3. Sora v2:
nums := [1, 2, 3, 4, 5]
result := nums |> filter(x => x > 2) |> map(x => x * 2)
```

---

## 🗺️ 5. Implementation Roadmap for Grammar v2 Parser

1. **Lexer Tokenizer**: Support `:=`, `=>`, `|>`, `?.`, `??`, `<-`, `..`, `..=`.
2. **Parser Productions**: Add expression-bodied functions (`fn id(p) => expr`), inline `if cond => expr`, and pipeline chaining.
3. **Type Checker Unification**: Enhance Hindley-Milner type inference for struct record shorthand `{ a: 1, b: 2 }`.
4. **Formatter & LSP**: Update AST formatter and VS Code grammar syntax tree.
