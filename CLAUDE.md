# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development server with hot reload (http://localhost:7401/cor24-rs/)
./serve.sh

# Production build (outputs to pages/)
trunk build --release

# Run tests
cargo test

# Check compilation without building
cargo check
```

Prerequisites: Rust 1.75+, Trunk (`cargo install trunk`), `rustup target add wasm32-unknown-unknown`.

## Deployment

The `pages/` directory contains pre-built production assets and is committed to git. GitHub Actions deploys from `pages/` on push to `main` — no CI build step, just upload. After `trunk build --release`, commit the updated `pages/` directory to deploy.

## Architecture

This is a browser-based COR24 CPU emulator written in Rust, compiled to WebAssembly via Trunk. The COR24 is a real 24-bit RISC architecture (C-Oriented RISC) designed for embedded systems education.

### Workspace Structure

- **`src/`** — Main application crate (`cor24-emulator`)
- **`components/`** — Reusable Yew UI components library
- **`rust-to-cor24/`** — Standalone CLI tool (not part of workspace) that translates WASM to COR24 assembly. Uses `wasmparser` crate. Not compiled to WASM — used offline to generate examples.

### Core Modules (src/)

- **`cpu/`** — CPU emulator core
  - `state.rs` — CPU state, memory (64KB subset of 24-bit address space), memory-mapped I/O (LED/switch at `0xFF0000`, UART at `0xFFFF00-02`)
  - `executor.rs` — Instruction execution engine
  - `decode_rom.rs` — Decode ROM extracted from actual FPGA Verilog hardware
  - `encode.rs` — Instruction encoding tables
  - `instruction.rs` — Opcode definitions, variable-length instructions (1/2/4 bytes)
- **`assembler.rs`** — Two-pass assembler producing machine code from COR24 assembly
- **`wasm.rs`** — `WasmCpu` wrapper exposing CPU to JavaScript/Yew via `wasm_bindgen`
- **`app.rs`** — Main Yew `#[function_component(App)]` — all application state and UI logic. This is the largest file; it manages two independent CPU instances (assembler tab and Rust pipeline tab)
- **`challenge.rs`** — Example programs and challenge definitions

### UI Components (components/)

Yew components: `Header`, `Sidebar`, `TabBar`, `ProgramArea`, `RegisterPanel`, `MemoryViewer`, `Modal`, `Collapsible`, `RustPipeline`. The `RustPipeline` component implements a wizard-driven 3-column view showing Rust→WASM→COR24 compilation stages.

### Key Patterns

- **Two CPU instances**: `app.rs` maintains separate `WasmCpu` state for the Assembler tab and Rust Pipeline tab
- **Animated run with stop**: Uses `Rc<Cell<bool>>` for stop flags and `Rc<Cell<u8>>` for switch state to ensure immediate visibility across async closures (Yew state updates are deferred)
- **Hardware-accurate I/O**: Matches COR24-TB test board — single LED (D2) and button (S2) using bit 0 of `IO_LEDSWDAT` (`0xFF0000`). Reference hardware docs are in `references/COR24-TB/`
- **Conditional compilation**: `app.rs` and `wasm.rs` are `#[cfg(target_arch = "wasm32")]` only; `cpu/`, `assembler`, and `challenge` modules compile on native targets for `cargo test`
- **`build.rs`**: Embeds git SHA, build timestamp, and hostname into the binary via env vars

### CSS

Two stylesheet files in `styles/`: `asm-game.css` (component styles) and `layout.css` (page structure). Referenced in `index.html` via Trunk's `data-trunk` attributes.

### Reference Materials

`references/COR24-TB/` contains the actual hardware documentation: Verilog source (including `cor24_io.v` for I/O address decoding), demo C programs (blinky, sieve, etc.), and FPGA project files. The decode ROM in `decode_rom.rs` was extracted from `cor24_cpu.v` using `scripts/extract_decode_rom.py`.
