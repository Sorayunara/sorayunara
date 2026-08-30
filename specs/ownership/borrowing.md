# Sorayunara Ownership, Move Semantics & Borrow Checker Specification

## 1. Core Invariants
- Each value has exactly one owner at any point in execution.
- Passing a non-Copy value transfers ownership (Move).
- Multiple immutable borrows `&T` allowed concurrently, XOR exactly one mutable borrow `&mut T`.
