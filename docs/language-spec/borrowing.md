# 🛡️ Sorayunara Formal Language Specification: Borrowing & References

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/borrowing.md`

---

## 1. References & Borrowing Invariants

Borrowing creates temporary pointers to data without transferring ownership. The compiler's borrow checker ([`bootstrap/src/semantics.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/semantics.rs)) strictly enforces the following invariants:

$$\forall x \in \text{Resources}, \quad \big(\text{Count}(\&x) \ge 0 \land \text{Count}(\&\text{mut } x) == 0\big) \lor \big(\text{Count}(\&x) == 0 \land \text{Count}(\&\text{mut } x) == 1\big)$$

1. **Shared References (`&T`)**: Any number of concurrent immutable borrows may exist simultaneously.
2. **Exclusive References (`&mut T`)**: Only ONE mutable borrow may exist at any time.
3. **No Aliasing with Mutability**: You may NEVER have a mutable reference simultaneously with any other reference (mutable or immutable).

---

## 2. Shared Borrows (`&T`)

Shared references allow read-only inspection of the underlying data:

```sora
fn calculate_length(text: &String) -> Int {
    text.len()
}

msg = String.from("Sorayunara")
len = calculate_length(&msg) // Borrowed immutably
println("${msg} has length ${len}") // msg remains valid
```

---

## 3. Mutable Borrows (`&mut T`)

Mutable references allow modifying data in place without reallocating or moving ownership:

```sora
fn append_suffix(target: &mut String) {
    target.push_str(" · 🌌")
}

mut banner = String.from("Sorayunara")
append_suffix(&mut banner)
println(banner) // "Sorayunara · 🌌"
```

---

## 4. Borrow Conflicts & Aliasing Detection

```sora
mut list = [1, 2, 3]

r1 = &list
r2 = &list // OK: Multiple shared borrows

// COMPILE ERROR: E0502 - Cannot borrow `list` as mutable because it is also borrowed as immutable
// r3 = &mut list 

println("${r1}, ${r2}")
```

### 4.1 Iterator Invalidation Prevention
```sora
mut numbers = [10, 20, 30]

for n in &numbers {
    // COMPILE ERROR: E0506 - Cannot mutate `numbers` while iterating over `&numbers`
    // numbers.push(40) 
    println(n)
}
```
