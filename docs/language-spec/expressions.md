# 💡 Sorayunara Formal Language Specification: Expressions

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/expressions.md`

---

## 1. Expression-Oriented Nature

In Sorayunara, nearly all syntactic forms evaluate to a typed value. Blocks `{ ... }`, conditionals `if / else`, pattern matches `match`, loops `loop`, and try-expressions all possess evaluation values and types.

---

## 2. Block Expressions

A block `{ stmt1; stmt2; expr }` evaluates sequentially. The evaluation value of the block is the value of its trailing expression (without trailing semicolon). If the block terminates with a statement, its evaluated value is unit `()`.

```sora
value = {
    x = 10
    y = 20
    x * y // Value of the block is 200
}
```

---

## 3. Control Flow Expressions

### 3.1 `if` / `else` Expressions
Both branches of an `if / else` expression MUST unify to the same common supertype $\tau$:

```sora
status = if score >= 90 { "Distinction" } else { "Pass" }
```

When an `else` branch is omitted, the `if` expression evaluates to `Option[T]` (`Some(v)` if true, `None` if false):
```sora
maybe_val = if condition { 42 } // Type is Int?
```

### 3.2 `match` Expressions
A `match` expression performs structural pattern matching over a subject. It is strictly checked for exhaustiveness at compile-time:

```sora
message = match http_status {
    200 => "OK",
    404 => "Not Found",
    500..=599 => "Server Error",
    _ => "Unknown Status",
}
```

### 3.3 `loop` Expressions with `break` Values
An infinite `loop` expression can produce a value when exiting via `break <expr>`:

```sora
found = loop {
    item = queue.pop()
    if item.is_valid() {
        break item // Evaluates loop to item
    }
}
```

---

## 4. Pipeline & Functional Expressions

### 4.1 Pipeline Operator (`|>`)
The pipeline operator threads the value on the left-hand side as the first argument (or explicit placeholder `_`) of the right-hand function call:

```sora
// a |> f(b) desugars into f(a, b)
// a |> f(b, _, c) desugars into f(b, a, c)

result = "  sorayunara  "
    |> string.trim
    |> string.to_uppercase
    |> string.repeat(2)
```

### 4.2 Lambda / Anonymous Function Expressions
```sora
add = (a, b) => a + b
square = x => x * x
complex = (x: Int) -> Int => {
    mut temp = x * 2
    temp += 10
    temp
}
```

---

## 5. Operators for Option & Result

| Syntax | Name | Semantic Description |
|---|---|---|
| `expr?` | Error Propagation | Unwraps `Ok(v)`/`Some(v)`; returns early with `Err(e)`/`None` on failure. |
| `expr ?? fallback` | Null/None Coalescing | Returns inner value if `Some(v)`, otherwise evaluates and returns `fallback`. |
| `expr?.field` | Optional Chaining | Evaluates `field` if `expr` is not `None`; otherwise yields `None`. |
| `expr!` | Forced Unwrap (Unsafe)| Unwraps value; triggers runtime panic if `None` or `Err`. |

---

## 6. Spawn & Async Expressions

```sora
// Spawns a cooperative green task running concurrently
future_handle = spawn fetch_user_data(user_id)

// Awaits resolution of an asynchronous computation
user = await future_handle
```
