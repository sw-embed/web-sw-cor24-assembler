# Future Enhancement: Rust Compiler Backend for COR24

This document outlines what would be required to create tooling for a Rust compiler targeting the COR24 architecture for `no_std` embedded targets.

## Overview

Creating a Rust compiler backend for a new architecture is a significant undertaking. There are several possible approaches, each with different trade-offs in terms of effort, capability, and maintainability.

## Approaches

### Option 1: LLVM Backend (Most Robust)

The most complete solution involves adding COR24 as an LLVM target, which Rust uses for code generation.

#### Components Required

1. **TableGen Target Description** (~5-10K lines)
   - Register definitions (8 x 24-bit registers)
   - Instruction patterns and encodings
   - Calling convention specification
   - Stack frame layout rules

2. **Target Machine Implementation** (~10-20K lines C++)
   ```
   llvm/lib/Target/COR24/
   ├── COR24TargetMachine.cpp      # Main target machine
   ├── COR24ISelLowering.cpp       # IR to SelectionDAG lowering
   ├── COR24ISelDAGToDAG.cpp       # SelectionDAG to MachineDAG
   ├── COR24InstrInfo.cpp          # Instruction information
   ├── COR24InstrInfo.td           # Instruction definitions (TableGen)
   ├── COR24RegisterInfo.cpp       # Register information
   ├── COR24RegisterInfo.td        # Register definitions (TableGen)
   ├── COR24FrameLowering.cpp      # Stack frame handling
   ├── COR24AsmPrinter.cpp         # Assembly output
   ├── COR24MCTargetDesc.cpp       # MC layer description
   └── COR24Subtarget.cpp          # Target features/variants
   ```

3. **Rust Target Specification** (JSON)
   ```json
   {
     "llvm-target": "cor24-unknown-none",
     "data-layout": "e-m:e-p:24:32-i8:8-i16:16-i24:32-i32:32",
     "arch": "cor24",
     "target-endian": "little",
     "target-pointer-width": "24",
     "target-c-int-width": "24",
     "os": "none",
     "executables": true,
     "linker-flavor": "ld.lld",
     "linker": "rust-lld",
     "panic-strategy": "abort",
     "disable-redzone": true,
     "features": ""
   }
   ```

4. **libcore Port**
   - Adapt core library for 24-bit pointers
   - Implement intrinsics for COR24
   - Software multiply/divide if not in hardware

#### Pros
- Full optimization pipeline
- Complete Rust feature support
- Industry-standard tooling
- Good debugging support

#### Cons
- Massive implementation effort
- Must track LLVM upstream changes
- C++ codebase (different from Rust ecosystem)

#### Effort Estimate
- **Time**: 6-12 months for experienced LLVM developer
- **Hours**: 2000-5000 hours
- **Team**: Ideally 2-3 people with LLVM experience

---

### Option 2: Cranelift Backend (Rust-Native)

Cranelift is a code generator written in Rust, used by Wasmtime and experimentally by rustc.

#### Components Required

1. **ISA Definition** (~2-3K lines Rust)
   ```rust
   // cranelift/codegen/src/isa/cor24/mod.rs
   pub struct Cor24Backend {
       // Target configuration
   }

   impl TargetIsa for Cor24Backend {
       fn compile_function(...) -> CodegenResult<...>;
       fn emit_binary(...) -> Result<MachBuffer>;
   }
   ```

2. **Instruction Definitions** using Cranelift's DSL
   ```rust
   // Define COR24 registers
   define_registers! {
       R0, R1, R2, FP, SP, Z, IV, IR
   }

   // Define instruction encodings
   define_instructions! {
       lc_imm: (ra: Reg, imm: Imm8) => [0x44 + ra.index(), imm],
       add_reg: (ra: Reg, rb: Reg) => [(ra.index() << 3) | rb.index()],
       // ...
   }
   ```

3. **Legalization Rules**
   - Convert Cranelift IR operations to COR24 instructions
   - Handle operations not directly supported (e.g., 32-bit ops on 24-bit)

