# 🔤 Sorayunara Syntax Reference

Sorayunara combines the readable, concise elegance of Python with the strict type safety and zero-cost performance of Rust.

---

## 1. Variables & Bindings
```sora
// Immutable variable (inferred type)
let x = 100

// Mutable variable
let mut count = 0
count = count + 1

// Explicit type annotation
let name: String = "Sorayunara"

// Declaration with walrus operator
total := 500
```

---

## 2. Control Flow & Branching
```sora
// If / Else If / Else
if score >= 90 {
    print("Grade: A")
} else if score >= 80 {
    print("Grade: B")
} else {
    print("Grade: C")
}

// Pattern Matching
let description = match status_code {
    200 => "OK",
    404 => "Not Found",
    500 => "Server Error",
    _   => "Unknown"
}
```

---

## 3. Iteration
```sora
let mut i = 0
while i < 10 {
    print("Index: ", i)
    i = i + 1
}
```
