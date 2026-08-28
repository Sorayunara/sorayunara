# 🌲 Compiler: Parser Subsystem (`compiler/parser/`)

The Sorayunara parser uses a recursive descent Pratt parsing algorithm with explicit binding power for operator precedence.

---

## Grammar Architecture
- **Statements**: Declarations (`let`, `mut`, `const`, `fn`, `struct`, `enum`, `trait`, `impl`).
- **Expressions**: Binary operations, unary operations, control-flow expressions (`if`, `match`), lambda arrow functions (`=>`).
- **Error Recovery**: Synchronizes on semicolons or block boundaries to report multiple distinct syntax errors per compilation pass.
