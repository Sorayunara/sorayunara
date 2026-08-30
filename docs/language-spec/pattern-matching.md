# 🎯 Sorayunara Formal Language Specification: Pattern Matching & Destructuring

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/pattern-matching.md`

---

## 1. Pattern Matching Invariants

1. **Static Exhaustiveness**: Every `match` expression must cover 100% of the value domain of the matched type. Missing a case emits compilation error `E0312: Non-exhaustive pattern match`.
2. **Reachability Check**: Any pattern arm rendered unreachable by preceding patterns emits warning/error `E0313: Unreachable pattern`.
3. **Deep Structural Destructuring**: Patterns can recursively inspect nested structs, tuples, slices, and enums.

---

## 2. Pattern Forms

### 2.1 Literal & Identifier Patterns
```sora
match status {
    200 => "OK",
    400 => "Bad Request",
    other => "Received ${other}",
}
```

### 2.2 Range Patterns
```sora
match age {
    0..=12 => "Child",
    13..=19 => "Teenager",
    20..=64 => "Adult",
    _ => "Senior",
}
```

### 2.3 Destructuring Structs & Tuples
```sora
struct Point { x: Int, y: Int }

match point {
    Point { x: 0, y: 0 } => "Origin",
    Point { x, y: 0 } => "On X axis at ${x}",
    Point { x: 0, y } => "On Y axis at ${y}",
    Point { x, y } => "Point at (${x}, ${y})",
}
```

### 2.4 Destructuring Enums & ADTs
```sora
enum Message {
    Quit,
    Move { x: Int, y: Int },
    Write(String),
    ChangeColor(Byte, Byte, Byte),
}

match msg {
    Message.Quit => shutdown(),
    Message.Move { x, y } => relocate(x, y),
    Message.Write(text) => log(text),
    Message.ChangeColor(r, g, b) => set_color(r, g, b),
}
```

### 2.5 Slice / Array Patterns
```sora
match command_tokens {
    ["exit"] => exit_app(),
    ["load", filename] => load_file(filename),
    ["set", key, value] => set_config(key, value),
    [head, ..tail] => handle_varargs(head, tail),
    [] => show_prompt(),
}
```

---

## 3. Pattern Guards (`if <condition>`)

Arms may append conditional boolean guard predicates:

```sora
match number {
    n if n % 15 == 0 => "FizzBuzz",
    n if n % 3 == 0 => "Fizz",
    n if n % 5 == 0 => "Buzz",
    n => "${n}",
}
```

---

## 4. `is` Operator for Pattern Testing

The `is` operator performs inline pattern checks with automatic flow-sensitive type narrowing:

```sora
if response is Ok(data) {
    // `data` is automatically bound and available inside this scope
    process(data)
}
```
