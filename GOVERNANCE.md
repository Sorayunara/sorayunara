# 🏛️ Sorayunara Project Governance & Team Structure

This document outlines the governance model, team structure, and decision-making processes for the **Sorayunara Programming Language Project**.

---

## 👥 1. Teams & Responsibilities

The Sorayunara organization is partitioned into specialized working groups (teams) to ensure fast, domain-focused reviews and clear ownership:

### 👑 Core Maintainers (`@Sorayunara/core-maintainers`)
- **Scope**: Global language architecture, security policy, release management, infrastructure, and final arbitration.
- **Responsibilities**:
  - Reviewing cross-cutting RFCs.
  - Publishing official releases (`v*`) and signing binaries.
  - Managing repository settings, secrets, and branch protection rules.

### ⚙️ Compiler Team (`@Sorayunara/compiler-team`)
- **Scope**: Lexer, Pratt Parser, AST, Hindley-Milner Typechecker, Borrowchecker, IR lowering, Optimizer, and Codegen (LLVM, C99, WASM).
- **Responsibilities**:
  - Reviewing changes to `compiler/`, `bootstrap/`, `specs/`.
  - Ensuring zero compiler performance regressions and parser correctness.

### 📦 Standard Library & Runtime Team (`@Sorayunara/stdlib-team`)
- **Scope**: Standard library modules (`std/*`), runtime VM, memory allocators, M:N green-thread scheduler, CSP channels, and actors.
- **Responsibilities**:
  - Reviewing API additions to standard libraries.
  - Ensuring cross-platform portability across Linux, Windows, macOS, Android, iOS, and WebAssembly.

### 🛠️ Tooling & Developer Ecosystem Team (`@Sorayunara/tooling-team`)
- **Scope**: Language Server Protocol (`lsp`), VS Code / Vim / Neovim / Emacs plugins, `sora fmt`, `sora lint`, package manager (`sora pkg`), and Web Playground.
- **Responsibilities**:
  - Maintaining developer ergonomics, formatting standards, and IDE extensions.

### 📖 Documentation & Community Team (`@Sorayunara/docs-team`)
- **Scope**: Language specifications, tutorials, architectural guides, i18n translations, and community management.
- **Responsibilities**:
  - Reviewing and editing `docs/`, `SPECIFICATION.md`, and tutorials.
  - Welcoming new contributors and moderating discussions.

---

## 📜 2. Decision Making & RFC Process

For major features (new syntax, keywords, breaking changes to stdlib or type system):
1. **Submit an RFC**: Create an issue using the `RFC Proposal` template.
2. **Community Discussion**: The community and relevant domain team discuss tradeoffs for at least 7 days.
3. **Consensus & Approval**: The relevant domain team votes (majority consensus required) before implementation begins.
4. **Implementation**: Pull request created targeting the `develop` branch.

---

## 🔒 3. Branch Protection & Merge Policy

- **`main` Branch**: Production releases only. Direct pushes are disallowed. Merges require all CI matrix checks to pass and approval from `@Sorayunara/core-maintainers`.
- **`develop` Branch**: Integration branch for active development. Merges require:
  - All automated CI checks green (`Test on ubuntu-latest`, `Test on windows-latest`, `Test on macos-latest`).
  - At least 1 review approval from the code-owning team according to `.github/CODEOWNERS`.
  - Clean linear Git history (Squash and Merge or Rebase).
