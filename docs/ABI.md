# 🏛️ Sorayunara Application Binary Interface (ABI) Specification
**Architecture Targets**: `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`

---

## 1. Calling Conventions
- **Internal Functions**: `fastcall` / register-based vector calling convention.
- **Extern Functions (`extern "C"`)**: Standard C calling convention (`cdecl` / MS x64 ABI / System V AMD64 ABI).
  - First 4 arguments on Windows x64 passed in `RCX`, `RDX`, `R8`, `R9`.
  - First 6 arguments on System V x64 passed in `RDI`, `RSI`, `RDX`, `RCX`, `R8`, `R9`.

---

## 2. Struct Memory Layout
- Default struct layout matches C alignment rules (natural alignment with padding).
- Explicit layout control supported via `@repr("C")` or `@repr("packed")`.
