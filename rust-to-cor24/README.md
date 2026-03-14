# Rust to COR24 Pipeline

Compiles Rust to COR24 machine code via the MSP430 target as a 16-bit intermediate.

## Pipeline Overview

```
                      ┌─────────┐
  demo_blinky.rs      │  rustc  │   Uses MSP430 as 16-bit backend
  (Rust source)  ───► │ nightly │   (no custom compiler needed)
                      │ msp430  │
                      └────┬────┘
                           │
                    ┌──────▼───────┐
                    │  .s file     │   MSP430 assembly text
                    │  (--emit asm)│   Functions in alphabetical section order
                    └──────┬───────┘
                           │
                 ┌─────────▼──────────┐
                 │  msp430-to-cor24   │   Translator (this project)
                 │  --entry demo_blinky│   Adds reset vector prologue
                 └─────────┬──────────┘
                           │
                    ┌──────▼───────┐
                    │  .cor24.s    │   COR24 assembly text
                    │              │   With `bra demo_blinky` at address 0
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  COR24       │   Two-pass assembler (in cor24-emulator crate)
                    │  Assembler   │   Resolves labels, encodes instructions
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  Binary blob │   Raw bytes, loaded at address 0x000000
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  COR24 CPU   │   PC starts at 0x000000 (RESET_ADDRESS)
                    │  Emulator    │   Executes `bra demo_blinky` first
                    └──────────────┘
```

## How the Entry Point Works

### The Problem

The Rust compiler (`rustc`) emits functions in `.section .text.<name>` sections,
ordered **alphabetically by function name** — not by source order. So a file with
`demo_blinky`, `delay`, and `mmio_write` produces MSP430 assembly with sections in
this order:

```
.section .text._RN...rust_begin_unwind   ← panic handler (first alphabetically!)
.section .text.delay
.section .text.demo_blinky               ← our entry point (buried in the middle)
.section .text.mmio_write
```

The COR24 CPU starts executing at address 0x000000. Without intervention, it would
execute the panic handler (an infinite loop), not our demo.

### The Solution: Reset Vector Prologue

The `msp430-to-cor24` translator emits an **absolute jump at address 0** that
jumps to the `start` function. It uses `la`+`jmp` instead of `bra` because
`bra` has a ±127 byte offset limit:

```asm
; Reset vector -> start
    la      r0, start            ; 4 bytes at address 0x000000
    jmp     (r0)                 ; 2 bytes at address 0x000004

; --- function: _RN...rust_begin_unwind ---   (panic handler)
_RN...rust_begin_unwind:
    ...                          ; writes "PANIC\n" to UART, then infinite loop

; --- function: demo_blinky ---
demo_blinky:
    la      r0, 0xFF0000
    lc      r1, 1
    ...

; --- function: start ---                    ; ← CPU jumps here from address 0
start:
    jal     r1, demo_blinky      ; call the demo function
```

This mirrors how real microcontrollers work: the hardware reset vector at address 0
contains a jump to the startup code.

### The `start` Convention

Every COR24 program compiled from Rust uses a fixed entry point convention:

```rust
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_blinky()       // calls the per-program named function
}
```

The `start()` function is the bridge between the CPU reset and the program logic.
Each program has exactly one `start()` that calls its main function.

### How the Entry Point is Identified

1. **Convention (default)**: The translator looks for a `start` label. If the input
   has `.globl` directives (i.e., comes from `rustc`) but no `start` label, it
   returns an error listing the available labels.

2. **Explicit `--entry <func>` flag** (optional override):
   ```bash
   msp430-to-cor24 input.msp430.s --entry my_entry -o output.cor24.s
   ```
   Only needed if a program can't use `start` for some reason.

3. **No `.globl` directives** (e.g., hand-written single-function tests):
   - No prologue emitted — first instruction is at address 0 (legacy behavior)

### In the Rust Source

```rust
#[no_mangle]
pub unsafe fn demo_blinky() -> ! {
    loop {
        mmio_write(LED_ADDR, 1);
        delay(5000);
        mmio_write(LED_ADDR, 0);
        delay(5000);
    }
}

#[no_mangle]
pub unsafe fn start() -> ! {
    demo_blinky()
}
```

The `#[no_mangle]` attribute makes both functions visible as `.globl` symbols in
the MSP430 assembly output. The translator finds `start` and emits the prologue.

## Register Mapping

| MSP430 | COR24 | Role |
|--------|-------|------|
| r12 | r0 | arg0 / return value |
| r13 | r1 | arg1 |
| r14 | r2 | arg2 |
| r1 | sp | stack pointer |
| r4-r11 | stack | spilled to fp-relative offsets |

