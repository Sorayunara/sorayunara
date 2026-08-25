# 🌌 Sorayunara Ubiquitous Multi-Target & Tier Architecture

> **Strategic Architecture Blueprint for Universal Execution**: From 8-bit / 32-bit Microcontrollers to Cloud Supercomputers, Web Browsers, and Edge Runtimes.

---

## 🏛️ 1. Quad-Tier Multi-Target Backend Architecture

Sorayunara achieves complete hardware and platform independence through four distinct execution and codegen backends:

```
                               ┌─────────────────────────────┐
                               │   Sorayunara Source (.sora) │
                               └──────────────┬──────────────┘
                                              │
                                 [Compiler Frontend & HIR]
                                              │
                                ┌─────────────▼─────────────┐
                                │ HM Type & Borrow Checker  │
                                └─────────────┬─────────────┘
                                              │
                                  [Mid-Level IR (MIR)]
                                              │
        ┌───────────────────────┬─────────────┴─────────────┬───────────────────────┐
        ▼                       ▼                           ▼                       ▼
┌───────────────┐       ┌───────────────┐           ┌───────────────┐       ┌───────────────┐
│ LLVM Native   │       │ WebAssembly   │           │ C99 Fallback  │       │ Bytecode VM   │
│ Backend       │       │ & WASI        │           │ Transpiler    │       │ Interpreter   │
└───────┬───────┘       └───────┬───────┘           └───────┬───────┘       └───────┬───────┘
        │                       │                           │                       │
 ┌──────▼──────┐         ┌──────▼──────┐             ┌──────▼──────┐         ┌──────▼──────┐
 │ x86, ARM64, │         │ Web Browser │             │ ESP32, STM32│         │ Instant CLI │
 │ RISC-V, s390│         │ Edge / WASI │             │ Zephyr, RTOS│         │ Zero-Tooling│
 └─────────────┘         └─────────────┘             └─────────────┘         └─────────────┘
```

### 1.1 LLVM Backend (Native High-Performance Target)
- **Target ISAs**: `x86_64`, `aarch64`, `armv7`, `riscv32/64`, `powerpc64le`, `mips64`, `loongarch64`, `s390x`.
- **Target OS**: Linux (Ubuntu, Debian, RHEL, Arch), Windows (MSVC / GNU), macOS (Apple Silicon & Intel), FreeBSD, OpenBSD, NetBSD, Solaris, AIX.
- **Role**: Maximum optimization passes, SIMD vectorization, LTO (Link-Time Optimization), and zero-cost abstraction compilation.

### 1.2 WebAssembly & WASI (Web, Sandbox & Edge Target)
- **Targets**: `wasm32-unknown-unknown` (Browser Canvas/DOM/WebGPU) and `wasm32-wasi` (Cloudflare Workers, Fastly Compute, Node.js, Deno, Wasmtime).
- **Role**: Sandboxed execution, serverless edge functions, and client-side web applications without JS glue overhead.

### 1.3 C-Transpiler / C99 Fallback (Embedded & Retro Target)
- **Target**: ISO/IEC 9899:1999 (Pure C99) with zero external runtime dependencies.
- **Embedded Platforms & RTOS**: FreeRTOS, Zephyr OS, NuttX, ESP-IDF (ESP32), STM32 (HAL/LL), Arduino, Raspberry Pi RP2040, Teensy.
- **Role**: Guaranteed compilation on legacy architectures, DSPs, automotive chips, and bare-metal environments lacking native LLVM targets.

### 1.4 Bytecode VM / Standalone Portable Interpreter
- **Target**: Compact stack VM implemented in strict ANSI C / pure Rust.
- **Role**: Instant script evaluation (`sora run script.sora`), REPL sessions, configuration scripts, and constrained environments with no compiler installed.

---

## 📦 2. Standard Library Layering (`core`, `alloc`, `std`)

To support resource-constrained microcontrollers ($\le 32 \text{ KB RAM}$) up to enterprise clusters, the standard library is strictly segmented:

| Tier | Namespace | Target Environments | Available Features | Memory Model |
| :--- | :--- | :--- | :--- | :--- |
| **Tier 0** | `core` (`no_std`) | Bare-metal, Bootloaders, Microcontrollers (STM32, ESP32, AVR, RP2040), RTOS kernel | Bit manipulation, fixed-size arrays, numeric primitives, atomic operations, format traits. | **Zero Heap** (Stack & static only, no dynamic allocation). |
| **Tier 1** | `alloc` | Smart contracts, micro-embedded with SRAM, pure WASM sandbox | `String`, `Vec<T>`, `HashMap<K, V>`, `Box<T>`, dynamic memory management. | **Heap Allocation** (No OS filesystem or OS thread APIs). |
| **Tier 2** | `std` (Full OS) | Desktop, Server, Mobile, Cloud, Consoles (Windows, Linux, macOS, Android, iOS) | File I/O, non-blocking TCP/UDP sockets, OS multi-threading, Actor mailboxes, Process management, GPU compute bindings. | **Full OS Integration** (POSIX / Win32 / Apple frameworks). |

---

## 🎨 3. Universal UI & Graphics Strategy

```
                                  ┌───────────────────────────┐
                                  │   Sorayunara UI Layer     │
                                  └─────────────┬─────────────┘
                                                │
         ┌──────────────────────────────┬───────┴──────────────────────┬──────────────────────────────┐
         ▼                              ▼                              ▼                              ▼
┌──────────────────┐          ┌──────────────────┐           ┌──────────────────┐           ┌──────────────────┐
│ Headless / TUI   │          │ Web / PWA Canvas │           │ Desktop Native   │           │ XR / Spatial / TV│
│ ANSI Escape Code │          │ WebGPU / WebGL   │           │ Vulkan / DirectX │           │ Metal / OpenGL ES│
└──────────────────┘          └──────────────────┘           └──────────────────┘           └──────────────────┘
```

1. **Headless / ANSI TUI**: Zero-dependency terminal rendering for CLI utilities, servers, and embedded headless devices (routers, OpenWrt).
2. **Web-Based UI (Canvas & WebGPU)**: PWA integration for smart TVs (Samsung Tizen, LG webOS), ChromeOS, mobile PWAs, and web browsers.
3. **Hardware-Accelerated Native Graphics (HAL)**:
   - **Vulkan / WebGPU / OpenGL ES**: Linux, Android, Smart TVs, and XR/VR headsets.
   - **Metal**: Apple ecosystem (macOS, iOS, iPadOS, visionOS, watchOS).
   - **DirectX 12 / Direct3D**: Windows desktop, Windows Gaming, and Xbox.

---

## ⚙️ 4. Cross-Compilation CI/CD & Emulation Pipeline

The automated multi-platform compilation matrix utilizes:

- **Universal C/C++ Cross-Toolchain**: `zig cc` and `cross-rs` for glibc/musl compatibility without vendor SDK lock-in.
- **Windows Cross-Target**: `cargo-xwin` for building Windows MSVC binaries directly from Linux CI runners.
- **QEMU User Emulation Matrix**: Automated execution of test suites across non-x86 architectures:
  ```yaml
  strategy:
    matrix:
      arch: [x86_64, aarch64, riscv64, s390x, armv7, wasm32]
  ```
- **Hermetic Build Verification**: Checksum generation (`SHA256SUMS.txt`) and automated smoke testing verifying complete compiler toolchain passes (`check` $\rightarrow$ `build` $\rightarrow$ `test` $\rightarrow$ `run`).
