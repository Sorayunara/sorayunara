# ⚡ Sorayunara Functions

Functions in Sorayunara are first-class citizens with support for short arrow expressions and explicit blocks.

---

## 1. Function Syntax
```sora
// Standard function declaration
fn add(a: Int, b: Int) -> Int {
    return a + b
}

// Expression-oriented short syntax
fn square(x: Int) -> Int => x * x
```

---

## 2. Parameter Passing & Borrowing
```sora
// By-value (takes ownership / copy for primitives)
fn process_val(val: Int) -> Int {
    return val * 2
}

// By-reference (borrowing)
fn compute_len(s: &String) -> Int {
    // Read-only borrow
    return 42
}
```
