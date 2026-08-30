# 📐 Sorayunara Formal Language Specification: Generics & Monomorphization

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/generics.md`

---

## 1. Parametric Polymorphism

Sorayunara supports compile-time parametric polymorphism across functions, structs, enums, traits, and type aliases. Generic parameters are enclosed in square brackets `[...]`.

```sora
// Generic Function
fn swap[T](a: &mut T, b: &mut T) {
    temp = *a
    *a = *b
    *b = temp
}

// Generic Struct
struct Stack[T] {
    elements: [T],
}

impl[T] Stack[T] {
    fn new() -> Stack[T] => Stack { elements: [] }
    fn push(mut self, item: T) {
        self.elements.push(item)
    }
    fn pop(mut self) -> T? {
        self.elements.pop()
    }
}
```

---

## 2. Trait Bounds & Where Clauses

Type parameters can be constrained using trait bounds with colon `:` syntax or explicit `where` clauses:

```sora
// Inline Bound
fn print_debug[T: Display + Debug](item: T) {
    println("${item.debug_format()}")
}

// Where Clause Syntax
fn serialize_payload[T, E](data: T) -> [Byte]!E
where
    T: Serializable + Clone,
    E: Error + From[IoError],
{
    serializer.encode(data)
}
```

---

## 3. Higher-Kinded Types (HKT)

Sorayunara enables abstraction over type constructors with wildcard kind annotations `F[_]`:

```sora
trait Functor[F[_]] {
    fn fmap[A, B](self: F[A], transform: fn(A) -> B) -> F[B]
}

trait Monad[M[_]]: Functor[M] {
    fn pure[A](value: A) -> M[A]
    fn flat_map[A, B](self: M[A], transform: fn(A) -> M[B]) -> M[B]
}
```

---

## 4. Compile-Time Monomorphization

### 4.1 Static Specialization
The compiler monomorphizes all generic instantiations into concrete machine instructions during the HIR/MIR lowering phase ([`monomorphizer.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/monomorphizer.rs)).

$$\text{Stack}[\text{Int}] \implies \text{Stack\_\_Int} \quad (\text{Dedicated Specialized Struct})$$
$$\text{Stack}[\text{String}] \implies \text{Stack\_\_String} \quad (\text{Dedicated Specialized Struct})$$

### 4.2 Zero Runtime Overhead
- No vtable pointer indirection or boxing for concrete generic invocations.
- Inlining optimizations apply seamlessly across monomorphized functions.
