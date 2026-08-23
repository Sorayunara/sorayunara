# Aether Language Support (VS Code Extension)

Rich Language Server Protocol (LSP), Syntax Highlighting, Snippets, Formatter, and Debugging support for the **Aether Programming Language**.

## Features

- **Autocomplete & IntelliSense**: Fast contextual completions for keywords (`fn`, `spawn`, `await`, `comptime`), types (`Int`, `String`, `Chan`, `Task`), attributes (`@derive`, `@cfg`), and stdlib symbols.
- **Go to Definition & Implementation**: Jump directly to function, struct, trait, or implementation definitions across workspace files.
- **Find References & Rename**: Workspace-wide symbol discovery and safe refactoring.
- **Diagnostics & Error Highlighting**: Real-time syntax errors, borrow checker violations, and type mismatch alerts.
- **Hover Documentation**: Type signatures, docstrings, and concurrency safety notes on symbol hover.
- **Code Actions & Quick Fixes**: Automated import organization (`source.organizeImports`), borrow mutability fixes, and macro derivation.
- **Document Formatting**: Built-in AST-based code formatting via `aether fmt`.
- **Semantic Highlighting**: Full TextMate grammar tokenization and semantic token modifiers.

## Getting Started

1. Ensure the `aether` binary is installed and present in your system `PATH`.
2. Open any `.ae` or `.aether` file in VS Code.
3. The language server will automatically start using `aether lsp`.
