# 🧪 Sorayunara Native Testing Framework & Test Runner
**Subsystem**: Built-in Unit, Integration, Snapshot & Property-Based Testing

---

## 1. Writing Tests (`test "name" { ... }`)
```sora
// In any .sora source file or tests/ directory
test "vector push and pop operations" {
    let mut vec = Vector::new();
    vec.push(10);
    vec.push(20);
    assert(vec.len() == 2);
    assert(vec.pop() == Option::Some(20));
}
```

---

## 2. Test Execution & Flags
```powershell
sorayunara test                  # Run all tests in project
sorayunara test --coverage       # Generate line and branch coverage report
sorayunara test --fuzz           # Run continuous fuzzing mutations
```
