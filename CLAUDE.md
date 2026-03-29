# web-sw-cor24-assembler — Claude Instructions

## Project Overview

Browser-based COR24 assembly IDE. Yew frontend compiled to WASM via Trunk.
Depends on sibling repos for emulator and assembler logic.

## Build

```bash
./scripts/serve.sh          # dev server with hot reload
./scripts/build-pages.sh    # production build to pages/
cargo check                 # native check (non-wasm modules only)
cargo check --target wasm32-unknown-unknown  # full wasm check
```

## File Structure

- `src/app.rs` — Main Yew App component (all UI state and logic)
- `src/wasm.rs` — WasmCpu wrapper exposing emulator to JS/Yew
- `src/challenge.rs` — Challenge definitions and self-test system
- `src/c_examples.rs` — C pipeline example data
- `src/rust_examples.rs` — Rust pipeline example data
- `src/examples/` — Assembly, C, and Rust example source files (included via `include_str!`)
- `components/` — Reusable Yew UI components library
- `styles/` — CSS stylesheets
- `index.html` — Trunk entry point

## Dependencies

- `cor24-emulator` (path: `../sw-cor24-emulator`) — CPU, ISA, assembler, emulator core
- `cor24-assembler` (path: `../sw-cor24-assembler`) — assembler library
- `components` (local) — Yew UI components

## Commit Discipline

Commit early and often. Each commit should do one thing.
Run `cargo check --target wasm32-unknown-unknown` before committing.
