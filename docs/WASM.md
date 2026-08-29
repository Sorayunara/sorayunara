# 🌐 Sorayunara WebAssembly & WASI Target Architecture
**Target Triples**: `wasm32-unknown-unknown`, `wasm32-wasi`

---

## 1. WebAssembly Compilation Workflow
```powershell
sorayunara build --target wasm32-unknown-unknown --release
# Emits target/wasm/app.wasm with zero external runtime requirements
```

---

## 2. JavaScript / TypeScript Interoperability
```typescript
import { instantiateSorayunara } from "@sorayunara/wasm-bridge";

const module = await instantiateSorayunara("./app.wasm");
const result = module.sorayunara_compute(10, 20);
console.log("Result from Sorayunara WASM:", result);
```
