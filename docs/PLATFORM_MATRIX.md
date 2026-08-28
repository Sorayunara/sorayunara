# 🌐 Sorayunara (.sora) Universal Platform Target Matrix

**Specification Version**: 2.0.0 (LTS Architecture)  
**Document ID**: [`Sorayunara_Sora_Universal_Platform_Target_Matrix.pdf`](../Sorayunara_Sora_Universal_Platform_Target_Matrix.pdf)  
**Multi-Target Backend Engine**: LLVM Native IR / ANSI C99 / WebAssembly WASI / Bytecode VM  

---

## 🏛️ Multi-Target Compiler Pipeline

```
                           SORAYUNARA SOURCE CODE (.sora)
                                         │
                                         ▼
                     ┌───────────────────────────────────────┐
                     │   Unified Frontend & Type Checker     │
                     │  HM Type Inference & Borrow Checking  │
                     └───────────────────┬───────────────────┘
                                         │
                                         ▼
                     ┌───────────────────────────────────────┐
                     │    Three-Address Code Intermediate    │
                     │   Representation (Bytecode / MIR)     │
                     └───────────────────┬───────────────────┘
                                         │
          ┌──────────────────────────────┼──────────────────────────────┐
          ▼                              ▼                              ▼
 ┌──────────────────┐           ┌──────────────────┐           ┌──────────────────┐
 │  LLVM Native IR  │           │ ANSI C99 / C11   │           │ WebAssembly Text │
 │   Emission (.ll) │           │ Transpiler (.c)  │           │ & Binary (.wasm) │
 └────────┬─────────┘           └────────┬─────────┘           └────────┬─────────┘
          │                              │                              │
          ▼                              ▼                              ▼
 ┌──────────────────┐           ┌──────────────────┐           ┌──────────────────┐
 │ Desktop, Servers │           │ Embedded, MCUs,  │           │ Web Browsers,    │
 │ Mobile OS, Cloud │           │ RTOS, Legacy C   │           │ Cloudflare Edge  │
 └──────────────────┘           └──────────────────┘           └──────────────────┘
```

---

## 📑 1. Tier 1: Production Desktop, Cloud & Server Platforms (100% CI Guaranteed)

| Target Triple | Operating System | Architecture | Backend Engine | ABI / Linker |
| :--- | :--- | :--- | :--- | :--- |
| `x86_64-unknown-linux-gnu` | Linux (Ubuntu, Debian, RHEL) | x86_64 | LLVM Native | SysV ABI / GNU ld |
| `x86_64-unknown-linux-musl` | Linux Static (Alpine, Docker) | x86_64 | LLVM Native | Static musl |
| `aarch64-unknown-linux-gnu` | Linux (AWS Graviton, Pi 4/5) | ARM64 | LLVM Native | AAPCS64 / lld |
| `aarch64-unknown-linux-musl`| Linux Static ARM (Containers) | ARM64 | LLVM Native | Static musl |
| `x86_64-pc-windows-msvc` | Windows 10, 11, Server | x86_64 | LLVM Native | MSVC link.exe |
| `aarch64-pc-windows-msvc` | Windows on ARM (Snapdragon) | ARM64 | LLVM Native | MSVC ARM64 |
| `aarch64-apple-darwin` | macOS (Apple Silicon M1..M4) | ARM64 | LLVM Native | Mach-O ld64 |
| `x86_64-apple-darwin` | macOS (Intel Core / Xeon) | x86_64 | LLVM Native | Mach-O ld64 |
| `wasm32-unknown-unknown` | Web Browsers (Chrome, Safari) | WebAssembly | WASM Backend | wasm-ld |
| `wasm32-wasi` | Cloud Edge (Cloudflare, Deno) | WASI POSIX | WASM Backend | wasm-ld |

---

## 📑 2. Tier 2: Mobile & Unix Ecosystem Platforms

