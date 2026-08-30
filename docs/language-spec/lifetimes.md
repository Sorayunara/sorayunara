# ⏳ Sorayunara Formal Language Specification: Lifetimes & Flow-Sensitive Analysis

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/lifetimes.md`

---

## 1. Concept of Lifetimes

A **lifetime** is the span of Control Flow Graph (CFG) basic blocks during which a reference is guaranteed to point to valid, initialized memory.

Sorayunara guarantees that **dangling pointers and use-after-free conditions are statically impossible**.

---

## 2. Non-Lexical Lifetimes (NLL)

Lifetimes in Sorayunara are flow-sensitive and fine-grained (Non-Lexical Lifetimes). A borrow ends at its **last point of use**, rather than at the lexical closing brace `}`:

```sora
mut buffer = [1, 2, 3]

ref = &buffer[0]
println(*ref) // Last use of `ref` occurs here!

// Borrows ends here in NLL (even inside same scope)
buffer.push(4) // VALID in Sorayunara (NLL)
```

---

## 3. Lifetime Elision Rules

To maintain high ergonomics ("Python Simplicity"), lifetime parameters are automatically inferred in function signatures by 3 standard elision rules:

1. **Input Elision**: Each omitted lifetime in the parameter list is assigned a distinct lifetime parameter:
   ```sora
   // fn parse(input: &str) -> Token
   // desugars to:
   // fn parse['a](input: &'a str) -> Token
   ```
2. **Single Parameter Rule**: If there is exactly one input lifetime parameter, that lifetime is assigned to all omitted output lifetimes:
   ```sora
   // fn first_word(s: &str) -> &str
   // desugars to:
   // fn first_word['a](s: &'a str) -> &'a str
   ```
3. **Method `&self` Rule**: If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all omitted output lifetimes.

---

## 4. Explicit Lifetime Annotations

When multiple references have interdependent return lifetimes, explicit lifetime parameters are specified:

```sora
fn longest['a](x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 4.1 The `'static` Lifetime
The `'static` lifetime denotes data that lives for the entire duration of program execution (such as string literals and global constants):

```sora
const APP_NAME: &'static str = "Sorayunara"
```
