# 🚀 Sorayunara Migration & Upgrade Guide
**Upgrade Tool**: Automated Code Modernizer (`sorayunara fix`)

---

## 1. Upgrading from v0.1.x / v0.2.x to v2.x.x
1. Update `sorayunara.toml` package edition:
   ```toml
   [package]
   edition = "2026"
   ```
2. Run the automated migration fixer:
   ```powershell
   sorayunara fix --all
   ```
3. Verify test suites:
   ```powershell
   sorayunara test
   ```
