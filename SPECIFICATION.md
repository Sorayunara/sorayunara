# 🌌 Sorayunara Core Language Specification (v1.0)

**Language Name**: Sorayunara  
**File Extension**: `.sora`  
**Core Motto**: **"Python Simplicity + Rust Safety + Zero Boilerplate"**  
**Compiler & Toolchain**: `sora` / `sorayunara`  

---

## 📑 1. Core Principles

1. **Minimal Keywords**: Keep grammar clean, expressive, and uncluttered.
2. **No Block Colons (`:`)**: Blocks are delimited by braces `{ ... }` or single `=` expressions.
3. **No Semicolon Burden**: Semicolons are optional and never required.
4. **Expression-Oriented**: Every block, `if`, and `match` evaluates to a value.
5. **Type Inference by Default**: The compiler automatically infers types via Hindley-Milner unification.
6. **Explicit Types Always Available**: Type annotations (`x: Int`) can be added when desired.
7. **Readability Over Symbol Overload**: Terse without degenerating into line noise.
8. **Rejection of Ambiguity**: Grammar must be deterministic and unambiguous (LALR(1) / PEG compatible).

---

## 📑 2. The 100-Keyword Architecture & Taxonomy

Sorayunara establishes a strict **100-keyword maximum** structured into:
- **Core Reserved (45 keywords)**: Always reserved (`fn`, `if`, `else`, `match`, `struct`, `enum`, `mut`, `const`, etc.).
- **Feature Reserved (30 keywords)**: Reserved in typings, concurrency & memory (`async`, `await`, `actor`, `spawn`, `channel`, etc.).
- **Contextual (25 keywords)**: Tokens only active at specific syntax positions (`as`, `from`, `where`, `defer`, `guard`, `derive`, etc.).

👉 *Complete detailed taxonomy and code examples for all 100 keywords*: **[`docs/KEYWORDS.md`](docs/KEYWORDS.md)**  
👉 *Ultra-Ergonomic Syntax & Operator Architecture*: **[`docs/GRAMMAR_V2.md`](docs/GRAMMAR_V2.md)**  
👉 *Complete 18-Domain Language Reference & Dictionary*: **[`docs/LANGUAGE_REFERENCE.md`](docs/LANGUAGE_REFERENCE.md)**

---

## 📑 3. Variables & Mutability

```sora
// Immutable variable (inferred, no 'let' required)
name = "Lutfi"
age = 21
active = true

// Compile-Time Constant
const version = "1.0"
const PI = 3.14159265359

// Mutable Variable
mut count = 0
count += 1

// Explicit Type Annotation (Optional)
port: Int = 8080
```

---

## 📑 3. Type System & Type Sugar

### 3.1 Primitive Types
- `Int`: 64-bit signed integer
- `Float`: 64-bit IEEE-754 floating point
- `Bool`: Boolean (`true` / `false`)
- `Char`: 32-bit Unicode character
- `String`: UTF-8 immutable string
- `Byte`: 8-bit unsigned integer (`u8`)

### 3.2 Collection Types
- `[Int]`: Array / Slice of integers
- `[String]`: Array of strings
- `Map[String, Int]`: Key-value hash map
- `Set[Int]`: Unique hash set

### 3.3 Sugar for Option & Result
- `T?`: Sugar for `Option<T>` (e.g. `String?`)
- `T!E`: Sugar for `Result<T, E>` (e.g. `Int!Error`)

```sora
name: String? = "Lutfi"
result: Int!Error = Ok(42)
```

---

## 📑 4. Functions & Expression Bodies

```sora
// Single-line expression function
fn add(a, b) = a + b

// Explicit typed function
fn add(a: Int, b: Int) -> Int = a + b

// Multi-line function (last expression is returned automatically)
fn calculate(x) {
    y = x * 2
    y + 10
}
```

---

## 📑 5. Lambdas & Closures

```sora
// Single parameter lambda
x => x * 2

// Multi parameter lambda
(a, b) => a + b

// Usage in higher-order functions
doubled = numbers.map(x => x * 2)
```

---

## 📑 6. Control Flow as Expressions

```sora
// Standard If-Else
if age >= 18 {
    println("adult")
} else {
    println("child")
}

// Expression form
status = if age >= 18 "adult" else "child"

// Multi-branch Expression
level = if score >= 90 {
    "A"
} else if score >= 80 {
    "B"
} else {
    "C"
}
```

---

## 📑 7. Loops & Iteration

```sora
// For-in loop
for x in numbers {
    println(x)
}

// Range iteration
for i in 0..10 {
    println(i)
}

// Infinite loop
loop {
    work()
    if should_stop() { break }
}

// While loop
while count < 10 {
    count += 1
}
```

---

## 📑 8. Structs & Object Instantiation

```sora
struct User {
    name: String,
    age: Int,
    is_active: Bool = true
}

// Named field instantiation
user = User(
    name: "Lutfi",
    age: 21
)

// Positional short-form constructor
user = User("Lutfi", 21)
```

---

## 📑 9. Methods & Implementation Blocks

```sora
impl User {
    fn greet(&self) = println("Hello {self.name}")
    
    fn is_adult(&self) -> Bool {
        self.age >= 18
    }
}

// Invocation
user.greet()
```

---

## 📑 10. Enums & Algebraic Data Types

```sora
enum Status {
    Active,
    Inactive,
    Banned
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

---

## 📑 11. Pattern Matching

```sora
label = match status {
    Status::Active   => "online"
    Status::Inactive => "offline"
    Status::Banned   => "blocked"
}

