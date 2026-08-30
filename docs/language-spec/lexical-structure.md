# 🔤 Sorayunara Formal Language Specification: Lexical Structure

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/lexical-structure.md`

---

## 1. Encoding & Source Representation

1. **Character Encoding**: Sorayunara source files (`.sora`) MUST be encoded in valid **UTF-8**. Byte order marks (BOM) are disallowed or silently discarded by the compiler frontend.
2. **Source Units**: Lexical analysis operates on Unicode scalar values (`U+0000` through `U+10FFFF`, excluding surrogate code points `U+D800` through `U+DFFF`).
3. **Case Sensitivity**: The language is strictly case-sensitive. `Identifier`, `identifier`, and `IDENTIFIER` represent distinct lexical symbols.

---

## 2. White Space & Line Terminators

### 2.1 Whitespace Characters
Whitespace characters separate tokens and are otherwise ignored except inside string and character literals:
- Space (`U+0020`)
- Horizontal Tab (`U+0009`, `\t`)
- Carriage Return (`U+000D`, `\r`)
- Line Feed (`U+000A`, `\n`)

### 2.2 Semicolon Insertion Rules (ASI)
Sorayunara does NOT require explicit semicolons (`;`). 
- Newlines serve as statement separators when the preceding token can legitimately end a statement and the succeeding token can legitimately begin one.
- Semicolons (`;`) are permitted as explicit statement separators on single lines.

---

## 3. Comments

Sorayunara defines three distinct comment syntactic structures:

```sora
// 1. Line comment: runs to the end of the line

/* 2. Block comment: can span multiple lines 
   /* Block comments can be arbitrarily nested */
*/

/// 3. Documentation comment (Outer Doc): attached to the subsequent declaration
//! 4. Module Documentation comment (Inner Doc): attached to the enclosing module/scope
```

---

## 4. Identifiers

### 4.1 Regular Identifiers
A standard identifier starts with an ASCII alphabetic character (`a-z`, `A-Z`) or underscore (`_`), followed by any sequence of ASCII alphanumerics or underscores:
$$\text{IdentStart} \Coloneqq [\text{a-zA-Z\_}]$$
$$\text{IdentContinue} \Coloneqq [\text{a-zA-Z0-9\_}]$$
$$\text{Identifier} \Coloneqq \text{IdentStart} \times \text{IdentContinue}^*$$

- **Naming Conventions (Normative Style)**:
  - Types, Traits, Enums: `UpperCamelCase` (e.g., `HttpRequest`, `Option`)
  - Functions, Methods, Variables, Modules: `snake_case` (e.g., `calculate_hash`, `user_id`)
  - Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_BUFFER_SIZE`, `DEFAULT_TIMEOUT`)
  - Static lifetimes/Generics: `'a`, `'static`, `T`, `E`, `Self`

### 4.2 Raw Identifiers
To use a keyword as an identifier, the prefix `r#` is used:
```sora
r#match = "exact"
r#type = 42
```

---

## 5. Keywords & Reserved Words

Sorayunara maintains a strict budget of **100 keywords**, structured into three deterministic tiers:

### 5.1 Core Keywords (Reserved in All Scopes)
```
fn          let         mut         const       struct      enum        type
trait       impl        operator    if          else        match       is
while       for         in          loop        break       continue    return
import      export      mod         pub         use         as          from
true        false       null        self        Self        unsafe      move
defer       guard       comptime    assert      test        where       derive
extern      async       await       task        spawn       chan        scope
```

### 5.2 Contextual Keywords (Positional Grammar)
```
get         set         default     dyn         static      atomic      pinned
region      arena       inline      packed      transparent override    abstract
lazy        catch       throw       try         yield       final       sealed
```

---

## 6. Literals

### 6.1 Integer Literals
Integers support decimal, hexadecimal, octal, and binary bases. Optional underscores (`_`) may be inserted for visual grouping:
- **Decimal**: `42`, `1_000_000`
- **Hexadecimal**: `0x2A`, `0xFF_AA_00`
- **Octal**: `0o52`, `0o755`
- **Binary**: `0b101010`, `0b1111_0000`

Explicit type suffixes: `42i8`, `42i16`, `42i32`, `42i64` (default `Int`), `42u8` (`Byte`), `42u16`, `42u32`, `42u64`, `42usize`.

### 6.2 Floating-Point Literals
Floating-point literals follow standard IEEE 754 representations:
- `3.14159265`
- `1.0e-5`, `2.5E+10`
- Explicit suffixes: `3.14f32`, `3.14f64` (default `Float`).

### 6.3 Boolean & Null Literals
- `true` and `false` (type `Bool`)
- `null` (represents empty reference/pointer state in FFI / pointer domains)

### 6.4 Character Literals
Enclosed in single quotes (`'...'`), supporting UTF-8 scalar values and escape sequences:
- `'a'`, `'🚀'`, `'\n'`, `'\t'`, `'\''`, `'\\'`, `'\0'`, `'\x7F'`, `'\u{1F600}'`

### 6.5 String Literals
- **Standard String**: Enclosed in double quotes (`"..."`). Supports interpolation and escapes:
  ```sora
  "Hello, World!\n"
  "Value: ${x + 1}"
  ```
- **Raw String**: `r"..."` or `r#"..."#` (does not interpret escape characters).
- **Multi-line / Block String**: Triple double quotes `"""..."""`.

---

## 7. Operators & Punctuators

| Category | Operators |
|---|---|
| **Arithmetic** | `+`, `-`, `*`, `/`, `%`, `**` |
| **Bitwise** | `&`, `\|`, `^`, `~`, `<<`, `>>` |
| **Logical** | `&&`, `\|\|`, `!` |
| **Comparison** | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| **Assignment** | `=`, `:=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `\|=`, `^=`, `<<=`, `>>=` |
| **Pipelines & Arrows** | `\|>`, `=>`, `->`, `<-` |
| **Option & Result Sugar**| `?`, `??`, `?.`, `!`, `?!` |
| **Ranges** | `..`, `..=` |
| **Punctuation** | `(`, `)`, `{`, `}`, `[`, `]`, `,`, `.`, `:`, `::`, `;`, `@`, `#` |
