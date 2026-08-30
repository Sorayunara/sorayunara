# 🏛️ Sorayunara Formal Compiler Pipeline & IR Architecture

**Document Version**: 1.0.0  
**Specification Status**: Normative Architecture Standard  
**File Identifier**: `docs/language-spec/compiler-architecture.md`

---

## 1. End-to-End Multistage Pipeline

```
                      .sora Source Code
                             │
                             ▼
                    ┌─────────────────┐
                    │ 1. Lexer        │  (Deterministic UTF-8 tokenization)
                    └────────┬────────┘
                             │ Token Stream
                             ▼
                    ┌─────────────────┐
                    │ 2. Pratt Parser │  (Precedence climbing & concrete syntax)
                    └────────┬────────┘
                             │ AST (Abstract Syntax Tree)
                             ▼
                    ┌─────────────────┐
                    │ 3. Name Resolver│  (Symbol scopes & lexical visibility)
                    └────────┬────────┘
                             │ Resolved Scopes
                             ▼
                    ┌─────────────────┐
                    │ 4. HIR Lowering │  (High-Level IR: typed tree representation)
                    └────────┬────────┘
                             │ HIR
                             ▼
                    ┌─────────────────┐
                    │ 5. Type Checker │  (Damas-Hindley-Milner bidirectional inference)
                    └────────┬────────┘
                             │ Typed HIR
                             ▼
                    ┌─────────────────┐
                    │ 6. Borrow Check │  (Non-Lexical Lifetimes & Move tracking)
                    └────────┬────────┘
                             │ Validated HIR
                             ▼
                    ┌─────────────────┐
                    │ 7. MIR Lowering │  (Mid-Level IR: SSA BasicBlock CFG)
                    └────────┬────────┘
                             │ MIR
                             ▼
                    ┌─────────────────┐
                    │ 8. Optimization │  (Constant folding, DCE, Inlining)
                    └────────┬────────┘
                             │ Optimized MIR / Bytecode IR
                             ▼
           ┌─────────────────┼─────────────────┐
           ▼                 ▼                 ▼
    ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
    │ LLVM IR     │   │ ANSI C99    │   │ WebAssembly │
    │ (.ll/Obj)   │   │ (.c/Headers)│   │ (.wat/.wasm)│
    └─────────────┘   └─────────────┘   └─────────────┘
```

---

## 2. Intermediate Representations (IR) Specification

### 2.1 High-Level Intermediate Representation (HIR)
- **Module**: [`bootstrap/src/hir.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/hir.rs)
- **Structure**: Tree-based IR preserving high-level expressions, typed variables, loop constructs, and function signatures.
- **Role**: Discards syntax sugar, desugars `T?` to `Option[T]`, and attaches full type schemes.

### 2.2 Mid-Level Intermediate Representation (MIR)
- **Module**: [`bootstrap/src/mir.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/mir.rs)
- **Structure**: Control Flow Graph (CFG) consisting of `BasicBlockData` with explicit SSA `MirStatement` and `Terminator` (`Goto`, `SwitchInt`, `Return`).
- **Role**: Explicit memory operations (`StorageLive`, `StorageDead`, `Move`, `Copy`, `Ref`), fine-grained borrow checking, and cross-block dataflow analysis.

### 2.3 Optimization Passes
- **Module**: [`bootstrap/src/optimizer.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/optimizer.rs)
- **Passes**:
  1. Constant Folding & Propagation: `10 + 20 * 2` $\to$ `50`.
  2. Dead Code Elimination (DCE): Strips unreachable basic blocks.
  3. Link-Time Dead Function Elimination (LTO).

### 2.4 Multi-Target Codegen Emitters
- **LLVM Backend ([`bootstrap/src/llvm_backend.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/llvm_backend.rs))**: Generates `.ll` with `target triple`, metadata, and calling conventions.
- **ANSI C Transpiler ([`bootstrap/src/codegen.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/codegen.rs))**: Emits portable C99 with stdio/stdlib headers and FFI wrappers.
- **WASM Backend ([`bootstrap/src/wasm_backend.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/wasm_backend.rs))**: Emits S-expressions in WebAssembly Text format (`.wat`).
