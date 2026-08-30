# Sorayunara (.sora)

**High-Performance, Memory-Safe Systems & AI Programming Language**

---

## 1. Quickstart & Installation

```bash
# Clone the repository
git clone https://github.com/Sorayunara/sorayunara.git
cd sorayunara

# Build the bootstrap toolchain
cargo build --release

# Run a Sorayunara source file
cargo run --release -- run main.sora

# Run the test suite
cargo run --release -- test

# Format code
cargo run --release -- fmt
```

---

## 2. Feature Maturity Status

| Subsystem | Status | Description |
| :--- | :---: | :--- |
| **Lexer & Parser** | 🟢 Stable | Full EBNF grammar, token spans, recovery. |
| **Hindley-Milner Type Inference** | 🟢 Stable | Bidirectional inference, parametric generics, ADTs. |
| **Borrow Checker** | 🟡 Experimental | Flow-sensitive borrow checking, move semantics. |
| **Bytecode VM & JIT** | 🟢 Stable | Stack VM, instruction pipeline, optimizer. |
| **LLVM IR Backend** | 🟡 Experimental | Native SSA emission for x86_64 & ARM64. |
| **C99 & WebAssembly Backends** | 🟡 Experimental | ANSI C transpile & WAT emission. |
| **Concurrency Runtime** | 🟡 Experimental | M:N work-stealing scheduler, CSP channels, Actors. |
| **Standard Library** | 🟡 Experimental | 18 core modules (`std/`). |
| **Package Manager** | 🔵 Preview | `sora.toml`, `sora.lock`, SHA-256 integrity. |
| **Language Server Protocol (LSP)** | 🔵 Preview | Hover, diagnostics, completion. |
| **Debugger (DAP) & Profiler** | 🔵 Preview | Stack frames, memory sampling. |
| **Self-Hosting (Stage 2/3)** | ⚪ Planned | RFC 0001 compilation bootstrap. |

---

## 3. Official Ecosystem Structure

```
Sorayunara/
├── sorayunara                # Core compiler, VM runtime, and toolchain
├── sorayunara-starter-template # Official project starter templates
├── sorayunara-http-service    # Production-ready HTTP backend reference
└── sorayunara-algorithms      # Benchmark & computational algorithms suite
```

---

## 4. Governance & Contributing

- **[Language Specification](docs/language-spec/)**: Formal semantics, ABI, and grammar rules.
- **[RFC Process](rfcs/)**: Open proposals for language evolution.
- **[Contributing Guide](CONTRIBUTING.md)**: Developer onboarding for Rust engine and `.sora` native code.
- **[Governance Model](GOVERNANCE.md)**: Core team and decision-making framework.
- **[Support Channels](SUPPORT.md)**: Issue routing and community help.
