# 📋 Sorayunara Formal Language Specification: Statements

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/statements.md`

---

## 1. Statement Categories

Sorayunara distinguishes between **binding declarations**, **control statements**, and **deferral blocks**. Statements do not evaluate to values (yielding unit type `()`).

---

## 2. Variable Bindings

### 2.1 Immutable Bindings (Default)
In Sorayunara, bindings are immutable by default:
```sora
x = 42
name := "Sorayunara" // Walrus shorthand declaration
```

### 2.2 Mutable Bindings (`mut`)
Mutability must be explicitly declared at definition:
```sora
mut counter = 0
counter += 1

let mut buffer = Buffer.new(1024)
```

### 2.3 Constants (`const`)
Constants are evaluated at compile time and must be pure expressions:
```sora
const MAX_CONCURRENT_TASKS = 64
const PI: Float = 3.141592653589793
```

---

## 3. Control Flow Statements

### 3.1 `guard` Statement
Guarantees preconditions. If the condition evaluates to `false`, the mandatory `else` block MUST diverge (`return`, `break`, `continue`, or `panic`):
```sora
fn process_payment(amount: Int, account: Account?) {
    guard account is Some(acc) else {
        return Err("Invalid account")
    }
    guard amount > 0 else {
        return Err("Amount must be positive")
    }
    acc.deduct(amount)
}
```

### 3.2 `defer` Statement
Defers execution of a statement or block until the enclosing scope exits (via normal return, error propagation, or panic). Deferred statements run in **LIFO (Last-In, First-Out)** order:

```sora
fn read_config(path: String) -> String!Error {
    file = fs.open(path)?
    defer file.close() // Executed on any function exit

    lock = mutex.acquire()
    defer lock.release()

    file.read_to_string()
}
```

### 3.3 Iteration Statements (`while`, `for .. in`, `loop`)
```sora
// While loop
while is_running {
    poll_events()
}

// For-in iterator loop
for item in collection {
    println(item)
}

// Range loops
for i in 0..10 { // 0 to 9
    process(i)
}

for i in 0..=10 { // 0 to 10 inclusive
    process(i)
}
```

---

## 4. Termination & Divergence Statements

- `return [expr]`: Exits the current function, returning `expr` or unit `()`.
- `break [expr]`: Exits the current loop, optionally passing a value to a `loop` expression.
- `continue`: Skips the remainder of the current loop iteration.
- `throw expr`: Raises an unrecoverable panic or throws typed exceptional failure.
