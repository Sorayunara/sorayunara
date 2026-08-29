# 🌌 Sorayunara (.sora) Syntax & Language Architecture v3
**Proposal Document**: `Sorayunara_Syntax_Architecture_Proposal_v3.pdf`  
**Specification Version**: 3.0.0-LTS · **Extension**: `.sora`  
**Core Directive**: Rich, consistent, memory-safe, and realistic for compiler lowering.

---

## 🏛️ 1. Design Principles (Syntax v3)

1. **Satu Cara Utama (Canonical Form)**: One predictable canonical syntax for common patterns; shorthands act as ergonomic layers without conflicting semantics.
2. **Explicit When Public**: Public API boundaries are encouraged to specify explicit types and visibility (`pub`), even with aggressive Hindley-Milner type inference.
3. **Expression-Oriented**: `if`, `match`, blocks, and pipelines (`|>`) evaluate to values.
4. **Ownership-Aware**: Shorthands never obscure critical `move`, shared borrow (`&`), or exclusive borrow (`&mut`) semantics.
5. **Async Tanpa Magic**: `await` and `spawn` maintain explicit cancellation tokens and lifetime boundaries.
6. **Library Over Keyword**: High-level DSLs (HTTP routing, SQL queries, CLI parsing, JSON, Logging) live in the standard library (`std.*`), keeping the core compiler grammar lean and robust.
7. **Tooling-First**: Formatter, Language Server Protocol (LSP), diagnostics, and syntax highlighting share the single canonical grammar contract.

---

## 📐 2. Syntax v3 Core Features

### 2.1 Destructuring with Pattern Guard
```sora
let { id, name, age } = user;

match user {
    { age, name } if age >= 18 => greet(name),
    _ => deny(),
}
```

### 2.2 Let-Else for Invariant Enforcement
```sora
let user = find_user(id) else {
    return err "user not found"
};

let config = load_config() else {
    return err "invalid configuration"
};
```

### 2.3 Safe Navigation on Collections & Nested Structures
```sora
let first = users?[0];
let city = users?[0]?.address?.city ?? "Unknown";
let value = matrix?[row]?[col];
```

### 2.4 Generics with Where Clauses & Rich Enum Cases
```sora
fn max<T>(a: T, b: T) -> T 
where T: Ord {
    if a > b => a else b
}

enum HttpEvent {
    Request { method: Method, path: String },
    Response { status: Int, body: Bytes },
    Closed
}
```

### 2.5 Lazy Iterator Chains & Pipelines
```sora
let names = users
    |> iter
    |> filter(.active)
    |> map(.name)
    |> take(100)
    |> collect();
```

### 2.6 Defer with Lexical Scope
```sora
fn transaction() -> Result<(), Error> {
    let tx = db.begin()?;
    defer tx.rollback_if_open();
    
    update_account(tx)?;
    tx.commit()?
}
```

---

## ⚖️ 3. Operator Precedence Hierarchy (11 Levels)

| Level | Operator Category | Operators | Associativity |
|---|---|---|---|
| **1 (Highest)** | Postfix | `()`, `[]`, `.`, `?.` | Left-to-right |
| **2** | Unary | `!`, `-`, `&`, `&mut`, `*` | Right-to-left |
| **3** | Multiplicative | `*`, `/`, `%` | Left-to-right |
| **4** | Additive | `+`, `-` | Left-to-right |
| **5** | Range | `..`, `..=` | Non-associative |
| **6** | Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` | Left-to-right |
| **7** | Logical AND | `&&` | Left-to-right |
| **8** | Logical OR | `\|\|` | Left-to-right |
| **9** | Null Coalescing | `??` | Right-to-left |
| **10** | Pipeline | `\|>` | Left-to-right |
| **11 (Lowest)** | Assignment | `=`, `+=`, `-=`, `*=`, `/=`, `??=` | Right-to-left |
