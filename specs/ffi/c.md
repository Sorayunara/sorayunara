# Sorayunara Foreign Function Interface (FFI) Specification

## 1. C ABI Integration
- `extern "C"` declarations bind to platform C symbols without name mangling.
- Supports pointer arithmetic, raw allocations, and standard POSIX/Win32 APIs.
