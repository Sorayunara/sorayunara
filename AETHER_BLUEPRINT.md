# 🌌 AETHER: Master Blueprint & Specification
> **Aether — Fast, Safe and Simple Language for Modern Backend & Systems.**

---

## 🏛️ 1. Identitas & Pilar Desain

```
                      AETHER
                         │
         ┌───────────────┼───────────────┐
         │               │               │
       FAST            SAFE            SIMPLE
         │               │               │
   • Native Binaries   • Memory Safety • Clean Syntax
   • LLVM Backend      • Concurrency   • Modern Tooling
   • Zero-copy I/O     • Type Safety   • Helpful Errors
   • Async/Coroutines  • Capability FFI• Package Manager
         │               │               │
         └───────────────┬───────────────┘
                         │
                 BACKEND & SYSTEMS
```

---

## 📐 2. Arsitektur Kompiler (Multi-Stage Pipeline)

```
[ Source (.ae) ]
       │
       ▼ (Lexer - SIMD-friendly tokenization)
[ Token Stream ]
       │
       ▼ (Recursive Descent + Pratt Parser)
[ Abstract Syntax Tree (AST) ]
       │
       ▼ (Macro Expander & Comptime Evaluator)
[ Expanded AST ]
       │
       ▼ (Type Checker & Flow-Sensitive Borrow Checker)
[ Typed AST (HIR) ]
       │
       ▼ (Generic Monomorphization & Optimization Pass)
[ Mid-Level IR (MIR) ]
       │
       ├─────────────────────────┬─────────────────────────┐
       ▼                         ▼                         ▼
 [ VM Bytecode ]         [ LLVM IR (Native) ]      [ WebAssembly (WAT) ]
 (Debug & REPL)           (x86_64, ARM64)          (Wasm32 Browser/WASI)
```

---

## 🔒 3. Memory & Safety Model

Aether memiliki **tiga mode memori eksplisit** — tidak berhenti di garbage collector:

| Mode | Sintaks | Perilaku | Target |
|:---|:---|:---|:---|
| **Managed** | `let user = User::new()` | Dikelola otomatis (GC/reference-counted), aman, fokus developer | Backend, application, async services |
| **Owned** | `let owner = move user` | Single-owner, move semantics, zero GC pause, `is_moved` di-track compile-time | CLI, systems, HPC, networking |
| **Unsafe** | `unsafe { ptr.write(v) }` | Raw pointer (`*const T` / `*mut T`), operasi tanpa cek | Embedded, FFI, kernel-adjacent |

1. **Ownership & Borrowing (Zero-Cost)**:
   - Nilai memiliki tepat 1 owner.
   - Peminjaman borrowed (`&T`) atau mutable eksklusif (`&mut T`).
   - Tidak memerlukan Garbage Collector (GC pause = 0ms).
2. **Capability-Based Security Sandbox**:
   - Akses file system, network socket, dan environment variables dikontrol via explicit permission model.
3. **Checked Arithmetic & Bounds Safety**:
   - Proteksi runtime & compile-time terhadap integer overflow dan array out-of-bounds.
4. **Raw Pointer Hanya di `unsafe`**:
   - Dereference pointer / operasi unchecked hanya legal di dalam `unsafe { ... }`.
   - Aether tetap melakukan typecheck terhadap statement di dalam blok `unsafe`.

---

## 🚀 4. Roadmap Pengembangan Resmi (v0.1 → v1.0)

| Versi | Milestone & Fokus Utama | Status |
|:---|:---|:---:|
| **v0.1** | **Core Foundation**: Lexer, Parser, AST, Basic Interpreter, Variables, Functions, Primitives | ✅ Selesai |
| **v0.2** | **Type System**: Struct, Enum, Module, Flow-based Borrow Checker, Diagnostic Engine | ✅ Selesai |
| **v0.3** | **Advanced Language**: Generics, Pattern Matching, Result/Option, Standard Library (`std/`) | ✅ Selesai |
| **v0.4** | **Runtime Engine**: Stack Bytecode VM, Async Coroutines, Capability Sandbox | ✅ Selesai |
| **v0.5** | **Native Codegen**: HIR/MIR Optimizer, LLVM Multi-Target (Linux, Windows, macOS, ARM64), Wasm | ✅ Selesai |
| **v1.0** | **Production Ecosystem**: Package Registry, Reproducible Build (`aether.lock`), LSP, Debugger, Profiler, Benchmarks | ✅ Selesai |

---

## 🛠️ 5. Unified CLI Toolchain (`aether`)

Satu executable tunggal untuk seluruh alur kerja developer:

```bash
aether new <app>       # Scaffold direktori dan proyek baru
aether init [app]      # Inisialisasi proyek pada direktori aktif
aether build [--locked]# Kompilasi native binary dengan reproducible lockfile
aether run [file.ae]   # Eksekusi instan program
aether test            # Jalankan test suite, assertion, dan benchmark
aether fmt [file.ae]   # Auto-formatting kode sumber
aether lint [file.ae]  # Linter dan analisis kualitas kode
aether check [file.ae] # Static typecheck & borrow check cepat
aether add <pkg>       # Install package dari Aether Registry (http, postgres, json)
aether remove <pkg>    # Hapus dependency
aether update          # Sinkronisasi dependency graph
aether publish         # Publikasikan package ke registry
aether doc [file.ae]   # Generate dokumentasi API
aether clean           # Bersihkan target dan cache build
aether doctor          # Diagnostik kesehatan toolchain & environment
aether bench           # Jalankan benchmark resmi vs Rust, Go, C++, Zig
aether lsp             # Language Server Protocol untuk VS Code & IDE
```
