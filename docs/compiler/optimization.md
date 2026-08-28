# ⚡ Compiler: Multi-Pass Optimizer (`compiler/optimizer/`)

The optimizer performs analyses and transformations on the intermediate representations (MIR / Bytecode IR).

---

## Optimization Passes
1. **Constant Folding**: Evaluates constant expressions at compile time.
2. **Dead Code Elimination (DCE)**: Truncates unreachable bytecode instructions while preserving jump targets.
3. **Function Inlining**: Inlines parameterless small functions to eliminate call-frame overhead.
4. **Peephole Optimization**: Replaces redundant store-load pairs with direct register/stack reuse.