| Target Triple | Platform / Ecosystem | Architecture | Toolchain & Linking Strategy |
| :--- | :--- | :--- | :--- |
| `aarch64-linux-android` | Android 8.0+ (ARM64 Native) | ARM64 | Android NDK / JNI Wrapper |
| `armv7-linux-androideabi` | Android Legacy (32-bit) | ARMv7-A | Android NDK / Thumb-2 |
| `aarch64-apple-ios` | iOS / iPadOS / watchOS | ARM64 | Xcode iOS SDK / C-ABI Framework |
| `x86_64-apple-ios` | iOS Simulator (Intel Mac) | x86_64 | Xcode iOS Simulator SDK |
| `riscv64gc-unknown-linux-gnu`| RISC-V 64 Linux Servers | RV64GC | LLVM RISCV64 / GNU ABI |
| `x86_64-unknown-freebsd` | FreeBSD 13, 14 | x86_64 | LLVM Native / FreeBSD ELF |
| `x86_64-unknown-openbsd` | OpenBSD Secure OS | x86_64 | LLVM Native / OpenBSD ELF pledge |

---

## 📑 3. Tier 3: Bare-Metal Microcontrollers, IoT & RTOS

| Target Triple | Hardware Platform / Chipset | CPU Architecture | RAM / Flash Budget |
| :--- | :--- | :--- | :--- |
| `thumbv6m-none-eabi` | RP2040, STM32F0, SAMD21 | ARM Cortex-M0/M0+ | &ge; 16 KB RAM / 64 KB Flash |
| `thumbv7m-none-eabi` | STM32F1, STM32F3, NXP LPC | ARM Cortex-M3 | &ge; 32 KB RAM / 128 KB Flash |
| `thumbv7em-none-eabihf` | STM32F4, STM32F7, Teensy 4.0 | Cortex-M4F / M7F | &ge; 64 KB RAM / 256 KB Flash |
| `riscv32imc-unknown-none-elf`| ESP32-C3, GD32V, Kendryte | RISC-V 32 (RV32IMC) | &ge; 32 KB RAM / 128 KB Flash |
| `xtensa-esp32-none-elf` | ESP32, ESP8266, ESP32-S3 | Xtensa LX6 / LX7 | &ge; 128 KB RAM / 512 KB Flash |
| `avr-unknown-none` | Arduino Uno, Nano, Mega 2560 | 8-bit AVR ATmega328P | &ge; 2 KB RAM / 32 KB Flash |
| `c99-generic-embedded` | Any microcontroller with C compiler | Custom / Automotive MCU | Arbitrary Micro-runtime |

---

## 📑 4. Tier 4: GPU, AI Accelerators & Heterogeneous Compute

| Target Driver | Hardware Ecosystem | Output Format | Compute Model |
| :--- | :--- | :--- | :--- |
| `nvptx64-nvidia-cuda` | NVIDIA GPUs (RTX, A100, H100) | PTX Bytecode / CUDA C | Massively Parallel SIMT Kernels |
| `amdgcn-amd-amdhsa` | AMD Radeon / Instinct (ROCm) | AMDGCN Machine Code | Heterogeneous Compute Kernels |
| `spirv-unknown-unknown`| Vulkan, OpenCL, Intel oneAPI | SPIR-V Binary Shader | Standard GPU Compute Shaders |
| `wgsl-webgpu` | Browser WebGPU Compute | WGSL Text Shaders | Web-accelerated Tensor Ops |

---

## 🛠️ CLI Cross-Compilation Commands

```bash
# Compile for Linux Server
sora build --target x86_64-unknown-linux-gnu main.sora

# Compile for Apple Silicon Mac
sora build --target aarch64-apple-darwin main.sora

# Compile for Windows x86_64
sora build --target x86_64-pc-windows-msvc main.sora

# Compile for Cloud WebAssembly Edge
sora build --target wasm32-wasi main.sora

# Compile for Embedded Microcontrollers (STM32 / RP2040)
sora build --target thumbv7em-none-eabihf --no-std main.sora

# Transpile to Portable ANSI C99
sora build --target c99-generic-embedded main.sora
```
