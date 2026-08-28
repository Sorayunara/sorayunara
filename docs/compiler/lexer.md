# 🔍 Compiler: Lexer Subsystem (`compiler/lexer/`)

The lexer performs zero-allocation streaming tokenization of UTF-8 Sorayunara source code.

---

## Key Features
- **Deterministic Tokenization**: Finite State Machine (FSM) scanner.
- **Span Tracking**: Tracks byte offsets, line numbers, and column numbers for rich compiler diagnostics.
- **Strict Keyword Budget**: Enforces the 100-keyword maximum language limit.
