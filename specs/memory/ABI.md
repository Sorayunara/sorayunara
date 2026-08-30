# Sorayunara Memory Model & ABI Specification

## 1. Data Representation
- Primitives: 64-bit aligned (`Int`, `Float`).
- Structs: Field alignment max with deterministic padding.
- Enums: 4-byte discriminant + payload union.
