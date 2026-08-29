# 🚀 Sorayunara Official Release & Governance Process
**CI/CD Pipeline**: Multi-Platform GitHub Actions Matrix (`.github/workflows/release.yml`)

---

## 1. Release Milestones & Quality Gates
Before any official release is tagged, the following gates must be 100% satisfied:
1. **Zero Failing Tests**: `cargo test --all-targets` must pass 100% across Windows, Ubuntu, and macOS runners.
2. **Security & Dependency Audit**: `cargo audit` & `sorayunara audit` report 0 known vulnerabilities.
3. **Multi-Target Cross-Compilation**: `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `wasm32-wasi` build successfully.

---

## 2. Release Tagging Workflow
```powershell
# 1. Update version in Cargo.toml and package.json
# 2. Commit and push to main
# 3. Create cryptographically signed git tag
git tag -a v2.1.0 -m "Release v2.1.0 - Long Term Support Edition"
# 4. Push tag (Triggers automated multi-platform binary compilation and artifact asset upload)
```
