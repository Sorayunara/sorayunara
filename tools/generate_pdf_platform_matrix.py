import os
import subprocess
import sys

html_content = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Sorayunara (SORA) Universal Platform Target Matrix 2.0</title>
<style>
  @page {
    size: A4;
    margin: 16mm 14mm 16mm 14mm;
    @bottom-right {
      content: counter(page);
    }
  }
  body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, Helvetica, Arial, sans-serif;
    color: #1a1a2e;
    line-height: 1.5;
    font-size: 10pt;
    margin: 0;
    padding: 0;
  }
  .cover {
    page-break-after: always;
    text-align: center;
    padding-top: 70px;
  }
  .cover-title {
    font-size: 26pt;
    font-weight: 800;
    color: #4A35B0;
    margin: 0 0 10px 0;
    letter-spacing: -0.5px;
  }
  .cover-subtitle {
    font-size: 13pt;
    font-weight: 600;
    color: #555;
    margin-bottom: 25px;
  }
  .badge {
    display: inline-block;
    background: #6C5CE7;
    color: #ffffff;
    font-size: 8.5pt;
    font-weight: 700;
    padding: 4px 10px;
    border-radius: 16px;
    margin: 3px;
  }
  .cover-meta {
    margin-top: 50px;
    font-size: 9.5pt;
    color: #666;
    border-top: 1px solid #e0e0e0;
    display: inline-block;
    padding-top: 20px;
    text-align: left;
  }
  .cover-meta table td {
    padding: 4px 10px;
  }
  h1 {
    font-size: 16pt;
    color: #2D3436;
    border-bottom: 2px solid #6C5CE7;
    padding-bottom: 4px;
    margin-top: 24px;
    margin-bottom: 12px;
    page-break-after: avoid;
  }
  h2 {
    font-size: 12pt;
    color: #4A35B0;
    margin-top: 18px;
    margin-bottom: 6px;
    border-bottom: 1px solid #dfe6e9;
    padding-bottom: 2px;
    page-break-after: avoid;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 8px 0 14px 0;
    font-size: 8.5pt;
    page-break-inside: avoid;
  }
  th {
    background: #6C5CE7;
    color: #ffffff;
    font-weight: 700;
    text-align: left;
    padding: 5px 8px;
    border: 1px solid #5644d8;
  }
  td {
    padding: 4px 8px;
    border: 1px solid #dfe6e9;
    vertical-align: top;
  }
  tr:nth-child(even) {
    background: #f8f9fa;
  }
  pre, code {
    font-family: 'Consolas', 'Courier New', Courier, monospace;
  }
  pre {
    background: #23272e;
    color: #abb2bf;
    padding: 8px 12px;
    border-radius: 5px;
    font-size: 8pt;
    line-height: 1.4;
    margin: 6px 0 10px 0;
    page-break-inside: avoid;
  }
  code {
    background: #ececf7;
    color: #4a35b0;
    padding: 1px 4px;
    border-radius: 3px;
    font-size: 8.5pt;
  }
  pre code {
    background: transparent;
    color: inherit;
    padding: 0;
  }
  .tag-tier1 { background: #00b894; color: #fff; padding: 2px 6px; border-radius: 4px; font-weight: bold; font-size: 7.5pt; }
  .tag-tier2 { background: #0984e3; color: #fff; padding: 2px 6px; border-radius: 4px; font-weight: bold; font-size: 7.5pt; }
  .tag-tier3 { background: #e17055; color: #fff; padding: 2px 6px; border-radius: 4px; font-weight: bold; font-size: 7.5pt; }
  .tag-tier4 { background: #6c5ce7; color: #fff; padding: 2px 6px; border-radius: 4px; font-weight: bold; font-size: 7.5pt; }
  .alert-box {
    background: #f0edff;
    border-left: 4px solid #6C5CE7;
    padding: 8px 12px;
    margin: 8px 0;
    border-radius: 0 4px 4px 0;
    font-size: 9pt;
  }
  .page-break {
    page-break-after: always;
  }
</style>
</head>
<body>

<!-- COVER PAGE -->
<div class="cover">
  <div style="font-size: 48pt; color: #6C5CE7; line-height: 1; margin-bottom: 12px;">🌐</div>
  <div class="cover-title">Sorayunara (SORA)</div>
  <div class="cover-subtitle">Universal Platform Target Matrix & Cross-Compilation Specification v2.0</div>
  <div>
    <span class="badge">Desktop & Server (Linux, Windows, macOS)</span>
    <span class="badge">Mobile (Android, iOS)</span>
    <span class="badge">Web & Cloud Edge (WASM / WASI)</span>
    <span class="badge">Embedded & Microcontrollers (ARM Cortex, RISC-V, ESP32)</span>
    <span class="badge">GPU & Accelerators (CUDA, ROCm, WebGPU)</span>
  </div>

  <div class="cover-meta">
    <table>
      <tr><td><strong>Document ID:</strong></td><td><code>Sorayunara_Sora_Universal_Platform_Target_Matrix.pdf</code></td></tr>
      <tr><td><strong>Compiler Architecture:</strong></td><td>Ubiquitous Multi-Target (LLVM / C / WASM / JIT)</td></tr>
      <tr><td><strong>Official Repository:</strong></td><td><code>https://github.com/Sorayunara/sorayunara</code></td></tr>
      <tr><td><strong>Cross-Compilation CLI:</strong></td><td><code>sora build --target &lt;target-triple&gt;</code></td></tr>
      <tr><td><strong>Status:</strong></td><td>Official Production Specification (LTS)</td></tr>
      <tr><td><strong>License:</strong></td><td>MIT Open Source License</td></tr>
    </table>
  </div>
</div>

<!-- SECTION 1: ARCHITECTURAL OVERVIEW -->
<h1>1. Ubiquitous Platform Architecture Overview</h1>
<p>
Sorayunara (<code>.sora</code>) is engineered from the ground up to achieve <strong>100% universal platform portability</strong>. No matter what platform, hardware architecture, operating system, or execution environment a developer targets—from supercomputers to 8-bit microcontrollers, mobile smartphones, or browser sandboxes—Sorayunara provides first-class native compilation and runtime execution.
</p>

<div class="alert-box">
  <strong>Universal Portability Guarantee:</strong> Any platform possessing an LLVM backend, an ANSI C99/C11 compiler (GCC, Clang, MSVC, SDCC), or a WebAssembly virtual machine can build, run, and execute Sorayunara code natively without behavioral divergence.
</div>

<h2>Multi-Target Compiler Pipeline</h2>
<pre><code>                           SORAYUNARA SOURCE CODE (.sora)
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
</code></pre>

<!-- SECTION 2: TIER 1 TARGETS -->
<div class="page-break"></div>
<h1>2. Tier 1: Production Desktop, Cloud & Server Platforms</h1>
<p>
Tier 1 targets are guaranteed to build, run, and pass 100% of automated CI test matrices on every commit. Pre-compiled binaries and official installers are distributed for all Tier 1 architectures.
</p>

<table>
  <thead>
    <tr>
      <th>Target Triple</th>
      <th>Operating System</th>
      <th>Architecture</th>
      <th>Backend Engine</th>
      <th>ABI / Linker</th>
      <th>Tier</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>x86_64-unknown-linux-gnu</code></td>
      <td>Linux (Ubuntu, Debian, Fedora, RHEL)</td>
      <td>x86_64 (64-bit AMD/Intel)</td>
      <td>LLVM Native</td>
      <td>SysV ABI / GNU ld, lld</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>x86_64-unknown-linux-musl</code></td>
      <td>Linux Static (Alpine, Containers)</td>
      <td>x86_64</td>
      <td>LLVM Native</td>
      <td>SysV ABI / Static musl</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>aarch64-unknown-linux-gnu</code></td>
      <td>Linux (AWS Graviton, Ampere, Pi 4/5)</td>
      <td>ARM64 / AArch64</td>
      <td>LLVM Native</td>
      <td>AAPCS64 / lld</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>aarch64-unknown-linux-musl</code></td>
      <td>Linux Static ARM (Embedded Containers)</td>
      <td>ARM64 / AArch64</td>
      <td>LLVM Native</td>
      <td>AAPCS64 / Static musl</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>x86_64-pc-windows-msvc</code></td>
      <td>Microsoft Windows 10, 11, Server</td>
      <td>x86_64</td>
      <td>LLVM Native</td>
      <td>MSVC ABI / link.exe, lld-link</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>aarch64-pc-windows-msvc</code></td>
      <td>Windows on ARM (Snapdragon X Elite)</td>
      <td>ARM64</td>
      <td>LLVM Native</td>
      <td>MSVC ARM64 / lld-link</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>aarch64-apple-darwin</code></td>
      <td>macOS (Apple Silicon M1, M2, M3, M4)</td>
      <td>ARM64</td>
      <td>LLVM Native</td>
      <td>Darwin Mach-O / ld64</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>x86_64-apple-darwin</code></td>
      <td>macOS (Intel Core i5/i7/i9/Xeon)</td>
      <td>x86_64</td>
      <td>LLVM Native</td>
      <td>Darwin Mach-O / ld64</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>wasm32-unknown-unknown</code></td>
      <td>Modern Web Browsers (Chrome, Safari, Edge)</td>
      <td>WebAssembly 32-bit</td>
      <td>WASM Backend</td>
      <td>WASM Bytecode / wasm-ld</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
    <tr>
      <td><code>wasm32-wasi</code></td>
      <td>Cloud Edge (Cloudflare, Deno, Wasmtime)</td>
      <td>WebAssembly with WASI</td>
      <td>WASM Backend</td>
      <td>WASI POSIX / wasm-ld</td>
      <td><span class="tag-tier1">Tier 1</span></td>
    </tr>
  </tbody>
</table>

<!-- SECTION 3: TIER 2 TARGETS -->
<h1>3. Tier 2: Mobile & Unix Ecosystem Platforms</h1>
<p>
Tier 2 targets provide complete compiler support and automated cross-compilation toolchains for mobile apps, Unix derivatives, and next-generation RISC-V compute servers.
</p>

<table>
  <thead>
    <tr>
      <th>Target Triple</th>
      <th>Platform / Ecosystem</th>
      <th>Architecture</th>
      <th>Toolchain & Linking Strategy</th>
      <th>Tier</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>aarch64-linux-android</code></td>
      <td>Android 8.0+ (ARM64 Native NDK)</td>
      <td>ARM64</td>
      <td>Android NDK / Clang toolchain / JNI Wrapper</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>armv7-linux-androideabi</code></td>
      <td>Android Legacy (ARMv7 32-bit)</td>
      <td>ARMv7-A</td>
      <td>Android NDK / Thumb-2 mode</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>aarch64-apple-ios</code></td>
      <td>iOS / iPadOS / watchOS Devices</td>
      <td>ARM64</td>
      <td>Xcode iOS SDK / Static C-ABI Framework</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>x86_64-apple-ios</code></td>
      <td>iOS Simulator (macOS Intel)</td>
      <td>x86_64</td>
      <td>Xcode iOS Simulator SDK</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>riscv64gc-unknown-linux-gnu</code></td>
      <td>RISC-V 64 Linux (SiFive, StarFive)</td>
      <td>RISC-V 64 (RV64GC)</td>
      <td>LLVM RISCV64 / GNU ABI</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>x86_64-unknown-freebsd</code></td>
      <td>FreeBSD 13, 14</td>
      <td>x86_64</td>
      <td>LLVM Native / FreeBSD ELF</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
    <tr>
      <td><code>x86_64-unknown-openbsd</code></td>
      <td>OpenBSD Secure OS</td>
      <td>x86_64</td>
      <td>LLVM Native / OpenBSD ELF pledge/unveil</td>
      <td><span class="tag-tier2">Tier 2</span></td>
    </tr>
  </tbody>
</table>

<!-- SECTION 4: TIER 3 TARGETS -->
<div class="page-break"></div>
<h1>4. Tier 3: Bare-Metal Microcontrollers, IoT & Real-Time OS</h1>
<p>
Sorayunara provides zero-overhead execution for embedded electronics, IoT microcontrollers, and Real-Time Operating Systems (FreeRTOS, Zephyr, RT-Thread) through its direct ANSI C99 transpiler and embedded LLVM emitter.
</p>

<table>
  <thead>
    <tr>
      <th>Target Triple / Hardware</th>
      <th>Hardware Platform / Chipset</th>
      <th>CPU Architecture</th>
      <th>RAM / Flash Budget</th>
      <th>Tier</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>thumbv6m-none-eabi</code></td>
      <td>Raspberry Pi RP2040, STM32F0, SAMD21</td>
      <td>ARM Cortex-M0 / M0+</td>
      <td>&ge; 16 KB RAM / 64 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>thumbv7m-none-eabi</code></td>
      <td>STM32F1, STM32F3, NXP LPC1700</td>
      <td>ARM Cortex-M3</td>
      <td>&ge; 32 KB RAM / 128 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>thumbv7em-none-eabihf</code></td>
      <td>STM32F4, STM32F7, Teensy 4.0/4.1, nRF52840</td>
      <td>ARM Cortex-M4F / M7F (Hardware FPU)</td>
      <td>&ge; 64 KB RAM / 256 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>riscv32imc-unknown-none-elf</code></td>
      <td>Espressif ESP32-C3, GD32V, Kendryte K210</td>
      <td>RISC-V 32 (RV32IMC)</td>
      <td>&ge; 32 KB RAM / 128 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>xtensa-esp32-none-elf</code></td>
      <td>Espressif ESP32, ESP8266, ESP32-S3</td>
      <td>Xtensa LX6 / LX7 Dual-Core</td>
      <td>&ge; 128 KB RAM / 512 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>avr-unknown-none</code></td>
      <td>Arduino Uno, Nano, Mega 2560</td>
      <td>8-bit AVR (ATmega328P / 2560)</td>
      <td>&ge; 2 KB RAM / 32 KB Flash</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
    <tr>
      <td><code>c99-generic-embedded</code></td>
      <td>Any microcontroller with standard C compiler</td>
      <td>Custom / Automotive / Avionics MCU</td>
      <td>Arbitrary (Micro-runtime)</td>
      <td><span class="tag-tier3">Tier 3</span></td>
    </tr>
  </tbody>
</table>

<!-- SECTION 5: TIER 4 ACCELERATORS -->
<h1>5. Tier 4: GPU, AI Accelerators & Heterogeneous Compute</h1>
<p>
For high-performance AI, linear algebra, tensor computation, and graphics, Sorayunara emits high-performance kernels and compute shaders.
</p>

<table>
  <thead>
    <tr>
      <th>Target Driver / Backend</th>
      <th>Hardware Ecosystem</th>
      <th>Output Format</th>
      <th>Compute Model</th>
      <th>Tier</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>nvptx64-nvidia-cuda</code></td>
      <td>NVIDIA GPUs (GeForce, RTX, Tesla, A100, H100)</td>
      <td>PTX Bytecode / CUDA C</td>
      <td>Massively Parallel SIMT Kernels</td>
      <td><span class="tag-tier4">Tier 4</span></td>
    </tr>
    <tr>
      <td><code>amdgcn-amd-amdhsa</code></td>
      <td>AMD Radeon / Instinct (ROCm compute)</td>
      <td>AMDGCN Machine Code / HIP</td>
      <td>Heterogeneous Compute Kernels</td>
      <td><span class="tag-tier4">Tier 4</span></td>
    </tr>
    <tr>
      <td><code>spirv-unknown-unknown</code></td>
      <td>Vulkan, OpenCL, Intel oneAPI GPUs</td>
      <td>SPIR-V Binary Shader</td>
      <td>Standard GPU Compute Shaders</td>
      <td><span class="tag-tier4">Tier 4</span></td>
    </tr>
    <tr>
      <td><code>wgsl-webgpu</code></td>
      <td>Browser WebGPU Compute (Chrome, Safari)</td>
      <td>WGSL Text Shaders</td>
      <td>Web-accelerated Matrix Tensor Ops</td>
      <td><span class="tag-tier4">Tier 4</span></td>
    </tr>
  </tbody>
</table>

<!-- SECTION 6: CLI & CROSS-COMPILATION WORKFLOW -->
<div class="page-break"></div>
<h1>6. CLI Cross-Compilation Workflows</h1>

<h2>6.1 Target Specification Commands</h2>
<p>
Developers can cross-compile any Sorayunara project with a single CLI parameter:
</p>
<pre><code><span class="comment"># 1. Compile for Linux x86_64 server</span>
sora build --target x86_64-unknown-linux-gnu main.sora

<span class="comment"># 2. Compile for Apple Silicon ARM64 (macOS)</span>
sora build --target aarch64-apple-darwin main.sora

<span class="comment"># 3. Compile for Windows x86_64 binary</span>
sora build --target x86_64-pc-windows-msvc main.sora

<span class="comment"># 4. Compile WebAssembly for Browser or Cloud Edge</span>
sora build --target wasm32-wasi main.sora

<span class="comment"># 5. Compile for Embedded Microcontroller (STM32 / RP2040)</span>
sora build --target thumbv7em-none-eabihf --no-std main.sora

<span class="comment"># 6. Transpile to Standalone Portable ANSI C99</span>
sora build --target c99-generic-embedded main.sora
</code></pre>

<h2>6.2 Standard Library Feature Availability Matrix</h2>
<table>
  <thead>
    <tr>
      <th>Module Category</th>
      <th>Desktop / Server (Tier 1)</th>
      <th>Mobile (Tier 2)</th>
      <th>WebAssembly (Tier 1/2)</th>
      <th>Embedded / Bare-Metal (Tier 3)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Core Math & Logic (<code>std.math</code>, <code>std.types</code>)</strong></td>
      <td>✅ Full Support</td>
      <td>✅ Full Support</td>
      <td>✅ Full Support</td>
      <td>✅ Full Support (Zero-alloc)</td>
    </tr>
    <tr>
      <td><strong>Collections & Strings (<code>std.collections</code>)</strong></td>
      <td>✅ Full Support</td>
      <td>✅ Full Support</td>
      <td>✅ Full Support</td>
      <td>✅ Static / Bump Allocator</td>
    </tr>
    <tr>
      <td><strong>File System & I/O (<code>std.fs</code>, <code>std.io</code>)</strong></td>
      <td>✅ Native OS Filesystem</td>
      <td>✅ App Sandbox Storage</td>
      <td>✅ Virtual WASI Filesystem</td>
      <td>⚠️ Flash ROM / LittleFS</td>
    </tr>
    <tr>
      <td><strong>Networking & HTTP (<code>std.net</code>, <code>std.http</code>)</strong></td>
      <td>✅ TCP/UDP/TLS Sockets</td>
      <td>✅ Native Mobile Sockets</td>
      <td>✅ Fetch API / WebSocket</td>
      <td>⚠️ LwIP / WiFi HAL (ESP32)</td>
    </tr>
    <tr>
      <td><strong>Concurrency & Actors (<code>std.channel</code>)</strong></td>
      <td>✅ Multi-Threaded Threads</td>
      <td>✅ Multi-Threaded Coroutines</td>
      <td>✅ Event-Loop Coroutines</td>
      <td>⚠️ FreeRTOS Task Scheduling</td>
    </tr>
    <tr>
      <td><strong>GPU & SIMD (<code>std.simd</code>, <code>std.gpu</code>)</strong></td>
      <td>✅ AVX-512 / NEON / CUDA</td>
      <td>✅ ARM NEON / Metal</td>
      <td>✅ WebAssembly SIMD128</td>
      <td>❌ N/A</td>
    </tr>
  </tbody>
</table>

<h2>6.3 Summary of Compatibility</h2>
<p>
With this comprehensive 4-Tier Target Matrix, Sorayunara accomplishes universal execution versatility, allowing a single codebase written in <code>.sora</code> to seamlessly power high-throughput web backends, responsive mobile applications, interactive browser UIs, and mission-critical embedded IoT devices.
</p>

<div style="margin-top: 30px; text-align: center; font-size: 8.5pt; color: #888; border-top: 1px solid #e0e0e0; padding-top: 12px;">
  Sorayunara Universal Platform Target Matrix v2.0 • Sorayunara Core Team • https://github.com/Sorayunara/sorayunara
</div>

</body>
</html>
"""

html_path = os.path.abspath("docs/platform_matrix.html")
pdf_path = os.path.abspath("Sorayunara_Sora_Universal_Platform_Target_Matrix.pdf")

with open(html_path, "w", encoding="utf-8") as f:
    f.write(html_content)

print(f"Generated Platform Matrix HTML source at: {html_path}")

chrome_exe = r"C:\Program Files\Google\Chrome\Application\chrome.exe"
if not os.path.exists(chrome_exe):
    print("Chrome not found at standard path.")
    sys.exit(1)

cmd = [
    chrome_exe,
    "--headless",
    "--disable-gpu",
    "--no-pdf-header-footer",
    f"--print-to-pdf={pdf_path}",
    html_path
]

print(f"Executing: {' '.join(cmd)}")
res = subprocess.run(cmd, capture_output=True, text=True)
if res.returncode == 0 and os.path.exists(pdf_path):
    size_kb = os.path.getsize(pdf_path) / 1024
    print(f"SUCCESS: Generated PDF '{pdf_path}' ({size_kb:.1f} KB)")
else:
    print("Failed to generate PDF:", res.stderr)
    sys.exit(1)
