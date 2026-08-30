# 📦 Sorayunara Formal Language Specification: Modules & Namespaces

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/modules.md`

---

## 1. Module Hierarchy & Organization

Sorayunara organizes code into hierarchical namespaces called **modules**. A compilation unit is a **package** containing one or more modules.

```
project_root/
├── Cargo.toml / sora.toml
├── src/
│   ├── main.sora           (Root entry module)
│   ├── network/
│   │   ├── mod.sora        (Sub-module root)
│   │   ├── http.sora       (network.http module)
│   │   └── websocket.sora  (network.websocket module)
│   └── utils.sora          (utils module)
```

---

## 2. Module Declarations (`mod`)

```sora
// Declares submodule defined in separate file `network.sora` or `network/mod.sora`
mod network

// Inlined nested module
mod math {
    pub const PI = 3.14159265
    pub fn add(a: Int, b: Int) -> Int => a + b
}
```

---

## 3. Visibility & Encapsulation (`pub`)

Items are private to their enclosing module by default:

| Modifier | Visibility Scope |
|---|---|
| *(none)* | Private to the current module |
| `pub` | Public to all modules and external consumers |
| `pub(crate)` | Accessible anywhere within the current package |
| `pub(super)` | Accessible only to the immediate parent module |
| `pub(self)` | Restricted strictly to the declaring module |

```sora
pub struct Config {
    pub host: String,     // Public field
    port: Int,            // Private field
}
```

---

## 4. Importing Items (`import` / `use`)

```sora
// Import specific symbols
import std.fs.{File, Path}
import std.net.http.Client as HttpClient

// Import all public symbols (wildcard)
import std.collections.*

// Re-exporting symbols
pub use internal_engine.Engine as PublicEngine
```
