//! COR24 Assembly Emulator — Web IDE
//!
//! Browser-based IDE for COR24 assembly programming with interactive
//! examples, challenges, and a live emulator.

pub mod challenge;

// Yew app (only for standalone wasm32 builds)
#[cfg(all(target_arch = "wasm32", feature = "standalone"))]
pub mod app;
#[cfg(all(target_arch = "wasm32", feature = "standalone"))]
pub mod c_examples;
#[cfg(all(target_arch = "wasm32", feature = "standalone"))]
pub mod rust_examples;

// WASM bindings (only for standalone wasm32 builds)
#[cfg(all(target_arch = "wasm32", feature = "standalone"))]
pub mod wasm;
