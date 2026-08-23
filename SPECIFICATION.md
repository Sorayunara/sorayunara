# 🌌 Sorayunara Language Specification (v1.0)

**Organization**: Sorayunara  
**Institution/Business**: Sorayunara  
**Programming Language**: Sorayunara  
**Compiler**: `sorayunara`  
**Source File Extension**: `.sora`  

*Formally specified for systems, application, embedded, AI/ML, and cloud-scale programming.*

---

## 📑 Table of Contents
1. [Lexical Structure](#1-lexical-structure)
2. [Keywords & Identifiers](#2-keywords--identifiers)
3. [Operators & Expressions](#3-operators--expressions)
4. [Variables & Mutability](#4-variables--mutability)
5. [Functions & Signatures](#5-functions--signatures)
6. [Type System](#6-type-system)
7. [Generics & Type Parameters](#7-generics--type-parameters)
8. [Structs & Aggregate Types](#8-structs--aggregate-types)
9. [Enums & Algebraic Data Types (ADT)](#9-enums--algebraic-data-types-adt)
10. [Traits & Interfaces](#10-traits--interfaces)
11. [Module & Package System](#11-module--package-system)
12. [Error Handling (`Result` & `Option`)](#12-error-handling-result--option)
13. [Memory Model & Ownership/Borrowing](#13-memory-model--ownershipborrowing)
14. [Built-in Concurrency & Async Runtime](#14-built-in-concurrency--async-runtime)
15. [Compile-time Evaluation & Optimizations](#15-compile-time-evaluation--optimizations)
16. [Application Binary Interface (ABI) & Foreign Function Interface (FFI)](#16-application-binary-interface-abi--foreign-function-interface-ffi)

---

## 1. Lexical Structure

### 1.1 Source Text & Encoding
Sorayunara source files must be encoded in **UTF-8** with the primary file extension **`.sora`** (legacy extensions `.ao`, `.nm`, and `.ae` are supported for backward compatibility).

### 1.2 Comments
- **Line comments**: Starts with `//` and continues until the end of line (`\n`).
- **Block comments**: Starts with `/*` and ends with `*/`. Nested block comments are supported.
- **Documentation comments**: Starts with `///` and attaches markdown docstrings to declarations for `sorayunara doc`.

```sorayunara
/// Computes energy calculation.
// This is a standard line comment
/* This is a 
   block comment */
```

### 1.3 Whitespace
Whitespace characters (Space `\x20`, Horizontal Tab `\t`, Line Feed `\n`, Carriage Return `\r`) separate tokens and are otherwise discarded except inside string and character literals.

---

## 2. Keywords & Identifiers

### 2.1 Reserved Keywords
| Category | Keywords |
| :--- | :--- |
| **Declarations** | `fn`, `let`, `mut`, `const`, `struct`, `enum`, `type`, `trait`, `impl` |
| **Control Flow** | `if`, `else`, `while`, `for`, `in`, `loop`, `break`, `continue`, `return`, `match` |
| **Memory Model** | `move`, `unsafe` |
| **Concurrency** | `async`, `await`, `task`, `spawn`, `chan` |
| **Modules** | `import`, `mod`, `pub` |
| **Literals** | `true`, `false`, `null` |

### 2.2 Identifiers
Identifiers must start with an ASCII letter (`a-z`, `A-Z`) or underscore (`_`), followed by any combination of letters, digits (`0-9`), or underscores.

---

## 3. Operators & Expressions

### 3.1 Operator Precedence (Highest to Lowest)
1. **Primary & Postfix**: Call `()`, Field `.`, Index `[]`
2. **Unary**: Negative `-`, Logical Not `!`, Reference `&`, Mutable Reference `&mut`
3. **Multiplicative**: `*`, `/`, `%`
4. **Additive**: `+`, `-`
5. **Relational**: `<`, `<=`, `>`, `>=`
6. **Equality**: `==`, `!=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||`
9. **Assignment**: `=`

---

## 4. Variables & Mutability

Variables are **immutable by default**. Mutable variables require the `mut` qualifier.

```sorayunara
let x: Int = 10         // Immutable
// x = 20               // Compile Error: cannot mutate immutable variable

let mut count: Int = 0  // Mutable
count = count + 1       // OK
```

---

## 5. Functions & Signatures

Functions are declared using `fn`. Functions that perform asynchronous work are declared using `async fn`.

```sorayunara
fn calculate_sum(a: Int, b: Int) -> Int {
    return a + b
}

async fn fetch_status(endpoint: &String) -> String {
    return http_get(endpoint)
}
```

---

## 6. Type System

Sorayunara possesses a sound, strong, static type system with static type inference.

### 6.1 Primitive Types
- `Int`: 64-bit signed integer (`i64`).
- `Float`: 64-bit IEEE-754 double precision float (`f64`).
- `Bool`: Boolean value (`true` or `false`).
- `String`: UTF-8 dynamic string.
- `Char`: Unicode scalar value (32-bit `char`).
- `Void`: Empty return type.

### 6.2 Compound & Reference Types
- `[T; N]`: Fixed-size Array.
- `&T`: Shared Immutable Reference (Borrow).
- `&mut T`: Exclusive Mutable Reference (Mutable Borrow).
- `Task<T>`: Concurrent asynchronous computation token.
- `Chan<T>`: Synchronous/Asynchronous typed communication channel.

---

## 7. Generics & Type Parameters

Generic functions and types parameterize over concrete types using angle brackets `<T>`.

```sorayunara
struct Pair<T, U> {
    first: T,
    second: U,
}

fn identity<T>(value: T) -> T {
    return value
}
```

---

## 8. Structs & Aggregate Types

Structs represent named heterogeneous record types.

```sorayunara
struct ServerConfig {
    host: String,
    port: Int,
    is_tls: Bool,
}

let cfg: ServerConfig = ServerConfig {
    host: "127.0.0.1",
    port: 8080,
    is_tls: true,
}
```

---

## 9. Enums & Algebraic Data Types (ADT)

Enums define sum types with optional data payloads per variant.

```sorayunara
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected(Int),
}

let state: ConnectionState = ConnectionState.Connecting
```

---

## 10. Traits & Interfaces

Traits specify behavioral contracts that concrete types implement.

```sorayunara
trait Printable {
    fn format(&self) -> String
}
```

---

## 11. Module & Package System

Modules encapsulate symbols. Modules are imported via dot notation: `import <path>.<module>`.

```sorayunara
import std.http
import std.math
import std.time

fn main() {
    let result = math.pow(2, 8)
    println(result)
}
```

### Package Layout (`sorayunara.toml`):
```toml
[package]
name = "my-service"
version = "0.1.0"

[dependencies]
http = "1.2.0"
```

---

## 12. Error Handling (`Result` & `Option`)

Sorayunara explicitly rejects unchecked exceptions. Errors are handled using `Option<T>` and `Result<T, E>`.

```sorayunara
fn divide(a: Int, b: Int) -> Result<Int, String> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

fn handle_opt(opt: Option<Int>) -> Int {
    return match opt {
        Some(val) => val,
        None => 0,
    }
}
```

---

## 13. Memory Model & Ownership/Borrowing

Sorayunara exposes **three explicit memory modes**: Managed (GC default), Owned (`move` semantics zero-cost), and Unsafe (`unsafe { ... }` raw pointers).

### 13.0 The Three Modes

| Mode | Keyword | Semantics | Use Case |
|:---|:---|:---|:---|
| **Managed** | *(default `let`)* | GC-managed immutable value; no ownership tracking required | Backend, business logic, rapid development |
| **Owned** | `move` | Single-owner, move semantics, `is_moved` tracked at compile time, no GC | CLI, systems, high-performance computing |
| **Unsafe** | `unsafe { ... }` | Raw pointer access (`*const T` / `*mut T`), unchecked operations | Embedded, FFI glue, low-level networking |

### 13.1 Syntax

```sorayunara
struct User {
    name: String,
}

// Managed mode (default)
let user = User::new()

// Owned mode — explicit ownership transfer
let owner = move user
// let again = user      // Compile Error: use of moved value

// Unsafe mode — access raw pointers
unsafe {
    let ptr: *mut Void = malloc(1024)
    ptr.write(value)     // unchecked low-level write
}
```

---

## 14. Built-in Concurrency & Async Runtime

Sorayunara includes native primitives for non-blocking concurrent execution:

- `spawn`: Dispatches a background task.
- `await`: Suspends until a task completes and retrieves the resolved value.
- `chan<T>()`: Creates a thread-safe message passing channel.

```sorayunara
async fn worker(id: Int) -> String {
    return "Task completed"
}

fn main() {
    let t: Task<String> = spawn worker(1)
    let res: String = await t
    println(res)
}
```

---

## 15. Compile-time Evaluation & Optimizations

The Sorayunara compiler applies multi-pass transformations:
- **Constant Folding**: Precomputes arithmetic and logical operations at compile time.
- **Dead Code Elimination (DCE)**: Prunes unreachable basic blocks after unconditional returns and jumps.
- **Peephole Optimization**: Eliminates identity operations (`x + 0`, `x * 1`).
- **HIR / MIR Lowering**: Linearizes control flow into structured basic blocks for register allocation.

---

## 16. Application Binary Interface (ABI) & FFI

### 16.0 Foreign Function Interface

Sorayunara dapat memanggil fungsi native dari C, C++, Rust (`extern "C"`), Python extension, dan system libraries melalui blok `extern`:

```sorayunara
// libm (math.h) — link dengan -lm
@link("m")
extern "C" {
    fn sqrt(x: Float) -> Float
    fn pow(base: Float, exp: Float) -> Float
}

// libc (string.h)
extern "C" {
    fn strlen(s: String) -> Int
    fn strcmp(a: String, b: String) -> Int
    fn malloc(size: Int) -> *mut Void
}

fn main() {
    let root: Float = sqrt(144.0)
    print(root)
}
```

---
*Official specification maintained by the Sorayunara Organization Core Development Team (sorayunara.org).*
