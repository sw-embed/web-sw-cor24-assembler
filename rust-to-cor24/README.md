# Rust to COR24 Compiler

Tooling to compile Rust to COR24 assembly via WASM as an intermediate format.

## Status: Planning

See documentation:
- [Possible Futures](docs/possible-futures.md) - Survey of all approaches
- [Planned Simplest](docs/planned-simplest.md) - Chosen WASM→COR24 approach

## Approach

```
Rust (.rs) → WASM (.wasm) → COR24 Assembly (.s) → Binary
            ↑               ↑
         rustc          wasm2cor24
        (standard)       (this project)
```

## Why WASM?

- Uses standard Rust toolchain (no rustc modifications)
- WASM is well-documented and stable
- Existing parsing tools (`wasmparser` crate)
- Stack-based IR simplifies translation

## Quick Start (Future)

```bash
# Compile Rust to WASM
cargo build --target wasm32-unknown-unknown --release

# Translate WASM to COR24 assembly
wasm2cor24 target/wasm32-unknown-unknown/release/myapp.wasm -o myapp.s

# Assemble (using cor24 toolchain)
cor24-as myapp.s -o myapp.bin
```

## Project Structure

```
rust-to-cor24/
├── README.md
├── Cargo.toml          # (to be created)
├── src/                # (to be created)
│   ├── main.rs         # CLI
│   └── lib.rs          # Library
├── runtime/            # (to be created)
│   └── cor24_runtime.s # Software mul/div
└── docs/
    ├── possible-futures.md
    └── planned-simplest.md
```

## Estimated Effort

~280 hours for MVP (basic integer ops, control flow, function calls)

## License

MIT
