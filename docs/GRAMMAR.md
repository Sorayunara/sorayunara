# 📐 Sorayunara Formal Grammar Specification (Syntax v2)
**Reference Document**: `Sorayunara_Syntax_v2_Design_Proposal.pdf`  
**Version**: 2.2.0-LTS · **Extension**: `.sora` · **Principles**: Short to write, clear to read, powerful underneath.

---

## 1. Core Syntax Overview

| Feature | Syntax v2 Canonical Representation | Description |
|---|---|---|
| **Immutable variable** | `let x = 10` | Inferred, affine-linear variable |
| **Mutable variable** | `var x = 10` (or `let mut x = 10`) | Explicitly mutable binding |
| **Short function** | `fn add(a, b) => a + b` | Arrow expression body |
| **Block function** | `fn process(data) { => y }` | Standard block with return |
| **Output** | `say "Hello"` | Shorthand console print |
| **Conditional** | `if age >= 18 { ... } else { ... }` | Condition branching |
| **Loop & Range** | `for x in items { ... }`, `for i in 0..10 { ... }` | Iterator and range loop |
| **Infinite loop** | `loop { ... }` | Unconditional loop with `break`/`continue` |
| **Type / Struct** | `type User { name: String, age: Int }` | Struct / Product type definition |
| **Enum** | `enum Result<T, E> { Ok(T), Err(E) }` | Sum / Algebraic Data Type |
| **Pattern matching** | `match x { Ok(val) => say val, Err(e) => say e }` | Exhaustive match expression |
| **Async / Await** | `async fn work() { let m = await ch.recv()? }` | Async function and await |
| **Actor message** | `worker ! Message("hello")` | Zero-copy actor mailbox send |
| **Pipeline** | `let res = data \|> validate \|> transform` | Infix pipeline chaining |
| **Error propagation** | `let user = db.find(id)?` | Shorthand `Result`/`Option` unwrap |
| **Null coalescing** | `let name = user?.name ?? "Unknown"` | Safe navigation and fallback |
| **Import & Module** | `use std.io`, `use std.{io, channel, fs}`, `mod user` | Module system |

---

## 2. Formal EBNF Grammar

```ebnf
Program ::= ( UseDecl | ModDecl | TopLevelItem )* EOF ;

UseDecl ::= "use" QualifiedIdent ( "as" Ident | "::" "{" IdentList "}" )? ;
ModDecl ::= "mod" Ident ;

TopLevelItem ::= ( "pub" )? ( FunctionDecl | TypeDecl | EnumDecl | TraitDecl | ImplDecl | ServiceDecl ) ;

TypeDecl ::= "type" Ident ( "<" GenericParams ">" )? "{" ( FieldDecl ( "," FieldDecl )* ( "," )? )? "}" ;
FieldDecl ::= Ident ":" Type ;

EnumDecl ::= "enum" Ident ( "<" GenericParams ">" )? "{" ( EnumVariant ( "," EnumVariant )* ( "," )? )? "}" ;
EnumVariant ::= Ident ( "(" ( Type ( "," Type )* )? ")" )? ;

FunctionDecl ::= ( "async" )? "fn" Ident "(" ( ParamList )? ")" ( "->" Type )? ( Block | "=>" Expr ) ;

Stmt ::= LetStmt
       | VarStmt
       | SayStmt
       | IfStmt
       | ForStmt
       | LoopStmt
       | WhileStmt
       | ReturnStmt
       | ExprStmt ;

LetStmt ::= "let" Ident ( ":" Type )? "=" Expr ;
VarStmt ::= "var" Ident ( ":" Type )? "=" Expr ;
SayStmt ::= "say" Expr ;
ReturnStmt ::= ( "return" | "=>" ) ( Expr )? ;

Expr ::= PrimaryExpr ( BinaryOp Expr | PipelineOp Expr | QuestionOp | CoalesceOp )* ;
PipelineOp ::= "|>" Expr ;
CoalesceOp ::= "??" Expr ;
```
