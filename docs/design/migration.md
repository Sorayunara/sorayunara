# 🌌 PURE SORAYUNARA MIGRATION PLAN
> **Architectural Blueprint & Roadmap: Transition from Rust-Bootstrapped Prototype to Genuinely Self-Hosted Sorayunara (`.sora`) and GitHub Linguist Ecosystem Recognition**

---

## 1. Current Rust Dependency Map

The current bootstrap compiler codebase lives in `src/` (Rust 2021 edition) and exhibits zero third-party crate dependencies beyond standard library facilities for file I/O, concurrency, and process spawning.

```
src/main.rs (CLI Driver & Pipeline Orchestrator)
 ├── lexer.rs (Tokenization engine & span tracker)
 ├── parser.rs (Recursive descent AST parser with generics & traits)
 │    └── ast.rs (AST node definitions)
 ├── semantics.rs (Hindley-Milner type inference, borrow checker, ADT exhaustiveness)
 │    ├── symbol_table.rs (Lexical scoping & symbol resolver)
 │    └── monomorphizer.rs (Generic specialization pass)
 ├── hir.rs (High-level typed intermediate representation)
 ├── mir.rs (Control-flow graph with basic blocks)
 ├── ir.rs (Bytecode instruction set & emitter)
 ├── optimizer.rs (Constant folding, dead-code elimination, peephole optimizations)
 ├── vm.rs (Stack-based virtual machine execution sandbox)
 ├── llvm_backend.rs (LLVM 18 IR emitter for multi-target architectures)
 ├── wasm_backend.rs (WebAssembly WAT/WASM binary emitter)
 ├── codegen.rs (C99 & ES2022 transpiler emitters)
 ├── native_builder.rs (Clang/MSVC/GCC toolchain invocation driver)
 ├── lockfile.rs (Deterministic reproducible build lockfile engine)
 ├── registry.rs (Package manager, integrity audit & SHA-256 validator)
 ├── debugger.rs (DAP debugging engine)
 ├── profiler.rs (CPU profiler with call-tree tracking)
 ├── lsp.rs (Language Server Protocol daemon)
 └── test_runner.rs (Unit, property fuzzing & benchmark harness)
```

---

## 2. Current Sorayunara Implementation

- **Standard Library (`std/*.sora`)**: 37 comprehensive modules implemented in Sorayunara syntax (`std/alloc.sora`, `std/http.sora`, `std/ml.sora`, `std/sync.sora`, `std/embedded.sora`, `std/crypto.sora`, `std/cuda.sora`, `std/os.sora`, etc.).
- **Self-Hosting Bootstrap Prototype (`compiler/main.sora`)**: Initial Stage 1 self-hosting pipeline capable of scanning tokens, building AST node counts, and generating multi-architecture native IR (`x86_64`, `ARM64`, `RISC-V`).
- **Examples (`examples/main.sora`)**: Core language showcase exercising functions, type annotations, and VM execution.

---

## 3. Migration Plan: Rust → Pure Sora

Migration follows a strict 4-phase non-destructive staging model:

```
Phase 0 (Current): Rust Bootstrap Compiler (src/*.rs) + Initial .sora stdlib + compiler/main.sora prototype.
Phase 1 (Complete Sora Frontend): Implement lexer.sora, parser.sora, ast.sora, types.sora, and checker.sora in compiler/*.sora.
Phase 2 (Complete Sora Backend): Implement ir.sora, optimizer.sora, and codegen.sora in compiler/*.sora emitting direct Native Machine Code.
Phase 3 (Stage 2 Self-Hosting): Rust bootstrap compiles compiler/*.sora into sorayunara-stage1.exe; sorayunara-stage1 compiles compiler/*.sora to produce sorayunara-stage2.exe.
Phase 4 (Legacy Isolation & Deprecation): Move src/*.rs into bootstrap/legacy/ and designate as historical bootstrap artifacts. Primary codebase becomes 100% .sora.
```

---

## 4. Bootstrap Architecture

To ensure zero circular dependency failure during installation on clean machines:
1. `bootstrap/legacy/` contains the minimal C/Rust bootstrap loader that builds `stage0`.
2. `stage0` compiles `compiler/main.sora` using standard runtime libraries.
3. The resulting `stage1` executable is fully independent of Cargo and Rust toolchains.

---

## 5. Self-Hosting Architecture

The self-hosting pipeline operates across 3 discrete stages:

```
Stage 0 (Bootstrap): 
  [Rust/C Bootstrap Driver] ──compiles──▶ compiler/*.sora ──▶ sorayunara_stage1

Stage 1 (Intermediate Self-Hosted):
  [sorayunara_stage1] ──compiles──▶ compiler/*.sora ──▶ sorayunara_stage2

Stage 2 (Full Golden Self-Hosted Binary):
  [sorayunara_stage2] ──compiles──▶ compiler/*.sora ──▶ sorayunara_stage3

Validation:
  SHA-256(sorayunara_stage2) == SHA-256(sorayunara_stage3)  (Bit-for-bit Reproducible)
```

