# 🔬 Sorayunara Virtual Machine & Bytecode Internals

The Sorayunara VM is a stack-based bytecode execution engine.

---

## Instruction Set Overview

| Opcode | Arguments | Description |
|---|---|---|
| `PushInt` | `i64` | Push integer constant onto evaluation stack. |
| `PushStr` | `String` | Push string constant onto evaluation stack. |
| `Load` | `String` | Load variable value from local frame onto stack. |
| `Store` | `String` | Pop value from stack and store into local frame variable. |
| `Add` / `Sub` / `Mul` / `Div` | None | Pop binary operands, evaluate arithmetic / logic, push result. |
| `Equal` / `Less` / `Greater` | None | Pop comparison operands, push boolean result. |
| `Jump` | `usize` | Unconditional jump to bytecode instruction offset. |
| `JumpIfFalse` | `usize` | Pop boolean condition; if false, jump to target offset. |
| `Call` | `String, usize` | Push new call frame and execute target function with argc. |
| `Return` | None | Pop call frame and push return value to caller stack. |
