# Sorayunara Governance Model

This document outlines the governance structure, decision-making process, and RFC lifecycle for the Sorayunara project.

---

## 1. Governance Roles

- **Core Team**: Guides overall language philosophy, architectural consistency, and final approval on RFCs.
- **Compiler Maintainers**: Maintain compiler pipeline (`lexer`, `parser`, `semantics`, `hir`, `mir`, `llvm`, `wasm`, `codegen`).
- **Runtime & Stdlib Maintainers**: Maintain the concurrency runtime, scheduler, actors, and standard library modules.
- **Community Contributors**: Authors of RFCs, bug reports, documentation, and external ecosystem tooling.

---

## 2. Decision Making & RFC Lifecycle

All major language changes, syntax additions, and breaking modifications must proceed through the RFC process:

```
[ Draft ] ──▶ [ Discussion ] ──▶ [ Accepted ] ──▶ [ Implemented ] ──▶ [ Stable ]
```

1. **Draft**: Create a markdown proposal in `rfcs/` following the standard template.
2. **Discussion**: Community review on GitHub Discussions & Pull Requests.
3. **Accepted**: Approved by Core Team.
4. **Implemented**: Landed in compiler and verified with conformance tests.
5. **Stable**: Finalized into formal specification in `docs/language-spec/`.