## Calling Convention

The translator currently uses a **simplified calling convention** that differs from
the standard COR24 C compiler convention designed by Luther Johnson (COR24 architect).

| Aspect | Current Translator | Standard COR24 (Luther's) |
|--------|-------------------|---------------------------|
| **Function call** | `la r2, func; push r2; la r2, target; jmp (r2)` (4 insn, 10 bytes) | `la r0, func; jal r1,(r0)` (2 insn, 5 bytes) |
| **Return** | `pop r2; jmp (r2)` | `jmp (r1)` |
| **Arguments** | In registers (r0=arg0, r1=arg1, r2=arg2) | On the stack |
| **Return value** | In r0 | In r0 |
| **Prologue** | None (saves only what's needed) | `push fp; push r2; push r1; mov fp,sp` |
| **Epilogue** | None | `pop r1; pop r2; pop fp; jmp (r1)` |

The `jal` (jump-and-link) instruction is the COR24's dedicated call instruction. It
saves the return address in r1 automatically, eliminating the need to compute and push
a return label. The translator's current approach bypasses `jal` entirely.

A phased plan to adopt `jal` and the standard calling convention is documented in
[docs/research/20260314-cor24-plan.md](../docs/research/20260314-cor24-plan.md).

## I/O Address Mapping

MSP430 uses 16-bit addresses. COR24 uses 24-bit. The translator maps:

| MSP430 (16-bit) | COR24 (24-bit) | Device |
|------------------|-----------------|--------|
| 0xFF00 | 0xFF0000 | LED D2 / Button S2 |
| 0xFF01 | 0xFF0100 | UART data register |
| 0xFF02 | 0xFF0101 | UART status register |

In Rust source, we define 16-bit constants that the MSP430 target can handle:
```rust
const LED_ADDR: u16 = 0xFF00;     // → 0xFF0000 after translation
const UART_DATA: u16 = 0xFF01;    // → 0xFF0100 after translation
```

## CLI Usage

```bash
# Translate MSP430 .s to COR24 assembly (uses start convention by default)
msp430-to-cor24 input.msp430.s -o output.cor24.s

# Override entry point (rare — only if start can't be used)
msp430-to-cor24 input.msp430.s --entry my_entry -o output.cor24.s

# Compile a Rust project end-to-end
msp430-to-cor24 --compile path/to/rust/project

# Run built-in test case
msp430-to-cor24 --test
```

### Compile mode prerequisites

```bash
rustup toolchain install nightly
rustup target add msp430-none-elf --toolchain nightly
```

## File Types in the Pipeline

| Extension | Format | Description |
|-----------|--------|-------------|
| `.rs` | Rust source | `#![no_std]`, `#[panic_handler]`, `#[no_mangle]` on entry |
| `.msp430.s` | MSP430 asm text | Output of `rustc --emit asm`, `.section .text.<func>` per function |
| `.cor24.s` | COR24 asm text | `la+jmp start` prologue + translated instructions + labels |
| (in-memory) | `Vec<u8>` | Raw bytes from COR24 assembler, loaded at address 0 |

## Project Structure

```
rust-to-cor24/
├── src/
│   ├── lib.rs           # Re-exports translate_msp430
│   ├── msp430.rs        # MSP430 parser + COR24 translator + entry point detection
│   ├── msp430_cli.rs    # CLI: --entry, -o, --compile, --test
│   ├── run.rs           # COR24 emulator runner (assembles + executes)
│   └── pipeline.rs      # WASM pipeline (legacy, unused)
├── output/              # Pre-generated demo outputs
│   ├── demo_blinky.msp430.s   # MSP430 asm from rustc
│   ├── demo_blinky.cor24.s    # COR24 asm with bra prologue
│   └── demo_blinky.log        # Emulator run output
└── data/
    └── pipeline/
        └── demos.rs     # Complete Rust source for all demos
```

## Conventions

1. **`start` entry point**: Every program has `#[no_mangle] pub unsafe fn start()` that calls the main function
2. **`#[no_mangle]`**: Required on `start`, entry functions, and all functions called across translation units
3. **`#[inline(never)]`**: Required on helper functions to prevent inlining (which would eliminate the callable function)
4. **`#![no_std]` + `#[panic_handler]`**: Required — no standard library on bare metal
5. **Panic handler**: Writes "PANIC\n" to UART before entering infinite loop
6. **Reset vector**: `la r0, start` + `jmp (r0)` at address 0 — the COR24 equivalent of a hardware reset vector
7. **Halt convention**: `loop {}` in Rust compiles to a self-branch (`bra .`), detected by the emulator as halt
