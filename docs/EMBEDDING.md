# 🔌 Embedding Sorayunara Engine in Host Applications (C, C++, Rust, Go)
**C-API Header**: `sorayunara_embed.h` · **Shared Lib**: `sorayunara_runtime.dll` / `libsorayunara.so`

---

## 1. Embedding in C / C++
```c
#include <stdio.h>
#include "sorayunara.h"

int main() {
    SoraVM* vm = sora_vm_create();
    sora_vm_load_file(vm, "app.sora");
    SoraValue result = sora_vm_call(vm, "main", 0, NULL);
    printf("Execution returned: %lld\n", sora_value_as_int(result));
    sora_vm_destroy(vm);
    return 0;
}
```

---

## 2. Zero-Overhead Rust Embedding
Add `sorayunara = "0.2.2"` to your `Cargo.toml` and invoke the interpreter or JIT directly in-process.
