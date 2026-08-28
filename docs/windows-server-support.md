# 🪟 Sorayunara Windows Server Support Matrix & Production Guide

**Specification Version**: 2.0.0 (LTS Architecture)  
**Target Environment**: Windows Server (On-Premises, Azure, AWS EC2, Hybrid Cloud)  
**Supported Architectures**: `x86_64` (Primary), `aarch64` (ARM64 Windows on Snapdragon/Azure)

---

## 🏛️ 1. Official Support Tiers

| Windows Server Edition | Build / Version | Target Architecture | Installation Options | Support Tier |
|---|---|---|---|---|
| **Windows Server 2025** | 10.0.26100 (LTSC) | `x86_64`, `aarch64` | Server Core, Desktop Experience | **Tier 1 (Production)** |
| **Windows Server 2022** | 10.0.20348 (LTSC) | `x86_64` | Server Core, Desktop Experience | **Tier 1 (Production)** |
| **Windows Server 2019** | 10.0.17763 (LTSC) | `x86_64` | Server Core, Desktop Experience | **Tier 1 (Production)** |
| **Windows Server 2016** | 10.0.14393 (LTSC) | `x86_64` | Server Core, Desktop Experience | **Tier 1 (Production)** |
| **Windows Server 2012 R2** | 6.3.9600 (Legacy) | `x86_64` | Server Core, Desktop Experience | **Tier 2 (Legacy Compat)** |
| **Windows Server 2012** | 6.2.9200 (Legacy) | `x86_64` | Server Core, Desktop Experience | **Tier 2 (Legacy Compat)** |
| **Windows Server 2008 R2** | 6.1.7601 (Legacy) | `x86_64` | Desktop Experience | **Tier 3 (Best Effort)** |
| **Windows Server 2008** | 6.0.6003 (Legacy) | `x86_64` | Desktop Experience | **Tier 3 (Best Effort)** |
| **Windows Server 2003 / 2000 / NT** | < 6.0 | `x86` / `x64` | Any | ❌ **Unsupported** |

---

## 📋 2. Tier Definitions & Guarantees

### 🟢 Tier 1: Official Production (2016 – 2025)
- **100% Automated CI Guarantee**: Tested on Windows Server GitHub Actions runners.
- **Native Windows Service Subsystem**: Full SCM (Service Control Manager) integration, auto-recovery, and delayed-start support.
- **Security & Logging**: Direct Windows Event Log (`std.windows.eventlog`), DPAPI credential encryption, and ACL file permissions.
- **Server Core Certified**: 100% headless operation with zero dependencies on desktop GUI DLLs (`user32`, `gdi32`, `shell32` GUI dialogs).

### 🟡 Tier 2: Legacy Compatibility (2012 / 2012 R2)
- Compiles and runs basic console and HTTP workloads.
- Requires compatible MSVC redistributables / C-runtime.
- Security updates and newer Windows APIs (e.g. enhanced TLS 1.3 / QUIC) fallback to software implementations.

### 🔴 Tier 3: Best-Effort Legacy (2008 / 2008 R2)
- Portable ZIP deployment only.
- No SLA or enterprise production guarantees.

---

## 🎯 3. Compiler Target Triples & Configuration

In `sorayunara.toml` or via CLI:

```toml
[target.windows-server-2025-x64]
triple = "x86_64-pc-windows-msvc"
minimum_os = "10.0.26100"
subsystem = "console"

[target.windows-server-2022-x64]
triple = "x86_64-pc-windows-msvc"
minimum_os = "10.0.20348"
subsystem = "console"

[target.windows-server-2019-x64]
triple = "x86_64-pc-windows-msvc"
minimum_os = "10.0.17763"
subsystem = "console"

[target.windows-server-2016-x64]
triple = "x86_64-pc-windows-msvc"
minimum_os = "10.0.14393"
subsystem = "console"
```

```powershell
# Build for specific target
sora build --target windows-server-2022-x64
```

---

## ⚙️ 4. Server Core & Nano Server Guidelines

| Requirement | Server Core (`Tier 1`) | Nano Server (`Container Target`) | Desktop Experience (`Tier 1`) |
|---|---|---|---|
| **Headless CLI** | ✅ Native | ✅ Native | ✅ Native |
| **Windows Service** | ✅ Native SCM | ❌ Containers / Tasks | ✅ Native SCM |
| **PowerShell & WinRM** | ✅ Full Support | ✅ Core Only | ✅ Full Support |
| **GUI Explorer / Dialogs** | ❌ Blocked | ❌ Blocked | ⚠️ Allowed (Not recommended for servers) |
| **Memory Footprint** | ~512 MB – 1 GB | ~128 MB | ~2 GB+ |
