# 📦 Sorayunara Package Manager & Registry Specification
**Official Registry**: `packages.sorayunara.org` · **Manifest**: `sorayunara.toml` · **Lockfile**: `sorayunara.lock`

---

## 1. Manifest Structure (`sorayunara.toml`)
```toml
[package]
name = "hyper-server"
version = "1.0.0"
authors = ["Sorayunara Team <core@sorayunara.org>"]
edition = "2026"
license = "MIT OR Apache-2.0"

[dependencies]
std_http = "2.1.0"
serde_json = "1.0.4"

[target.x86_64-pc-windows-msvc]
features = ["windows-service", "eventlog"]
```

---

## 2. Cryptographic Integrity & Hermetic Builds
All downloaded tarball packages are cryptographically verified via **SHA-256 digests** recorded in `sorayunara.lock`.
Passing `--locked` during CI enforces strict offline deterministic dependency resolution.

---

## 3. GitHub Packages (GHCR / OCI) Registry Integration
Sorayunara packages (`.sora`) can be distributed and installed directly via **GitHub Packages** (`ghcr.io`):

```toml
[dependencies]
sora_math = { version = "0.1.0", registry = "ghcr.io/sorayunara/packages/sora-math" }
```

### Publishing to GitHub Packages
1. Tag your commit or trigger the `.github/workflows/packages.yml` workflow.
2. The workflow compiles, runs tests on `.sora` files, packages tarball archives (`.sorapkg.tar.gz`), generates SHA-256 signatures, and pushes OCI artifacts directly to `ghcr.io/<owner>/packages/<package-name>:<version>`.

