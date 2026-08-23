# Contributing to Sorayunara (`.sora`)

Thank you for your interest in contributing to the **Sorayunara** programming language! We welcome contributions from developers of all skill levels.

---

## 🌟 How Can You Contribute?

### 1. New to Open Source? Look for `good first issue`
Check out our beginner-friendly issues on the [Issue Tracker](https://github.com/Sorayunara/sorayunara/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).

### 2. Write Real `.sora` Sample Programs
Add new algorithms, CLI utilities, or web service examples in [`examples/`](examples/).

### 3. Improve Standard Library (`std/`)
Help optimize and add functions to any of the 37 standard library modules in [`std/*.sora`](std/).

### 4. Enhance VS Code Extension
Improve syntax highlighting, autocompletion, or snippets in [`editors/vscode/`](editors/vscode/).

---

## 🚀 Development Workflow

### 1. Fork & Clone
```bash
git clone https://github.com/<your-username>/sorayunara.git
cd sorayunara
```

### 2. Build the Toolchain
```bash
cargo build --release
```

### 3. Run the Test Suite
```bash
cargo test --all-targets
```

### 4. Create a Feature Branch & Open a PR
```bash
git checkout -b feat/my-new-feature
git commit -m "feat: add my new feature"
git push origin feat/my-new-feature
```

Open a Pull Request on GitHub and our team will review it promptly!
