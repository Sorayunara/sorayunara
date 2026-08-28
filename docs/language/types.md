# 🏷️ Sorayunara Type System

Sorayunara features a static, sound type system powered by Hindley-Milner type inference.

---

## 1. Primitive Types
- `Int`: 64-bit signed integer.
- `Float`: 64-bit IEEE 754 floating-point number.
- `Bool`: Boolean value (`true` / `false`).
- `Char`: Unicode scalar value.
- `String`: UTF-8 encoded heap-allocated string.

---

## 2. Algebraic Data Types (ADT) & Enums
```sora
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

---

## 3. Structs & Records
```sora
struct Point {
    x: Int,
    y: Int
}
```
