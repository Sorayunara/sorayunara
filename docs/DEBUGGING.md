# 🐞 Sorayunara Debugging & Diagnostics Guide
**Debug Formats**: CodeView (`.pdb` for MSVC/WinDbg/Visual Studio) · DWARF (`.dSYM` / ELF for GDB/LLDB)

---

## 1. Symbol Emission & Source Mapping
Passing `--debug` or compiling without `--release` emits complete source line mapping tables, variable scopes, and unwind callframes.

---

## 2. Interactive Breakpoints
```sora
import Std.Debug;

fn compute(x: Int) -> Int {
    Debug::breakpoint(); // Triggers hardware breakpoint trap in attached debugger
    return x * 2;
}
```
