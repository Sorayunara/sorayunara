# 📅 Sorayunara Language Edition & Versioning Scheme
**Standard**: Semantic Versioning (SemVer 2.0.0) & Epoch Editions

---

## 1. Release Version Format: `MAJOR.MINOR.PATCH`
- **MAJOR**: Breaking language semantics or ABI changes (guaranteed across Long-Term Support cycles).
- **MINOR**: Backward-compatible new syntax, compiler optimizations, standard library modules.
- **PATCH**: Bug fixes, security patches, compiler performance improvements.

---

## 2. Epoch Editions (`edition = "2026"`)
Editions allow opt-in syntax evolution without fracturing existing codebases. Projects specify their target edition in `sorayunara.toml`, and the compiler seamlessly compiles mixed-edition dependency graphs in the same binary.
