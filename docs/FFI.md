# 🌉 Sorayunara Foreign Function Interface (FFI) Guide
**Interop Standard**: Zero-Overhead C Foreign Interface & Dynamic Symbol Linking

---

## 1. Importing External C Functions
```sora
extern "C" {
    fn puts(s: *const U8) -> I32;
    fn malloc(size: Usize) -> *mut Unit;
    fn free(ptr: *mut Unit);
}
```

---

## 2. Exporting Sorayunara Functions to C / Rust / Python
```sora
@export("c")
pub fn sorayunara_compute(a: Int, b: Int) -> Int {
    return a * b + 42;
}
```
Compiles directly into static/shared libraries (`.dll`, `.so`, `.dylib`) callable by Python `ctypes`, C/C++, Rust, and C#.
