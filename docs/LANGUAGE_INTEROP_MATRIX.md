# 🌌 Sorayunara Language Interoperability & Compatibility Matrix

**Principle**: Sorayunara (.sora) is the **50% core language** — every program is written primarily in `.sora` syntax. The remaining 50% is achieved through **native interoperability bridges** to 16 major programming languages, so developers can leverage existing ecosystems without rewriting libraries.

---

## 🏛️ Interop Architecture

```
                        Your Code (100%)
                              │
                    ┌─────────┴─────────┐
                    │                   │
              Sorayunara (50%)    Foreign Lang (50%)
              .sora syntax        via FFI / Codegen
                    │                   │
                    └─────────┬─────────┘
                              │
                    ┌─────────┴─────────┐
                    │  Unified Binary   │
                    │  or Multi-Target  │
                    │  Output           │
                    └───────────────────┘
```

---

## 📊 Complete 16-Language Interoperability Matrix

| # | Language | Domain & Strengths | Interop Mechanism from `.sora` | Bridge Direction | Tier |
|---|---|---|---|---|---|
| 1 | **Python** | AI, ML, Data Science, Backend, Automation | `extern "C"` → CPython C-API / `ctypes` shared library (.so/.dll) | `.sora` ↔ Python | **S+** |
| 2 | **JavaScript** | Web, Node.js, Full-stack | `sora build --target wasm` → WASM import in JS / `emit_js()` transpiler | `.sora` → JS | **S+** |
| 3 | **Java** | Enterprise, Backend, Android legacy, Big Data | `extern "C"` → JNI (Java Native Interface) / GraalVM native-image | `.sora` ↔ Java | **S+** |
| 4 | **C++** | Game engines, Browsers, OS, Trading, HPC | `extern "C"` → C-ABI mangled bridge / direct LLVM object linking | `.sora` ↔ C++ | **S+** |
| 5 | **C** | OS kernels, Embedded, Firmware | `extern "C"` → Zero-cost direct C-ABI / `emit_c()` C99 transpiler | `.sora` ↔ C | **S** |
| 6 | **C#** | .NET, Enterprise, Unity Game Engine | `extern "C"` → P/Invoke / .NET NativeAOT interop | `.sora` ↔ C# | **S** |
| 7 | **TypeScript** | Modern Web & Backend | `sora build --target wasm` → WASM + TS type declarations (.d.ts) | `.sora` → TS | **S** |
| 8 | **Rust** | Systems, Security, Infrastructure | Direct LLVM object linking / `extern "C"` shared ABI | `.sora` ↔ Rust | **S** |
| 9 | **Go** | Cloud, DevOps, Networking, Backend | `extern "C"` → cgo bridge / shared library (.so) | `.sora` ↔ Go | **A+** |
| 10 | **PHP** | Web/Backend | `extern "C"` → PHP FFI extension / shared library | `.sora` → PHP | **A+** |
| 11 | **Kotlin** | Android, Backend (JVM/Native) | JNI bridge (Android) / Kotlin/Native C-interop | `.sora` ↔ Kotlin | **A+** |
| 12 | **Swift** | iOS, macOS native apps | `extern "C"` → Swift C-bridging header / XCFramework | `.sora` ↔ Swift | **A** |
| 13 | **Ruby** | Web/Rails | `extern "C"` → Ruby FFI gem / shared library (.so/.dylib) | `.sora` → Ruby | **A** |
| 14 | **Dart** | Flutter mobile & web | `extern "C"` → `dart:ffi` / `sora build --target wasm` for Flutter Web | `.sora` ↔ Dart | **A** |
| 15 | **R** | Statistics, Data Science, Academic | `extern "C"` → `.Call()` interface / shared library | `.sora` → R | **A** |
| 16 | **SQL** | Database, Data Engineering | `std.sql` / `std.postgres` / `std.redis` stdlib query drivers | `.sora` → SQL | **A** |

---

## 🔗 Interop Mechanisms Explained

### 1. `extern "C"` — Zero-Cost C-ABI Bridge (Primary)
```sora
// Sorayunara calling a C library function directly
extern "C" {
    fn sqrt(x: Float) -> Float
    fn printf(fmt: &String, ...) -> Int
}

fn main() -> Int {
    let result = sqrt(144.0)
    print("Square root: ", result)
    return 0
}
```
**Compatible with**: C, C++, Rust, Go (cgo), Python (ctypes), Java (JNI), C# (P/Invoke), Swift, Kotlin/Native, Ruby (FFI), Dart (dart:ffi), R (.Call), PHP (FFI)

### 2. WASM Codegen — Web & Edge Interop
```sora
// Compile to WebAssembly for browser/Node.js/Deno consumption
// sora build main.sora --target wasm

fn fibonacci(n: Int) -> Int {
    if n <= 1 { return n }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```
