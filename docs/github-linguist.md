# 🌌 Sorayunara — GitHub Linguist Language Specification & Submission

This document outlines the official language metadata and submission parameters for registering **Sorayunara** in [github-linguist/linguist](https://github.com/github-linguist/linguist).

---

## 1. Language Identity Metadata

```yaml
Sorayunara:
  type: programming
  color: "#5B7CEB"
  aliases:
    - sora
    - sorayunara-lang
  extensions:
    - ".sora"
  tm_scope: source.sora
  ace_mode: text
  codemirror_mode: clike
  codemirror_mime_type: text/x-csrc
  language_id: 81928001
```

---

## 2. Syntax Characteristics

Sorayunara is a modern, statically typed systems programming language with:
- **Keywords**: `fn`, `let`, `mut`, `const`, `struct`, `enum`, `type`, `trait`, `impl`, `if`, `else`, `while`, `for`, `in`, `loop`, `break`, `continue`, `return`, `match`, `move`, `unsafe`, `async`, `await`, `task`, `spawn`, `chan`, `import`, `mod`, `pub`, `test`, `comptime`.
- **Types**: `Int`, `Float`, `Bool`, `String`, `Char`, `Option`, `Result`, `Task`, `Chan`, `RawPointer`, `Void`.
- **Comments**: Single-line `//` and multi-line `/* ... */`.
- **File Extension**: `.sora`.

---

## 3. Representative Sample Code

### Hello World (`examples/hello.sora`)
```sorayunara
fn main() {
    print("Hello, Sorayunara World!")
}
```

### Fibonacci (`examples/fibonacci.sora`)
```sorayunara
fn fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() -> Int {
    let result: Int = fibonacci(10)
    print("Fibonacci(10) = ", result)
    return result
}
```

### Concurrency & M:N Async (`examples/concurrency.sora`)
```sorayunara
async fn worker(id: Int) -> String {
    print("Worker task starting: ", id)
    return "Task completed"
}

fn main() {
    let t: Task<String> = spawn worker(1)
    let res: String = await t
    print("Received: ", res)
}
```

---

## 4. TextMate Grammar Reference

The official TextMate grammar is hosted and maintained at:
- `editors/vscode/syntaxes/sorayunara.tmLanguage.json`
- Scope Name: `source.sora`

---

## 5. Linguist Submission Roadmap

1. **Repository Code Volume**: Ensure the repository maintains substantial genuine `.sora` code (>10,000 LOC across `compiler/`, `std/`, `runtime/`, `examples/`, `tests/`, `tools/`).
2. **Grammar Release**: Tag and release the VS Code extension bundle with `source.sora` grammar.
3. **Upstream Pull Request**:
   - Fork `github-linguist/linguist`.
   - Add the `Sorayunara` entry to `lib/linguist/languages.yml`.
   - Add sample code under `samples/Sorayunara/`.
   - Run Linguist test suite (`bundle exec rake test`).
   - Submit PR to GitHub Linguist.