4. **ABI Implementation**
   - Calling convention
   - Stack frame layout
   - Argument passing

5. **rustc Integration**
   ```bash
   RUSTFLAGS="-Zcodegen-backend=path/to/cor24_cranelift.so" cargo build
   ```

#### Pros
- Written in Rust
- Simpler than LLVM
- Easier to iterate and prototype
- Growing rustc support

#### Cons
- Less mature than LLVM
- Fewer optimizations
- Smaller community

#### Effort Estimate
- **Time**: 3-6 months
- **Hours**: 1000-2000 hours
- **Team**: 1-2 Rust developers with compiler experience

---

### Option 3: Source-to-Source via C

Since COR24 has an existing C compiler, leverage it through transpilation.

#### Pipeline
```
Rust Source
    ↓ (mrustc)
C Source
    ↓ (COR24 C compiler)
COR24 Assembly
    ↓ (assembler)
Binary
```

#### Components Required

1. **mrustc Setup** - Configure for COR24 C output
2. **C Runtime** - Minimal runtime for Rust semantics
3. **Build System Integration** - Cargo wrapper

#### Pros
- Leverages existing C toolchain
- Relatively quick to set up
- Proven path (mrustc works)

#### Cons
- Limited Rust feature support
- No async, limited generics
- C compilation adds complexity
- Debugging is harder

#### Effort Estimate
- **Time**: 1-2 months
- **Hours**: 200-500 hours
- **Team**: 1 developer

---

### Option 4: Embedded Rust Pattern (ARM-style ecosystem)

This is how Rust works with Arduino-supported ARM boards (Cortex-M). Could a similar pattern work for COR24?

#### How ARM Embedded Rust Works

```
┌─────────────────────────────────────────────────────────┐
│  Application Code (your #![no_std] Rust)                │
├─────────────────────────────────────────────────────────┤
│  HAL Crate (embedded-hal traits)                        │
│  e.g., stm32f4xx-hal, nrf52-hal, arduino-hal            │
├─────────────────────────────────────────────────────────┤
│  PAC Crate (Peripheral Access, generated from SVD)      │
│  e.g., stm32f4, nrf52840-pac                           │
├─────────────────────────────────────────────────────────┤
│  cortex-m-rt (Runtime: startup, interrupts, panic)      │
├─────────────────────────────────────────────────────────┤
│  LLVM ARM Backend (thumbv7em-none-eabihf, etc.)         │
└─────────────────────────────────────────────────────────┘
```

**Key Components:**
1. **LLVM Backend** - Already exists for ARM
2. **Target Spec** - `thumbv6m-none-eabi`, `thumbv7m-none-eabi`, etc.
3. **PAC** - Generated from SVD (System View Description) files
4. **HAL** - Implements `embedded-hal` traits
5. **Runtime** - `cortex-m-rt` handles startup code

#### Adapting for COR24

Since LLVM doesn't have a COR24 backend, this exact pattern won't work. However, we could:

**Option 4a: Virtual Machine / Interpreter Approach**
```
Rust Code
    ↓ (compile to custom bytecode)
COR24 Bytecode
    ↓ (runtime interpreter on COR24)
Execution
```

Create a simple bytecode that a COR24 interpreter can run. The interpreter is written in COR24 assembly, Rust compiles to the bytecode.

**Option 4b: Macro-based DSL**
```rust
#![no_std]
use cor24_dsl::*;

cor24_asm! {
    fn add_numbers() {
        lc r0, 10;
        lc r1, 20;
        add r0, r1;
        halt;
    }
}
```

Rust macros generate COR24 assembly at compile time. Not "real" Rust, but Rust-like syntax with full IDE support.

**Option 4c: Compile-time Code Generation**
```rust
// Build script generates COR24 assembly from Rust-like DSL
// Uses proc-macros to analyze code structure

#[cor24::function]
fn blinky(port: &mut GPIO) {
    loop {
        port.toggle();
        delay(1000);
    }
}
```

