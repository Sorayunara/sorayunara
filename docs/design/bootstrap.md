# 🚀 Sorayunara Self-Hosting & Bootstrap Strategy

This document outlines the evolutionary bootstrap roadmap for the Sorayunara programming language compiler.

---

## 🏛️ 4-Stage Bootstrap Pipeline

### Stage 0: Rust-Powered Reference Toolchain (`bootstrap/`)
- Reference compiler, Pratt parser, Hindley-Milner type checker, and bytecode VM written in Rust (`bootstrap/src/`).
- Serves as the golden baseline oracle for syntax validation, language semantics, and bytecode verification.

### Stage 1: Initial Pure Sorayunara Compiler (`compiler/*.sora`)
- High-level compiler written purely in `.sora`.
- Compiled and verified using the Stage 0 `sora` binary.
- Produces `sorayunara_stage1` executable.

### Stage 2: Self-Hosting Compiler
- `sorayunara_stage1` compiles `compiler/*.sora` source files to generate `sorayunara_stage2`.
- Validates that the language is completely self-hosting.

### Stage 3: Reproducible Build Verification
- `sorayunara_stage2` compiles `compiler/*.sora` to produce `sorayunara_stage3`.
- Validates bit-for-bit checksum reproducibility (`checksum(stage2) == checksum(stage3)`).
