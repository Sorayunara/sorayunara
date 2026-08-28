# 🌌 Sorayunara Formal Language & Type System Specification
**Language Design Document · Affine-Linear Algebraic Effect Architecture**  
**Version**: 2.1.0-LTS · **Status**: Formal Blueprint Specification

---

## 🏛️ 1. Core Architectural Pillars

Sorayunara (`.sora`) combines four advanced language paradigms into a unified, zero-overhead developer experience:

```
                          ┌─────────────────────────────────────────────────┐
                          │            Sorayunara Type Engine               │
                          └─────────────────────────────────────────────────┘
                                   │                              │
          ┌────────────────────────┴─────────┐   ┌────────────────┴────────────────────────┐
          │  Affine-Linear Memory Model      │   │  Algebraic Effect System                │
          │  • Deterministic (Zero-GC)       │   │  • Resumable Continuations              │
          │  • Move & Region-based Lifetimes │   │  • Explicit Effect Signatures           │
          └──────────────────────────────────┘   └─────────────────────────────────────────┘
                                   │                              │
          ┌────────────────────────┴─────────┐   ┌────────────────┴────────────────────────┐
          │  Actor & Fiber Concurrency       │   │  Category-Theoretic Trait System        │
          │  • Work-Stealing M:N Scheduler   │   │  • Higher-Kinded Types (HKT F[_])       │
          │  • Shared-Nothing Message Passing│   │  • Monadic / Functor Composition        │
          └──────────────────────────────────┘   └─────────────────────────────────────────┘
```

---

## 📜 2. Syntax & Grammar Formalization

### 2.1 Higher-Kinded Types (HKT) & Traits
```sora
module Core.Algebra.Stream;

/// Trait with Higher-Kinded Type abstraction F[_]
pub trait Streamable[F[_]] {
    fn stream[A](self: F[A]) -> Channel[A];
    fn map[A, B](self: F[A], transform: fn(A) -> B) -> F[B];
}
```

### 2.2 Linear-Constrained Algebraic Data Types (ADTs)
```sora
module Core.Network.Session;

/// Algebraic Data Types with explicit linear resource constraints
pub type ConnectionState = 
    | Disconnected
    | Connecting(attempt: U32)
    | Connected(linear session: SocketSession);
```

### 2.3 Algebraic Effects & Handler Declarations
```sora
module Core.Effects.Network;

/// Declares an algebraic effect operation
effect NetworkIO {
    fn emit_packet(bytes: Bytes) -> Result[Unit, NetError];
    fn read_packet(timeout_ms: U64) -> Result[Bytes, NetError];
}

effect Logger {
    fn info(msg: Str);
    fn warn(msg: Str);
    fn error(msg: Str);
}
```

### 2.4 Services with Effect Tracking & Concurrency Fibers
```sora
module Core.Network.Pipeline;

import Core.Algebra.Stream::{Streamable};
import Core.Network.Session::{ConnectionState, SocketSession};
import Core.Effects.Network::{NetworkIO, Logger};
import Std.Async::{Fiber, Channel};

service PacketRouter[T: Streamable] with [NetworkIO, Logger] {
    linear socket: SocketSession,
    stream_pipeline: T,

    pub async fn dispatch(&mut self, payload: Payload) -> Result[Unit, RouterError] {
        match self.socket.state() {
            Connected(session) => {
                Logger.info("Routing payload through active channel...");
                
                // Concurrent lightweight fiber dispatch with move semantics
                Fiber.spawn(move || {
                    let encoded = payload.serialize()?;
                    NetworkIO.emit_packet(encoded)?;
                    Ok(())
                }).await?
            },
            _ => Err(RouterError.Unavailable),
        }
    }
}
```

---

## 🔬 3. Formal Complexity & Lowering Matrix

| Subsystem | Underlying Theory | Lowering Strategy (MIR / LLVM IR) | Compiler Complexity |
|---|---|---|---|
| **Memory Model** | Affine Logic & Region Lifetimes | Zero-cost move semantics, destructors emitted at drop sites (no runtime overhead). | **High** (Liveness analysis + borrow checker) |
| **Effect System** | Delimited Continuations / CPS | Continuations desugared into state machines / CPS in Sora-MIR before LLVM emission. | **Very High** (Continuation-Passing Transform) |
| **Type System** | System $F_\omega$ + Refinements | Bidirectional constraint solver with Hindley-Milner type inference. | **High** (Higher-order unification) |
| **Concurrency** | CSP + Actor Mailboxes | Lock-free ring buffers + work-stealing green thread fiber runtime. | **Medium-High** (Non-blocking queue scheduler) |

---

## 🚀 4. Compiler Pipeline & Phasing (Rust Implementation)

```
.sora Source Code
       ↓
Lexer & Pratt Expression Parser (Infix, pipeline `|>`, bracket generics `T[_]`)
       ↓
Abstract Syntax Tree (AST with Effect Nodes)
       ↓
Type Checker (System F_omega + HKT Solver + Hindley-Milner)
       ↓
Borrow & Affine Ownership Checker (Linear Resource Enforcement)
       ↓
Sora-HIR (High-Level Intermediate Representation)
       ↓
Effect Desugaring (Delimited Continuations CPS Transform)
       ↓
Sora-MIR (Control Flow Graph Optimization)
       ↓
Code Generation (LLVM IR / Native MSVC / WASM Backend)
```