---

## 6. Native Compiler Architecture

The self-hosted compiler architecture in pure `.sora`:

```
program.sora
     │
     ▼
[compiler/lexer.sora] ──▶ Tokens
     │
     ▼
[compiler/parser.sora] ──▶ AST ([compiler/ast.sora])
     │
     ▼
[compiler/checker.sora] ──▶ Typed AST ([compiler/types.sora])
     │
     ▼
[compiler/ir.sora] ──▶ Sora Native IR
     │
     ▼
[compiler/optimizer.sora] ──▶ Optimized IR (Constant folding, DCE, Peephole)
     │
     ▼
[compiler/codegen.sora] ──▶ Machine Code Emitter
     │
     ├──▶ x86-64 (Linux ELF, Windows PE-COFF, macOS Mach-O)
     ├──▶ ARM64  (Linux ELF, Windows PE-COFF, macOS Mach-O)
     └──▶ RISC-V (Linux ELF, Baremetal Embedded)
```

---

## 7. Standard Library Architecture

All standard library modules reside in `std/` with `.sora` extensions:
- **Core**: `alloc.sora`, `collections.sora`, `string.sora`, `math.sora`, `unicode.sora`
- **I/O & OS**: `fs.sora`, `io.sora`, `os.sora`, `env.sora`, `process.sora`, `time.sora`
- **Concurrency**: `thread.sora`, `sync.sora`, `channel.sora`, `task.sora`, `actor.sora`
- **Networking**: `net.sora`, `http.sora`, `tls.sora`, `dns.sora`, `websocket.sora`, `quic.sora`, `grpc.sora`
- **Systems & Hardware**: `ffi.sora`, `embedded.sora`, `cuda.sora`, `reflection.sora`, `serialization.sora`, `compression.sora`
- **Database & AI**: `sql.sora`, `postgres.sora`, `redis.sora`, `json.sora`, `jwt.sora`, `crypto.sora`, `ml.sora`, `tensor.sora`

---

## 8. Runtime Architecture

`runtime/*.sora` encapsulates low-level memory allocation, green-thread M:N task scheduler, event loops, and exception-free result dispatching:
- `runtime/allocator.sora`: Arena & slab allocation mechanisms.
- `runtime/scheduler.sora`: Work-stealing coroutine fiber scheduler.
- `runtime/io_loop.sora`: epoll/IOCP/kqueue multiplexer wrapper.
- `runtime/panic.sora`: Clean stack unwinder and diagnostic reporter.

---

## 9. GitHub Linguist Architecture

GitHub Linguist determines language statistics using file extensions, signatures, and grammar definitions. To properly establish Sorayunara in the GitHub ecosystem without false overrides:
1. `.sora` is established as the sole primary source extension for the Sorayunara language.
2. The codebase must contain genuine `.sora` source files across `compiler/`, `std/`, `runtime/`, `examples/`, `tests/`, and `tools/`.
3. `.gitattributes` marks `.sora` files as detectable programming language files:
   ```gitattributes
   *.sora linguist-detectable=true
   ```
4. Prepare formal Linguist submission artifacts under `docs/github-linguist.md`.

---

## 10. TextMate Grammar Plan

`editors/vscode/syntaxes/sorayunara.tmLanguage.json` defines complete lexical scopes:
- `source.sora` root scope
- Keywords: `fn`, `let`, `mut`, `const`, `struct`, `enum`, `type`, `trait`, `impl`, `if`, `else`, `while`, `for`, `in`, `loop`, `break`, `continue`, `return`, `match`, `move`, `unsafe`, `async`, `await`, `task`, `spawn`, `chan`, `import`, `mod`, `pub`, `test`, `comptime`
- Primitives: `Int`, `Float`, `Bool`, `String`, `Char`, `Option`, `Result`, `Task`, `Chan`, `RawPointer`, `Void`
- Literals: Strings with escape sequences, integer/float constants, booleans `true`/`false`/`null`
- Comments: Single-line `//` and block `/* ... */`
- Operators: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`, `&`, `&mut`, `->`, `=>`

---

## 11. GitHub Language Submission Plan

Following the official [github-linguist/linguist CONTRIBUTING.md](https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md) guidelines:
1. **Repository Maturity**: Maintain active public repository `https://github.com/Sorayunara/sorayunara` with substantial `.sora` code volume (>10,000 LOC across compiler, stdlib, runtime, tests, examples).
2. **Language Definition**:
   ```yaml
   Sorayunara:
     type: programming
     color: "#5B7CEB"
     aliases: [sora, sorayunara-lang]
     extensions:
       - ".sora"
     tm_scope: source.sora
     ace_mode: text
     language_id: 81928001
   ```
