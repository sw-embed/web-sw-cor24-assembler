# Rust → COR24 Pipeline Demos

Rust programs compiled through the full cross-compilation pipeline:

```
Rust (.rs) → rustc --target msp430-none-elf → MSP430 ASM → msp430-to-cor24 → COR24 ASM → assembler → emulator
```

These demonstrate the Rust-to-COR24 translator. Each demo is a standalone
`#![no_std]` Rust crate with a `#[no_mangle] pub unsafe fn start()` entry point.

The demos are available in the **Rust tab** of the
[web emulator](https://sw-embed.github.io/cor24-rs/) (click "Rust" in the
header, then pick an example from the dropdown) and can also be run from
the command line.

## Web UI

1. Open the [web emulator](https://sw-embed.github.io/cor24-rs/)
2. Click the **Rust** tab in the header
3. Select an example from the dropdown (default: Blink LED)
4. Click **Compile** → **Translate** → **Assemble** to step through the pipeline
5. Click **Run** to execute in the emulator

## CLI

### Single demo

```bash
cd rust-to-cor24/demos

# Full pipeline: compile Rust → translate → assemble → run
./run-demo.sh demo_add

# Skip Rust compilation (use pre-built .msp430.s)
./run-demo.sh demo_add --skip-compile

# With UART input (for echo demos)
./run-demo.sh demo_echo_v2 --uart-input 'hello!'

# List available demos
./run-demo.sh --help
```

Each demo also has its own `run.sh`:

```bash
cd rust-to-cor24/demos/demo_add
./run.sh                    # runs the full pipeline
./run.sh --skip-compile     # skip Rust compilation step
```

Echo demos (`demo_echo`, `demo_echo_v2`) default to `--uart-input 'abc3\x21'`
(`\x21` = `!` which halts the program) if no input is specified.

### All demos

```bash
cd rust-to-cor24/demos
./generate-all.sh    # compiles, translates, assembles, and runs all 12 demos
```

### Prerequisites

```bash
rustup toolchain install nightly
rustup target add msp430-none-elf --toolchain nightly
cd rust-to-cor24 && cargo build --release    # builds msp430-to-cor24 and cor24-run
```

## Demo Catalog

### Arithmetic & Logic

| Demo | Description |
|------|-------------|
| **demo_add** | Computes `100 + 200 + 42 = 342`, stores result to memory. Note: `rustc` constant-folds the addition at compile time — the generated code just loads 342 directly. |
| **demo_stack_vars** | Accumulates values across many variables, forcing register spills to fp-relative stack slots. XOR result stored to memory at 0x0100. Demonstrates the translator's spill mechanism. |

### Control Flow

| Demo | Description |
|------|-------------|
| **demo_countdown** | Counts down from 10 to 0, storing each value to memory at 0x0100. Loop with delay. |
| **demo_fibonacci** | Recursive `fib(10) = 89`. Deep stack frames, result stored to memory. |
| **demo_fibonacci_iter** | Iterative `fib(10) = 89` using a simple loop. Fits in 3 registers, result stored to memory. |
| **demo_nested** | Call chain: `start → demo_nested → level_a → level_b → level_c`. Result stored to memory at 0x0100. All intermediate stack frames are visible in the memory dump at halt. |

### UART I/O

| Demo | Description |
|------|-------------|
| **demo_uart_hello** | Sends `"Hello\n"` character-by-character to the UART data register. |
| **demo_echo** | Interrupt-driven UART echo with ISR in pure COR24 inline assembly. Letters → uppercase, `'!'` → halt, others echoed as-is. |
| **demo_echo_v2** | Same behavior as `demo_echo`, but application logic (`to_upper`, `handle_rx`) is in Rust. Only ISR register save/restore and interrupt setup use `asm!()`. |

### Hardware I/O

| Demo | Description |
|------|-------------|
| **demo_blinky** | Toggles LED D2 on/off in a loop with delay pauses. |
| **demo_button_echo** | Reads button S2 state and echoes it to LED D2 in a tight loop. |

### Ownership & RAII

| Demo | Description |
|------|-------------|
| **demo_drop** | Demonstrates `Drop` trait on a stack-allocated `Guard` struct. The compiler inserts the destructor call at scope exit — no allocator needed. Memory at 0x0100 goes 1 (alive) → 0 (dropped) → 0xFF (done). |

### Error Handling

| Demo | Description |
|------|-------------|
| **demo_panic** | Triggers Rust's panic handler, which writes `"PANIC\n"` to UART and enters an infinite loop. |

## Pipeline Output

Each demo produces three artifacts:

- **`<demo>.msp430.s`** — MSP430 assembly from `rustc`
- **`<demo>.cor24.s`** — COR24 assembly (with reset vector prologue)
- **`<demo>.log`** — Emulator output (registers, memory, UART at halt)

The `run-demo.sh` script displays all three pipeline stages interactively:
source code, MSP430 assembly (key functions), COR24 assembly, and emulator
output with register/memory dump.

## COR24 I/O Addresses

All demos use byte-width MMIO (`sb`/`lb`) matching the hardware Verilog:

| Address | Register | Description |
|---------|----------|-------------|
| `0xFF0000` | `IO_LEDSWDAT` | LED D2 (write bit 0) / Button S2 (read bit 0) |
| `0xFF0100` | `IO_UARTDATA` | UART data (read: RX byte, write: TX byte) |
| `0xFF0101` | `IO_UARTSTATUS` | UART status (bit 0: RX ready) |
| `0xFF0110` | `IO_UARTINTENA` | UART interrupt enable (bit 0: RX interrupt) |

## Calling Convention Note

The translator currently generates `push`/`jmp` sequences for function calls rather
than using the COR24's `jal` (jump-and-link) instruction, which is the hardware's
intended calling mechanism. A future update will switch to `jal` per the COR24
architect's recommendation, saving ~4 bytes per call site. See
[docs/research/20260314-cor24-plan.md](research/20260314-cor24-plan.md) for details.

## Source Files

Rust sources are in `rust-to-cor24/demos/demo_*/src/lib.rs`. Pre-built pipeline
examples for the web UI are in `src/examples/rust_pipeline/`.

## See Also

- [Assembler Examples](assembler-examples.md) — Hand-written COR24 assembly programs
- [rust-to-cor24/README.md](../rust-to-cor24/README.md) — Translator architecture and register mapping
- [Live Web Emulator](https://sw-embed.github.io/cor24-rs/) — Browser-based emulator with Rust pipeline tab
