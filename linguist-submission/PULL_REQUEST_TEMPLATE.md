# Pull Request: Add Sorayunara Programming Language (.sora)

## Language Information
- **Language Name**: Sorayunara
- **Type**: `programming`
- **File Extension**: `.sora`
- **Scope**: `source.sorayunara`
- **Color**: `#6C5CE7`
- **Official Repository**: https://github.com/Sorayunara/sorayunara
- **Documentation**: https://github.com/Sorayunara/sorayunara/tree/main/docs

## Checklist
- [x] Language entry added to `lib/linguist/languages.yml`
- [x] Sample source files added to `samples/Sorayunara/`
- [x] TextMate grammar added to `grammars/`
- [x] Syntax tests passing
- [x] Verified in multiple real-world modules (`compiler/`, `std/`, `runtime/`, `examples/`)

## Sample Code Snippet
```sora
// server.sora
import std.io
import std.net

async fn main() -> Int {
    print("🌌 Sorayunara language runtime active.")
    return 0
}
```
