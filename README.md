# web-sw-cor24-assembler

Browser-based IDE for the COR24 (C-Oriented RISC, 24-bit) architecture.
Written in Rust, compiled to WebAssembly via [Trunk](https://trunkrs.dev/).

## Dependencies

- [sw-cor24-emulator](https://github.com/sw-embed/sw-cor24-emulator) — CPU emulator and ISA
- [sw-cor24-x-assembler](https://github.com/sw-embed/sw-cor24-x-assembler) — assembler library

## Build

```bash
# Dev server with hot reload
./scripts/serve.sh

# Production build (outputs to pages/)
./scripts/build-pages.sh
```

Prerequisites: Rust 1.85+, Trunk (`cargo install trunk`), `rustup target add wasm32-unknown-unknown`.
