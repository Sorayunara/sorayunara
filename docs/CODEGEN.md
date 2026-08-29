# 🛠️ Sorayunara Backend Code Generation Subsystems
**Targets**: LLVM Backend, WASM Emitter, ANSI C99 Transpiler, and Native Assembler

---

## 1. LLVM Backend Emitter (`compiler/codegen/llvm/`)
- Generates LLVM 18+ bitcode and text IR (`.ll`).
- Invokes LLVM optimization passes (`-O3`, loop unrolling, vectorization, function inlining).
- Emits native object files (`.obj` on Windows, `.o` on Linux/macOS) with debug info (CodeView / DWARF).

---

## 2. WebAssembly Backend (`compiler/codegen/wasm/`)
- Emits pure WebAssembly Core binaries (`.wasm`) conforming to WASI preview1 / preview2.
- Zero host runtime dependency for browser and cloud edge deployments.

---

## 3. C99 Transpiler Backend (`compiler/codegen/c/`)
- Emits standard ANSI C99 code for maximum portability across embedded microcontrollers and legacy platforms.
