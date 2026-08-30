# 🧬 Sorayunara Formal Type System Specification

**Document Version**: 1.0.0  
**Specification Status**: Normative Systems-Level Reference Standard  
**File Identifier**: `docs/language-spec/types.md`  
**Theoretical Basis**: System $F_\omega$ + Damas-Hindley-Milner Algorithm $\mathcal{W}$ + Affine Type Subtyping

---

## 1. Mathematical Foundations & Inference Engine

The Sorayunara type system $(\mathcal{T}, \le, \vdash)$ provides static type safety, zero-cost abstractions, and total memory safety without garbage collection pauses.

### 1.1 Typing Judgment & Unification
A typing environment $\Gamma$ maps identifiers to type schemes $\sigma \in \mathcal{T}$. The fundamental bidirectional typing judgment:

$$\Gamma \vdash e : \tau$$

Unification of two types $\tau_1 \doteq \tau_2$ computes the most general unifier (mgu) $\theta$:

$$\frac{\Gamma \vdash e_1 : \tau_1 \to \tau_2 \quad \Gamma \vdash e_2 : \tau_1}{\Gamma \vdash e_1(e_2) : \tau_2} \quad [\text{App}]$$

$$\frac{\Gamma, x : \tau_1 \vdash e : \tau_2}{\Gamma \vdash \lambda x. e : \tau_1 \to \tau_2} \quad [\text{Abs}]$$

---

## 2. Primitive Scalar Types & Memory Layout

Every primitive in Sorayunara has a fixed, platform-independent bitwidth and strict memory alignment conforming to System V AMD64 and ARM64 AAPCS ABIs.

| Primitive Type | Bitwidth | Alignment | Value Range / Representation | Memory Layout |
|---|---|---|---|---|
| `Byte` / `UInt8` | 8-bit | 1 byte | $0 \dots 255$ | Unsigned 8-bit byte |
| `Int8` | 8-bit | 1 byte | $-128 \dots 127$ | Two's complement signed |
| `Int16` / `UInt16` | 16-bit | 2 bytes | $-32,768 \dots 32,767$ / $0 \dots 65,535$ | 16-bit Little-Endian |
| `Int32` / `UInt32` | 32-bit | 4 bytes | $-2^{31} \dots 2^{31}-1$ / $0 \dots 2^{32}-1$ | 32-bit Little-Endian |
| `Int` / `Int64` | 64-bit | 8 bytes | $-2^{63} \dots 2^{63}-1$ (Default Integer) | 64-bit Two's complement |
| `UInt64` / `USize` | 64-bit | 8 bytes | $0 \dots 2^{64}-1$ (Target pointer width) | 64-bit Unsigned Word |
| `Float32` | 32-bit | 4 bytes | Single-precision IEEE-754 ($\approx 7$ dec digits) | 1 sign, 8 exp, 23 mantissa |
| `Float` / `Float64` | 64-bit | 8 bytes | Double-precision IEEE-754 ($\approx 15$ dec digits)| 1 sign, 11 exp, 52 mantissa |
| `Bool` | 8-bit | 1 byte | `true` (0x01) or `false` (0x00) | Single byte logical state |
| `Char` | 32-bit | 4 bytes | Unicode Scalar (`U+0000`..`U+10FFFF` exc. surrogates)| 4-byte UCS-4 UTF-32 code point |
| `()` (Unit) | 0-bit | 1 byte | Singleton unit value `()` | Zero-size type (ZST) |
| `Never` (`!`) | 0-bit | 1 byte | Empty set $\emptyset$ (Diverging functions/panic) | Bottom type $\bot$ |

---

## 3. Compound Data Structures

### 3.1 Tuples (Anonymous Product Types)
Fixed-length heterogeneous collections allocated contiguously on the stack:
```sora
coords: (Int, Float, Bool) = (10, 3.14, true)
x = coords.0
y = coords.1
```
- **Memory Layout**: Evaluated with standard struct padding to maintain field alignment. Zero overhead.

### 3.2 Fixed-Size Arrays (`[T; N]`)
Statically-sized, homogeneous elements stored inline in the stack frame or parent structure:
```sora
buffer: [Byte; 1024] = [0u8; 1024]
size = buffer.len() // Compile-time constant 1024
```
- **Memory Layout**: Contiguous array of $N \times \text{sizeof}(T)$ bytes without indirection pointers.

### 3.3 Dynamic Arrays & Slices (`[T]`)
A **slice** `&[T]` is a dynamically-sized fat pointer consisting of two 64-bit words:
$$\text{FatPointer} = (\text{ptr}: \text{*const } T, \text{len}: \text{USize}) \implies 16 \text{ bytes}$$

```sora
slice: &[Int] = &numbers[2..5]
```
- Bounds checks are verified at compile time when indices are static constants, and inserted as single conditional branches in JIT/LLVM codegen for dynamic indices.

---

## 4. Structures (Named Product Types)

Sorayunara supports three structural layouts:

```sora
// 1. Named Struct
struct PacketHeader {
    magic: UInt32,
    sequence: UInt32,
    payload_len: UInt16,
    flags: Byte,
}

// 2. Tuple Struct
struct Point3D(Float, Float, Float)

// 3. Unit Struct (Zero-Sized Type - ZST)
struct EmptyMarker
```

### 4.1 Memory Layout & Alignment
- Fields are ordered to satisfy alignment constraints: $\text{offset}(f_i) \equiv 0 \pmod{\text{align}(f_i)}$.
- Special representation attributes:
  - `@repr(C)`: Enforces standard ANSI C ABI struct layout and padding.
  - `@repr(packed)`: Eliminates padding bytes between fields (alignment = 1 byte).

