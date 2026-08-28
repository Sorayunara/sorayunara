# 🚀 Getting Started with Sorayunara

A quick guide to writing, compiling, and running your first `.sora` program.

---

## 1. Installation
Clone the repository and build the standalone `sora` binary:
```bash
cargo build --release
```

---

## 2. Hello World
Create `hello.sora`:
```sora
fn main() -> Int {
    print("🌌 Hello from Sorayunara!")
    return 0
}
```

---

## 3. Run and Build
```bash
# Execute in VM
sora run hello.sora

# Compile Ahead-of-Time
sora build hello.sora --target native
```