// Pattern matching with payload
value = match result {
    Result::Ok(val)  => val
    Result::Err(err) => 0
}
```

---

## 📑 12. Generics & Traits

```sora
// Generic function
fn first<T>(items: [T]) -> T = items[0]

// Generic struct
struct Box<T> {
    value: T
}

// Trait declaration
trait Show {
    fn show(&self)
}

// Trait implementation
impl Show for User {
    fn show(&self) = println("{self.name} ({self.age})")
}
```

---

## 📑 13. Null Safety & Safe Navigation

```sora
// Safe navigation
name = user?.name

// Safe fallback (Null/None coalescing)
display_name = user?.name ?? "Unknown"
```

---

## 📑 14. Error Propagation Operator (`?`)

```sora
fn load_user_profile(path: String) -> Profile!Error {
    data = read_file(path)?
    profile = parse_profile(data)?
    Ok(profile)
}
```

---

## 📑 15. Pipe Operator (`|>`)

For clean, readable data transformation pipelines:
```sora
result = numbers
    |> filter(x => x > 10)
    |> map(x => x * 2)
    |> sum()
```

---

## 📑 16. String Interpolation

```sora
// Variable interpolation
println("Hello {name}")

// Expression interpolation
println("Total: {price * quantity}")
```

---

## 📑 17. Destructuring

```sora
// Tuple destructuring
(name, age) = ("Lutfi", 21)

// Array destructuring with rest
[first, second, ...rest] = numbers

// Struct destructuring
{ name, age } = user
```

---

## 📑 18. Modules & Visibility

```sora
module auth

pub fn create_session(user: User) -> String {
    "token_123"
}

fn internal_secret() -> String {
    "secret"
}
```

Importing:
```sora
import auth
import auth.create_session
```

---

## 📑 19. Asynchronous Coroutines (`async` / `await`)

```sora
async fn fetch_data(url: String) -> String!Error {
    response = await http.get(url)?
    response.text()
}

// Awaiting task
data = await fetch_data("https://api.sorayunara.org/v1")
```

---

## 📑 20. Actor Concurrency Model

```sora
actor Counter {
    mut value: Int = 0

    on increment(amount: Int) {
        value += amount
    }

    on get -> Int {
        reply value
    }
}

// Spawning actor
counter = spawn Counter()

// Sending asynchronous message
counter <- increment(5)
```

---

## 📑 21. Foreign Function Interface (FFI)

```sora
extern "C" {
    fn puts(text: String) -> Int
    fn malloc(size: Int) -> Ptr
    fn free(ptr: Ptr)
}
```

---

## 🏛️ 22. Formal EBNF Grammar (Core Contract)

```ebnf
program         = { declaration } ;

declaration     = variable_decl
                | function_decl
                | struct_decl
                | enum_decl
                | trait_decl
                | impl_block
                | actor_decl
                | module_decl
                | import_decl ;

variable_decl   = [ "mut" | "const" ] identifier [ ":" type ] "=" expression ;

function_decl   = [ "async" ] [ "pub" ] "fn" identifier [ generic_params ]
                  "(" [ parameter_list ] ")" [ "->" type ]
                  ( "=" expression | block ) ;

block           = "{" { statement } [ expression ] "}" ;

if_expr         = "if" expression ( block | expression )
                  [ "else" ( block | if_expr | expression ) ] ;

for_expr        = "for" pattern "in" expression block ;

while_expr      = "while" expression block ;

loop_expr       = "loop" block ;

match_expr      = "match" expression "{" { match_arm } "}" ;
match_arm       = pattern "=>" ( expression | block ) [ "," ] ;

expression      = pipe_expr ;

pipe_expr       = binary_expr { "|>" binary_expr } ;

binary_expr     = unary_expr { binary_op unary_expr } ;

unary_expr      = [ "-" | "!" | "*" | "&" | "&mut" ] postfix_expr ;

postfix_expr    = primary_expr { call_suffix | member_suffix | safe_member_suffix | prop_suffix | index_suffix } ;

call_suffix     = "(" [ argument_list ] ")" ;
member_suffix   = "." identifier ;
safe_member_suffix = "?." identifier ;
prop_suffix     = "?" | "??" expression ;
index_suffix    = "[" expression "]" ;

primary_expr    = literal
                | identifier
                | "(" expression ")"
                | tuple_expr
                | array_expr
                | map_expr
                | struct_init
                | if_expr
                | match_expr
                | lambda_expr ;

lambda_expr     = ( identifier | "(" [ parameter_list ] ")" ) "=>" ( expression | block ) ;
```

---

## 🗺️ 23. 7-Phase Execution Plan

```
PHASE 1: Lexer ➔ Parser ➔ AST ➔ Basic Expressions
PHASE 2: Variables ➔ Functions ➔ If/Match ➔ For/While ➔ Collections
PHASE 3: Structs ➔ Enums ➔ Methods ➔ Modules
PHASE 4: Generics ➔ Traits ➔ HM Type Inference ➔ Option (T?) ➔ Result (T!E)
PHASE 5: Ownership ➔ Flow-Sensitive Borrow Checker ➔ Memory Safety
PHASE 6: Async Coroutines ➔ Actor Model ➔ Lock-Free Channels
PHASE 7: Macros ➔ Metaprogramming ➔ C/Rust FFI ➔ LLVM/WASM JIT
```