The `#[cor24::function]` proc-macro generates COR24 assembly during compilation.

#### COR24 Ecosystem Equivalent

If we were to build the full embedded ecosystem:

```
┌─────────────────────────────────────────────────────────┐
│  Application Code                                       │
├─────────────────────────────────────────────────────────┤
│  cor24-hal (GPIO, UART, Timer traits)                   │
├─────────────────────────────────────────────────────────┤
│  cor24-pac (Memory-mapped peripheral definitions)       │
├─────────────────────────────────────────────────────────┤
│  cor24-rt (Runtime: _start, interrupt vectors, panic)   │
├─────────────────────────────────────────────────────────┤
│  ??? Backend (Cranelift/LLVM/Custom)                    │
└─────────────────────────────────────────────────────────┘
```

**What we'd need to create:**

1. **cor24-pac** - Memory map definitions
   ```rust
   #[repr(C)]
   pub struct UART {
       pub data: VolatileCell<u8>,
       pub status: VolatileCell<u8>,
       pub control: VolatileCell<u8>,
   }

   pub const UART0: *mut UART = 0xFF00 as *mut UART;
   ```

2. **cor24-hal** - Hardware abstraction
   ```rust
   impl embedded_hal::serial::Write<u8> for Uart {
       fn write(&mut self, byte: u8) -> Result<(), Error> {
           while !self.is_ready() {}
           unsafe { (*UART0).data.set(byte); }
           Ok(())
       }
   }
   ```

3. **cor24-rt** - Runtime
   ```rust
   #[no_mangle]
   pub unsafe extern "C" fn _start() -> ! {
       // Initialize .data, .bss
       // Set up stack
       // Call main
       extern "Rust" { fn main() -> !; }
       main()
   }
   ```

#### Effort for Ecosystem Approach

If a backend existed, building the ecosystem is relatively easy:
- **PAC**: 1-2 weeks (if peripheral docs exist)
- **HAL**: 2-4 weeks for basic peripherals
- **Runtime**: 1 week

The bottleneck is always the compiler backend.

---

### Option 5: Custom MIR-to-Assembly (Minimal Viable)

For a minimal `no_std` embedded target, directly translate Rust's MIR.

#### Components Required

1. **MIR Consumer**
   ```rust
   // Read MIR from rustc
   fn process_mir(mir: &Body<'_>) -> Vec<Cor24Instruction> {
       for bb in mir.basic_blocks() {
           for stmt in &bb.statements {
               // Translate each MIR statement
           }
       }
   }
   ```

2. **Direct Code Generator**
   - Map MIR operations to COR24 assembly
   - Simple register allocator
   - Stack slot management

3. **Linker Script**
   - Memory layout for COR24
   - Section placement

#### Limitations
- No monomorphization (must be pre-expanded)
- Limited optimizations
- Basic control flow only
- No complex types

#### Pros
- Fastest path to working code
- Full control over output
- Good learning project

#### Cons
- Very limited Rust support
- Maintenance burden
- Not production-ready

#### Effort Estimate
- **Time**: 3-6 months for basic subset
- **Hours**: 500-1000 hours
- **Team**: 1 developer with rustc internals knowledge

---

## COR24-Specific Challenges

### 1. 24-bit Pointer Width

Rust assumes standard pointer widths (8/16/32/64 bits). 24-bit requires:

- Custom `usize`/`isize` handling
- Memory layout calculations adjusted
- Potential alignment issues
- May need to treat as 32-bit internally and mask

### 2. Limited Register Set

Only 3 general-purpose registers with 5 special-purpose:
- `r0-r2`: General purpose
- `fp (r3)`: Frame pointer
- `sp (r4)`: Stack pointer
- `z (r5)`: Zero (for compare instructions)
- `iv (r6)`: Interrupt vector
- `ir (r7)`: Interrupt return

This means:
- Heavy register spilling to stack
- Careful calling convention design
- May need custom register allocator hints

### 3. Variable-Length Instructions

