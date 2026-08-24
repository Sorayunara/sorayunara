# Sorayunara Language Support (VS Code Extension)

Rich Language Server Protocol (LSP), Syntax Highlighting, Snippets, Formatter, Debugging, and Package Tooling for the **Sorayunara Programming Language** (`.sora`).

## Features

- **Language Server (LSP)**: Autocomplete for keywords (`fn`, `spawn`, `await`, `actor`, `chan`), types (`Int`, `Float`, `String`, `Bool`, `Task`, `Chan`), traits, attributes, and stdlib modules (`std.http`, `std.json`, `std.net`, etc.).
- **Go to Definition & Implementation**: Jump directly to functions, structs, enums, traits, or methods.
- **Diagnostics & Error Highlighting**: Real-time syntax errors, borrow checker violations, and Hindley-Milner type inference checking.
- **Document Formatting**: Integrated AST formatting via `sorayunara fmt`.
- **Integrated Commands**:
  - `Sorayunara: Run Active File` (`sorayunara run`)
  - `Sorayunara: Build Active File / Project` (`sorayunara build`)
  - `Sorayunara: Run Test Suite` (`sorayunara test`)
  - `Sorayunara: Type Check & Verify` (`sorayunara check`)
  - `Sorayunara: Start Debug Session` (`sorayunara debug`)
  - `Sorayunara: Format Active Document` (`sorayunara fmt`)
  - `Sorayunara: Restart Language Server`
- **Snippets & Syntax Highlighting**: Comprehensive TextMate grammar covering `.sora` files.

## Getting Started

1. Ensure the `sorayunara` binary is built and available in your system `PATH`.
2. Open any `.sora` file in Visual Studio Code.
3. The extension automatically activates LSP and syntax features.