```javascript
// JavaScript consuming the compiled WASM module
const wasm = await WebAssembly.instantiate(soraModule);
console.log(wasm.exports.fibonacci(10)); // 55
```
**Compatible with**: JavaScript, TypeScript, Dart (Flutter Web), PHP (WASM runtimes)

### 3. C99 Transpiler — Embedded & Legacy Interop
```sora
// sora build sensor.sora --target c99
// Generates portable ANSI C99 code compilable by any C compiler
fn read_sensor(pin: Int) -> Int {
    return pin * 3
}
```
**Compatible with**: Any platform with a C compiler (GCC, Clang, MSVC, TinyCC, AVR-GCC, ARM-GCC)

### 4. LLVM Object Linking — Native Binary Interop
```
sora build lib.sora --emit=obj    →  lib.o
clang++ main.cpp lib.o -o app     →  Single native binary
```
**Compatible with**: C++, Rust, C, Swift, any LLVM-based toolchain

---

## 🧬 Per-Language Integration Examples

### Python ↔ Sorayunara
```sora
// fast_math.sora — compiled to shared library
fn matrix_multiply(a: &[Float], b: &[Float], n: Int) -> Vec<Float> {
    // High-performance computation in .sora
}
```
```python
# Python consuming the .sora shared library
import ctypes
lib = ctypes.CDLL('./libfast_math.so')
result = lib.matrix_multiply(a_ptr, b_ptr, n)
```

### Java ↔ Sorayunara (JNI)
```sora
// crypto.sora — compiled to .so/.dll with JNI convention
extern "C" fn Java_com_app_Crypto_encrypt(data: &[u8]) -> Vec<u8> {
    // Zero-overhead cryptographic operations
}
```
```java
// Java loading the Sorayunara native library
public class Crypto {
    static { System.loadLibrary("crypto"); }
    public native byte[] encrypt(byte[] data);
}
```

### Go ↔ Sorayunara (cgo)
```sora
// parser.sora — compiled to shared C-ABI library
extern "C" fn parse_document(json: &String) -> Int {
    // Fast JSON parsing in .sora
    return 0
}
```
```go
// Go consuming via cgo
// #cgo LDFLAGS: -L. -lparser
// #include "parser.h"
import "C"
func main() {
    C.parse_document(C.CString(`{"key": "value"}`))
}
```

### Swift ↔ Sorayunara (iOS/macOS)
```sora
// ml_engine.sora — compiled to .a static library
extern "C" fn predict(input: &[Float], len: Int) -> Float {
    // ML inference in .sora
    return 0.95
}
```
```swift
// Swift bridging header
@_silgen_name("predict")
func predict(_ input: UnsafePointer<Float>, _ len: Int32) -> Float

let result = predict(inputArray, Int32(inputArray.count))
```

### Kotlin ↔ Sorayunara (Android NDK)
```sora
// image_filter.sora → compiled via Android NDK
extern "C" fn apply_filter(pixels: &[u8], width: Int, height: Int) -> Vec<u8> {
    // Real-time image processing
}
```
```kotlin
// Kotlin loading native library
external fun applyFilter(pixels: ByteArray, width: Int, height: Int): ByteArray
companion object { init { System.loadLibrary("image_filter") } }
```

---

## 📐 Compatibility Ratio Guarantee

```
┌──────────────────────────────────────────────────────────┐
│              Sorayunara Program Composition               │
│                                                          │
│   ████████████████████  Sorayunara (.sora)    50%        │
│   ░░░░░░░░░░░░░░░░░░░░  Foreign Language     50%        │
│                          via FFI/Codegen Bridge          │
│                                                          │
│   Total: 100% functional, single unified binary          │
└──────────────────────────────────────────────────────────┘
```

The 50/50 split means:
- **Application logic, business rules, and orchestration** → Written in `.sora`
- **Ecosystem libraries, platform SDKs, and legacy code** → Called via interop bridges

This ensures Sorayunara files remain the dominant language in the repository (≥50% for GitHub Linguist detection) while providing seamless access to the entire global software ecosystem.

---

## 🔒 Dependency Direction Contract

```
.sora code ──→ extern "C" ──→ Foreign Library
    │                              │
    │         NEVER                │
    │     ◄───────────             │
    │   Foreign code cannot        │
    │   import .sora internals     │
    │   directly (use C-ABI only)  │
    └──────────────────────────────┘
```

| Rule | Description |
|---|---|
| `.sora` calls foreign | ✅ Always allowed via `extern "C"` or codegen targets |
| Foreign calls `.sora` | ✅ Allowed only through exported C-ABI functions |
| Foreign imports `.sora` module | ❌ Not allowed — use the compiled shared library |
| `.sora` depends on foreign type system | ❌ Not allowed — type safety boundary at FFI |