---

## 5. Enumerations & Algebraic Data Types (ADT)

Enums are **Tagged Unions (Sum Types)** representing values that can take one of several variant schemas:

```sora
enum IPAddress {
    V4(Byte, Byte, Byte, Byte),
    V6(String),
    Loopback,
}
```

### 5.1 Memory Representation
$$\text{sizeof}(\text{Enum}) = \text{sizeof}(\text{Discriminant Tag}) + \max_{v \in \text{Variants}} \text{sizeof}(v) + \text{Padding}$$
- **Discriminant Optimization**: If a variant holds a non-nullable pointer (e.g. `Option[&T]`), the discriminant tag is omitted, mapping `None` to `0x0` (**Null Pointer Niche Optimization**).

---

## 6. Nullable / Optional Types (`T?`)

`T?` is the first-class syntactic sugar for `Option[T]`:
```sora
enum Option[T] {
    Some(T),
    None,
}

name: String? = Some("Sorayunara")
unwrapped = name ?? "Default"
```
- **Safe Traversal (`?.`)**: Short-circuits property access if the subject is `None`.

---

## 7. Result Types & Error Handling (`T!E`)

`T!E` represents recoverable computations yielding either success `T` or failure `E`:
```sora
enum Result[T, E] {
    Ok(T),
    Err(E),
}

fn open_socket(port: Int) -> Socket!NetworkError {
    if port < 1024 && !is_root() {
        return Err(NetworkError.PermissionDenied)
    }
    Ok(Socket.bind(port))
}
```
- **Unwrap & Propagate (`?`)**: Unwraps `Ok(val)` or returns `Err(e)` immediately from the enclosing scope.

---

## 8. Generics & Parametric Polymorphism

Generics provide compile-time parametric abstraction over types and lifetimes:

```sora
struct Container[T] {
    item: T,
}

fn identity[T](value: T) -> T => value
```

### 8.1 Monomorphization
During compilation, each generic instantiation $\text{Container}[\text{Int}]$ and $\text{Container}[\text{String}]$ produces dedicated, fully-specialized native machine code:
- Zero runtime pointer indirection.
- Inlining and register allocation operate with exact physical type layouts.

---

## 9. Trait System & Interface Contracts

A trait defines a contract of associated methods, functions, and associated types:

```sora
trait Hashable {
    fn hash(self: &Self) -> UInt64
}

trait Iterator {
    type Item
    fn next(mut self: &mut Self) -> Self.Item?
}
```

### 9.1 Trait Bounds & Where Clauses
```sora
fn serialize[T: Serializable + Display](value: &T) -> String {
    value.to_json()
}

fn combine[T, U](a: T, b: U) -> String
where
    T: Display + Clone,
    U: Display + Debug,
{
    "${a} & ${b}"
}
```

### 9.2 Associated Types
Associated types model output relationships without introducing excessive generic type parameters:
$$\text{Iterator} \to \text{Item}$$

---

## 10. Type Coercion & Casting

Sorayunara adheres to **strict explicit type safety**:

1. **No Implicit Lossy Conversions**: Implicit conversion from `Int64` to `Int32` or `Float` to `Int` is rejected at compile-time to prevent silent overflow/precision bugs.
2. **Explicit Cast (`as`)**:
   ```sora
   integer: Int64 = 100
   truncated: Int32 = integer as Int32
   byte_val: Byte = integer as Byte
   ```
3. **Deref Coercion**: A mutable borrow `&mut T` safely coerces into an immutable borrow `&T` when passed into read-only parameters.
4. **Subtyping & Type Narrowing (`is`)**:
   ```sora
   if pet is Dog(d) {
       d.bark() // Flow-sensitively narrowed to Dog
   }
   ```

---

## 11. Type Aliases

Type aliases provide zero-cost ergonomic naming without runtime penalty:

```sora
type Port = UInt16
type ResultHandler[T] = fn(Result[T, Error]) -> ()
type Matrix4x4 = [[Float; 4]; 4]
```

---

## 12. Function Types & Closures

First-class functions represent callable references or heap-allocated closures:

```sora
// Pure Function Pointer
callback: fn(Int, Int) -> Int = (a, b) => a * b

// Async Function Type
fetcher: async fn(String) -> [Byte]!HttpError
```

---

## 13. Compile-Time Type Checking Pipeline

The type checker ([`bootstrap/src/semantics.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/semantics.rs)) executes in 4 deterministic stages:

```
                  ┌─────────────────────────────────────┐
                  │ 1. Symbol Table & Scope Resolution  │
                  └──────────────────┬──────────────────┘
                                     │
                                     ▼
                  ┌─────────────────────────────────────┐
                  │ 2. Hindley-Milner Type Inference     │
                  │    & Constraint Generation          │
                  └──────────────────┬──────────────────┘
                                     │
                                     ▼
                  ┌─────────────────────────────────────┐
                  │ 3. Trait Bounds & Unification       │
                  │    (Equality & Subtype Solving)     │
                  └──────────────────┬──────────────────┘
                                     │
                                     ▼
                  ┌─────────────────────────────────────┐
                  │ 4. Flow-Sensitive Borrow Checking   │
                  │    & Monomorphization Specialization │
                  └─────────────────────────────────────┘
```

1. **Scope Resolution**: Enforces lexical declaration rules and visibility.
2. **Inference & Constraint Generation**: Walks the AST, generating type variables $\alpha, \beta, \gamma$.
3. **Unification & Trait Resolution**: Resolves constraints, verifies trait contracts, and detects mismatches (`E0308: Mismatched Types`).
4. **Safety & Monomorphization**: Verifies affine ownership rules and generates concrete instances for LLVM and C codegen.
