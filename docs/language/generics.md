# 🧬 Sorayunara Generics & Polymorphism

Sorayunara provides zero-cost monomorphized generics with trait-based constraint resolution.

---

## 1. Generic Functions
```sora
fn identity<T>(val: T) -> T {
    return val
}
```

---

## 2. Trait Bounds
```sora
trait Printable {
    fn to_str(&self) -> String
}

fn print_item<T: Printable>(item: &T) {
    print(item.to_str())
}
```