COR24 has 1, 2, or 4 byte instructions:
- Complicates code size estimation
- Branch offset calculations are tricky
- May need multiple passes for correct offsets

### 4. Missing Hardware Operations

Likely no hardware:
- Multiply (need software `__mulsi3`)
- Divide (need software `__divsi3`, `__modsi3`)
- Floating point (software or skip entirely)

### 5. Address Space Considerations

16MB addressable memory (24-bit):
- Stack size limits
- No large allocations
- Memory-mapped I/O considerations

---

## Verification Strategy

### Level 1: Unit Tests
- Test each instruction pattern generation
- Verify encodings match specification
- Test register allocation scenarios

### Level 2: Execution Tests
- Run generated code on COR24 emulator (this project!)
- Compare results with expected values
- Test edge cases (overflow, underflow, etc.)

### Level 3: Compiler Test Suite
- Adapt subset of `rustc` test suite
- Focus on `no_std` compatible tests
- Run standard algorithms (sorting, searching)

### Level 4: Real Programs
- Blinky LED program
- UART echo
- Simple state machine
- Interrupt handlers

### Testing Infrastructure

```rust
// Example test harness
#[test]
fn test_addition() {
    let source = r#"
        #![no_std]
        #![no_main]

        #[no_mangle]
        pub fn add(a: u24, b: u24) -> u24 {
            a + b
        }
    "#;

    let binary = compile_cor24(source);
    let mut emu = Cor24Emulator::new();
    emu.load(binary);
    emu.set_reg(0, 5);  // a = 5
    emu.set_reg(1, 3);  // b = 3
    emu.run();
    assert_eq!(emu.get_reg(0), 8);  // result = 8
}
```

---

## Recommended Path for COR24

Given COR24's educational nature and this emulator's existence:

### Phase 1: Prototype (2-3 months)
1. Choose Cranelift backend approach
2. Implement minimal instruction set
3. Target `no_std`, `#![no_main]` only
4. Test with this emulator

### Phase 2: Core Support (2-3 months)
1. Full integer arithmetic
2. Control flow (branches, loops)
3. Function calls with proper ABI
4. Stack allocation

### Phase 3: Usability (2-3 months)
1. Cargo integration
2. Basic `core` library support
3. Inline assembly (`asm!` macro)
4. Debugging symbols

### Minimal Viable Feature Set
- Integer types: `u8`, `i8`, `u16`, `i16`, `u24`, `i24`
- No floating point
- No heap allocation
- No threading
- Basic structs and enums
- Simple generics (monomorphized)
- `panic = "abort"`

### Realistic Effort for MVP
- **Time**: 6-9 months part-time
- **Hours**: 500-1000 hours
- **Prerequisites**:
  - Familiarity with Rust internals
  - Understanding of compiler design
  - Knowledge of COR24 architecture

---

## Resources

### LLVM Backend Development
- [LLVM Backend Tutorial](https://llvm.org/docs/WritingAnLLVMBackend.html)
- [Creating an LLVM Backend for the Cpu0 Architecture](https://jonathan2251.github.io/lbd/)

### Cranelift
- [Cranelift Documentation](https://cranelift.dev/)
- [Cranelift ISA DSL](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)

### Rust Compiler
- [rustc Dev Guide](https://rustc-dev-guide.rust-lang.org/)
- [MIR Documentation](https://rustc-dev-guide.rust-lang.org/mir/index.html)

### Similar Projects
- [AVR-Rust](https://github.com/avr-rust) - Rust for 8-bit AVR
- [MSP430-Rust](https://github.com/rust-embedded/msp430) - Rust for 16-bit MSP430
- [xtensa-rust](https://github.com/esp-rs) - Rust for Xtensa (ESP32)

---

## Conclusion

Creating a Rust backend for COR24 is achievable but requires significant effort. The Cranelift approach offers the best balance of effort vs. capability for an educational/hobbyist project. The existence of this emulator makes testing and iteration much faster than working with real hardware.

For a serious effort, budget 500-1000 hours and expect a working `no_std` subset within 6-9 months of part-time work.
