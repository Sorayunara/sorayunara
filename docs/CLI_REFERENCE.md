# 💻 Sorayunara Command Line Interface (CLI) Reference
**Binary Aliases**: `sorayunara` · `sora`

---

## 1. Core Command Set

```powershell
# 1. Project Management
sorayunara new <name>            # Create new structured Sorayunara workspace
sorayunara init                  # Initialize current directory as Sorayunara workspace

# 2. Build & Execution
sorayunara run [file.sora]       # Compile and execute immediately
sorayunara build [--release]     # Build production binary / library
sorayunara check                 # Fast type-check and borrow-check without codegen
sorayunara clean                 # Clean build artifacts

# 3. Testing & Benchmarking
sorayunara test [--coverage]     # Execute native test runner with assertions
sorayunara bench                 # Run high-precision benchmark harness

# 4. Package Manager & Registry
sorayunara add <package>         # Add dependency and update lockfile
sorayunara remove <package>      # Remove dependency
sorayunara update                # Update dependencies
sorayunara search <query>        # Search packages on packages.sorayunara.org
sorayunara audit                 # Scan dependencies for known security vulnerabilities
sorayunara publish               # Publish package bundle to official registry

# 5. Developer Tooling & LSP
sorayunara lsp                   # Start Language Server Protocol daemon on stdio
sorayunara fmt [file.sora]       # Format source code
sorayunara lint [file.sora]      # Run static analyzer and linter
sorayunara doctor                # Environment, MSVC, LLVM, and Windows diagnostics
sorayunara system info           # Query OS, CPU, SCM, and target information
```
