# 🎭 Sorayunara Formal Language Specification: Traits & Interfaces

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/traits.md`

---

## 1. Trait Declarations

A `trait` defines a contract of associated functions, methods, default implementations, and associated types that a type must satisfy.

```sora
trait Summary {
    // Required method
    fn summarize(self: &Self) -> String

    // Default implementation
    fn summarize_preview(self: &Self) -> String {
        summary = self.summarize()
        if summary.len() > 50 {
            summary.slice(0, 47) + "..."
        } else {
            summary
        }
    }
}
```

---

## 2. Implementing Traits

Traits are implemented for concrete types using `impl Trait for Type`:

```sora
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(self: &Article) -> String {
        "${self.title} by ${self.author}"
    }
}
```

---

## 3. Operator Overloading via Traits

Operators in Sorayunara map deterministically to standard traits:

| Operator | Trait Name | Required Method |
|---|---|---|
| `+` | `Add[Rhs, Output]` | `fn add(self: Self, rhs: Rhs) -> Output` |
| `-` | `Sub[Rhs, Output]` | `fn sub(self: Self, rhs: Rhs) -> Output` |
| `*` | `Mul[Rhs, Output]` | `fn mul(self: Self, rhs: Rhs) -> Output` |
| `/` | `Div[Rhs, Output]` | `fn div(self: Self, rhs: Rhs) -> Output` |
| `==`, `!=` | `Eq` | `fn eq(self: &Self, other: &Self) -> Bool` |
| `<`, `<=`, `>`, `>=` | `Ord` | `fn cmp(self: &Self, other: &Self) -> Ordering` |
| `[]` | `Index[Idx, Output]`| `fn index(self: &Self, idx: Idx) -> &Output` |

---

## 4. Trait Objects & Dynamic Dispatch (`dyn`)

When heterogeneous collections or dynamic polymorphism are required, trait objects using the `dyn` modifier enable runtime vtable dispatch:

```sora
fn draw_all(elements: [&dyn Drawable]) {
    for elem in elements {
        elem.draw() // Dispatched through vtable
    }
}
```

---

## 5. Built-in Derivable Traits

Using the `@derive(...)` attribute, the compiler automatically generates canonical implementations:
- `@derive(Clone, Debug, Eq, Ord, Hash, Serializable, Default)`
