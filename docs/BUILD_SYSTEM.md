# 🏗️ Sorayunara Build System Architecture
**Engine**: Native Direct Linker & LLVM Target Orchestrator

---

## 1. Incremental Build DAG
The build system structures compilation units into a Directed Acyclic Graph (DAG). Caching is performed per AST module unit based on content cryptographic hashing.

---

## 2. Cross-Compilation Profiles
```powershell
# Cross-compile for Linux from Windows
sorayunara build --target x86_64-unknown-linux-gnu

# Cross-compile for macOS ARM64 Apple Silicon
sorayunara build --target aarch64-apple-darwin
```
