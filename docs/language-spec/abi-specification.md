# 🧩 Sorayunara Application Binary Interface (ABI) & Foreign Function Interface (FFI) Specification

**Document Version**: 1.0.0  
**Specification Status**: Normative Architecture Standard  
**File Identifier**: `docs/language-spec/abi-specification.md`

---

## 1. Calling Conventions & Target Architectures

Sorayunara conforms to standard platform calling conventions for zero-cost interoperability with C, C++, and Rust:

| Target Platform | Native Calling Convention | Register Arguments | Stack Alignment |
|---|---|---|---|
| **x86_64 Windows (MSVC)** | Microsoft x64 (`__fastcall`) | `RCX`, `RDX`, `R8`, `R9`, `XMM0-3` | 16-byte aligned |
| **x86_64 Linux/macOS** | System V AMD64 ABI | `RDI`, `RSI`, `RDX`, `RCX`, `R8`, `R9` | 16-byte aligned |
| **AArch64 / ARM64** | ARM64 AAPCS64 | `X0-X7`, `V0-V7` | 16-byte aligned |
| **WebAssembly (WASM)** | W3C WASM Component Model / Wasmtime | Stack-value arguments | 4-byte/8-byte aligned |

---

## 2. Type Representation & Alignment

### 2.1 Primitive Types
- `Int` / `i64`: 64-bit two's complement integer (8 bytes, align 8).
- `Float` / `f64`: 64-bit IEEE 754 double precision (8 bytes, align 8).
- `Bool`: 8-bit boolean byte (`0x00` = false, `0x01` = true; 1 byte, align 1).
- `Char`: 32-bit Unicode Scalar Value UTF-32 (4 bytes, align 4).
- `String`: Fat pointer structure `(ptr: *const u8, len: usize, cap: usize)` (24 bytes on 64-bit).

### 2.2 Struct Layout & Padding Rules
Fields are ordered sequentially in memory with padding inserted to ensure field alignment matches $\max(\text{field alignment})$.

```
Struct {
    a: Bool,   // offset 0 (1 byte)
    // padding 7 bytes
    b: Int,    // offset 8 (8 bytes)
    c: Float   // offset 16 (8 bytes)
} // Total size: 24 bytes, alignment: 8
```

### 2.3 Algebraic Data Types (Enums)
Enums use a discriminant tag (`u32` / 4 bytes) followed by union payloads:
```
Offset 0..3: Discriminant Tag (0 = Variant A, 1 = Variant B, ...)
Offset 4..7: Alignment Padding (if largest payload requires 8-byte alignment)
Offset 8..N: Variant Payload Memory Union
```

---

## 3. Foreign Function Interface (`extern "C"`)

Functions marked `extern "C"` export standard un-mangled symbols and adopt the platform C calling convention:

```sora
extern "C" {
    fn malloc(size: Int) -> Int
    fn free(ptr: Int)
    fn memcpy(dest: Int, src: Int, n: Int) -> Int
}
```

---

## 4. Symbol Naming & Binary Compatibility

- Standard Sorayunara functions: `_SORA_<module>_<function>_<type_hash>`
- FFI `extern "C"` functions: Raw unmodified C identifier name (e.g. `malloc`, `strlen`).
