# Contributing to Sorayunara

Thank you for contributing to Sorayunara! This guide clarifies our multi-layer architecture and contribution workflow.

---

## 1. Architecture Separation: Language vs Bootstrap Compiler

| Component | Source / Target | Implementation | Verification Tool |
| :--- | :--- | :--- | :--- |
| **Sorayunara Language Code** | `.sora` | Native Sorayunara | `sora test`, `sora check`, `sora fmt` |
| **Bootstrap Compiler & Core Toolchain** | Rust (`bootstrap/src/`) | Rust 2021 Edition | `cargo test`, `cargo fmt --check` |
| **Standard Library** | `std/*.sora` | Native Sorayunara | `cargo test --test standard_library_ecosystem_tests` |
| **Language Specification** | `docs/language-spec/`, `specs/` | Markdown & EBNF | `cargo test --test grammar_verification_tests` |

---

## 2. Developer Onboarding & Workflows

### A. Developing the Compiler Engine (Rust Core)
```bash
# Verify formatting
cargo fmt --check

# Run full compiler verification test suite
cargo test

# Run specific subsystem tests
cargo test --test borrow_checker_matrix_tests
cargo test --test language_conformance_matrix_tests
```

### B. Developing in Sorayunara (`.sora`)
```bash
# Run a Sorayunara source file
sora run main.sora

# Run Sorayunara unit tests
sora test

# Format Sorayunara source files
sora fmt

# Type-check and borrow-check without running
sora check
```

---

## 3. Pull Request Guidelines

1. Ensure all Rust tests pass (`cargo test`).
2. Ensure code formatting is clean (`cargo fmt`).
3. If introducing new syntax or semantics, update the relevant specification in `docs/language-spec/` or `specs/`.
4. Open an RFC in `rfcs/` for any breaking language design changes.
