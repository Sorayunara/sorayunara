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
