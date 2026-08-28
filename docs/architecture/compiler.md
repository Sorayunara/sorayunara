# ⚙️ Sorayunara Compiler Architecture

The Sorayunara compiler pipeline is a multi-stage, ahead-of-time (AOT) and bytecode compiling engine designed for speed, safety, and zero-boilerplate ergonomics.

---

## 🏛️ Pipeline Stages

1. **Lexical Analysis (`lexer/`)**:
   - Transforms UTF-8 source text into a linear token stream.
   - Enforces the strict keyword budget (<100 keywords) and operator tokenization (`:=`, `=>`, `|>`, `?`).

2. **Parsing & AST Construction (`parser/`)**:
   - Recursive descent Pratt parser with operator precedence.
   - Generates an Abstract Syntax Tree (AST) preserving source code spans for diagnostics.

3. **Macro Expansion & Compile-Time Evaluation (`comptime`)**:
   - Expands procedural macros and evaluates `comptime` blocks before static analysis.

4. **Hindley-Milner Type Inference (`typeck/`)**:
   - Unifies types bidirectionally across variables, function calls, and expressions.
   - Enforces generic monomorphization and trait boundary validation.

5. **Borrow Checking & Ownership (`borrowck/`)**:
   - Affine type system ensuring memory safety without a garbage collector.
   - Non-lexical lifetimes (NLL) preventing use-after-move and data races.

6. **Intermediate Representations (HIR & MIR)**:
   - Typed High-Level IR lowered into SSA-based Mid-Level IR with BasicBlocks.

7. **Multi-Pass Optimizer (`optimizer/`)**:
   - Dead Code Elimination (DCE), jump threading, constant folding, and function inlining.

8. **Multi-Target Backend Codegen (`codegen/`)**:
   - **LLVM**: Generates `.ll` and native object files.
   - **WebAssembly**: Generates `.wat` / `.wasm`.
   - **ANSI C**: Emits portable C99 / C11 code.