3. **Grammar Repository**: Maintain canonical grammar in `editors/vscode/syntaxes/sorayunara.tmLanguage.json`.
4. **Sample Programs**: Provide rich samples in `examples/` covering syntax diversity.
5. **Upstream PR**: Submit PR to `github-linguist/linguist` with language entry and grammar link once volume threshold is reached.

---

## 12. Exact Files to be Created

| Target Path | Category | Purpose |
|:---|:---|:---|
| `compiler/ast.sora` | Compiler | AST data structures and node types |
| `compiler/lexer.sora` | Compiler | Lexical analyzer and token stream generator |
| `compiler/parser.sora` | Compiler | Recursive descent parser producing AST |
| `compiler/types.sora` | Compiler | Type definitions and type system models |
| `compiler/checker.sora` | Compiler | Hindley-Milner type checker and borrow validator |
| `compiler/ir.sora` | Compiler | Sora Native Intermediate Representation (MIR/HIR) |
| `compiler/optimizer.sora` | Compiler | Constant folding, DCE, and optimization passes |
| `compiler/codegen.sora` | Compiler | Native machine code emitter (x86_64, ARM64, RISC-V) |
| `compiler/main.sora` | Compiler | Self-hosted CLI entrypoint and pipeline coordinator |
| `runtime/allocator.sora` | Runtime | Pure Sora memory allocator |
| `runtime/scheduler.sora` | Runtime | Coroutine task scheduler |
| `runtime/io_loop.sora` | Runtime | Event demultiplexer |
| `runtime/panic.sora` | Runtime | Error unwinder |
| `tools/fmt.sora` | Tooling | Code formatter in pure Sora |
| `tools/lint.sora` | Tooling | Static linter in pure Sora |
| `tools/doc.sora` | Tooling | Documentation generator in pure Sora |
| `docs/github-linguist.md` | Ecosystem | GitHub Linguist registration documentation |
| `.gitattributes` | Git Config | Linguist detectable settings |
| `examples/hello.sora` | Samples | Hello World sample |
| `examples/fibonacci.sora` | Samples | Fibonacci recursion & iteration |
| `examples/calculator.sora` | Samples | Expression evaluator |
| `examples/structs.sora` | Samples | Struct declarations and method dispatch |
| `examples/generics.sora` | Samples | Generic data structures and trait bounds |
| `examples/concurrency.sora` | Samples | Async/await, spawn & channels |
| `examples/io.sora` | Samples | File I/O and stream handling |
| `examples/compiler_example.sora` | Samples | Mini-compiler demonstration |

---

## 13. Exact Files to be Deprecated / Isolated

| File Path | Current Status | Future Target |
|:---|:---|:---|
| `std/*.ae`, `std/*.ao`, `std/*.nm` | Legacy non-.sora std files | Remove (replaced completely by `std/*.sora`) |
| `examples/*.ae`, `examples/*.ao`, `examples/*.nm` | Legacy samples | Remove (replaced by `examples/*.sora`) |
| `main.ae` | Legacy entrypoint | Remove (replaced by root `main.sora`) |
| `src/*.rs` | Active Rust bootstrap | Moved to `bootstrap/legacy/*.rs` upon Stage 2 completion |
| `Cargo.toml`, `Cargo.lock` | Active build manifest | Moved to `bootstrap/` upon full self-hosting independence |

---

## 14. Verification & Testing Required for Each Phase

1. **Phase 1 Verification**:
   - `test_lexer_tokenization`: Validates that `compiler/lexer.sora` tokenizes every keyword, operator, literal, and comment correctly.
   - `test_parser_syntax_tree`: Validates `compiler/parser.sora` builds full AST for functions, structs, enums, match expressions, and generics.
   - `test_typechecker_semantics`: Validates `compiler/checker.sora` catches type mismatches, uninitialized variables, and borrow violations.
2. **Phase 2 Verification**:
   - `test_ir_emission`: Validates `compiler/ir.sora` lowers AST to valid linear control-flow basic blocks.
   - `test_optimizer_passes`: Validates `compiler/optimizer.sora` eliminates dead code and folds constant expressions.
   - `test_native_machine_codegen`: Validates `compiler/codegen.sora` emits valid machine code / LLVM IR for Linux x86_64, Windows ARM64, and Linux RISC-V.
3. **Phase 3 Verification**:
   - `test_stage1_bootstrap_execution`: Validates `sorayunara_stage1` binary executes and compiles sample `.sora` programs.
   - `test_stage2_self_hosting_reproducibility`: Validates `sorayunara_stage2` compiles `compiler/*.sora` and produces an identical `sorayunara_stage3` binary.
4. **Phase 4 Verification**:
   - `test_full_suite_without_cargo`: Validates all 37 standard library modules, runtime, tools, and test suites run entirely through `sorayunara` standalone binary without Rust or Cargo dependencies.
