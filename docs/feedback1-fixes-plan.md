# Feedback1 Fixes Plan

## Issues to Address

Based on `docs/feedback1.txt`, there are 4 architectural inaccuracies that need correction:

### Issue 1: Address Space Size
**Problem**: Documentation incorrectly states "64KB" address space
**Correction**: Address space is 24 bits = 16 MB (16,777,216 bytes)

### Issue 2: General-Purpose Register Count
**Problem**: Documentation states "8 general-purpose registers"
**Correction**: Only 3 registers (r0, r1, r2) are truly general-purpose. Registers r3-r7 have specific purposes:
- r3 = fp (frame pointer)
- r4 = sp (stack pointer)
- r5 = z (zero, for compare instructions only)
- r6 = iv (interrupt vector)
- r7 = ir (interrupt return)

### Issue 3: Z Register Description
**Problem**: r5/z described as "zero/condition" register
**Correction**: The z register is simply a zero value accessible only in compare instructions. The condition flag C is separate.

### Issue 4: Instruction Lengths
**Problem**: Documentation says "1-4 bytes" suggesting all lengths possible
**Correction**: Instructions are 1, 2, or 4 bytes only. Never 3 bytes.
**Note**: Words/data ARE 3 bytes (24-bit), but instruction encoding never uses 3 bytes.

---

## Files to Fix

### Issue 1: Address Space (64KB → 16MB)

| File | Line | Current | Fix |
|------|------|---------|-----|
| `README.md` | 33 | `64KB Address Space` | `16MB Address Space (24-bit)` |
| `src/app.rs` | 976 | `64KB Memory` | `16MB Memory (24-bit addressable)` |
| `src/lib.rs` | - | - | Update any 64KB references |
| `src/cpu/state.rs` | 5-6 | `MEMORY_SIZE: usize = 65536` | Keep for emulation but update comment |
| `src/cpu/mod.rs` | - | - | Update any 64KB references |
| `work/description.txt` | 75 | `64KB byte-addressable memory` | `16MB byte-addressable memory (24-bit)` |
| `docs/isa-reference.md` | - | Check for 64KB references | Update if found |
| `docs/future-rust-backend.md` | 421 | `64KB addressable memory` | `16MB addressable memory` |
| `rust-to-cor24/src/pipeline.rs` | 34 | `vec![0; 65536]` | Document as emulation subset |
| `rust-to-cor24/src/run.rs` | 44 | `vec![0; 65536]` | Document as emulation subset |
| `rust-to-cor24/docs/planned-simplest.md` | 131 | `64KB` | Update to 16MB |
| `rust-to-cor24/docs/possible-futures.md` | 200 | `64KB memory` | Update to 16MB |
| `docs/issues.md` | 73 | `full 64KB` | Update to 16MB |

### Issue 2: GP Register Count (8 → 3)

| File | Line | Current | Fix |
|------|------|---------|-----|
| `README.md` | 26 | `8 General-Purpose Registers` | `3 General-Purpose Registers (r0-r2)` + add special registers section |
| `src/app.rs` | 975 | `8 Registers (24-bit)` | `3 GP Registers + 5 Special Registers` |
| `src/lib.rs` | 7 | `8 general-purpose 24-bit registers` | `3 general-purpose registers (r0-r2)` |
| `src/cpu/mod.rs` | 4 | `8 general-purpose 24-bit registers` | `3 general-purpose registers (r0-r2)` |
| `work/description.txt` | 68 | `8 general-purpose registers` | `3 general-purpose registers + 5 special` |
| `work/slides/99-resources.svg` | 94 | `8 registers` | `3 GP + 5 special registers` |
| `docs/isa-reference.md` | 10 | `8 general-purpose registers` | Update with accurate description |
| `docs/future-rust-backend.md` | 392 | `Only 8 registers` | `Only 3 GP registers` |
| `rust-to-cor24/docs/possible-futures.md` | 199 | `8 registers` | `3 GP registers` |
| `docs/issues.md` | 78 | `all 8 registers` | `all registers` |

### Issue 3: Z Register Description (remove "zero/condition")

| File | Line | Current | Fix |
|------|------|---------|-----|
| `README.md` | 29 | `r5 = z (zero/condition)` | `r5 = z (zero for comparisons)` |
| `src/app.rs` | 541, 985 | `Zero Register (r5)` | Clarify compare-only access |
| `src/assembler.rs` | 320 | `z register, also condition flag` | `z register (zero for comparisons)` |
| `work/description.txt` | 71 | `z (zero register)` | `z (zero for compare instructions)` |
| `docs/isa-reference.md` | 25 | `Zero register (for comparisons)` | Already correct, verify |
| `docs/future-rust-backend.md` | 396 | `z (r5): Zero register` | Add "for comparisons only" |

### Issue 4: Instruction Lengths (1-4 → 1, 2, or 4)

| File | Line | Current | Fix |
|------|------|---------|-----|
| `README.md` | 34 | `1-4 bytes` | `1, 2, or 4 bytes` |
| `src/app.rs` | 978 | `1-4 bytes` | `1, 2, or 4 bytes` |
| `src/app.rs` | 1046 | `Load word (3 bytes)` | This is DATA size, not instruction - OK |
| `src/lib.rs` | 10 | `1-4 bytes` | `1, 2, or 4 bytes` |
| `src/cpu/mod.rs` | 7 | `1-4 bytes` | `1, 2, or 4 bytes` |
| `work/description.txt` | 76 | `1-4 bytes` | `1, 2, or 4 bytes` |
| `docs/isa-reference.md` | 12 | `1 to 4 bytes` | `1, 2, or 4 bytes` |
| `docs/isa-reference.md` | 228-229 | `Words are 24 bits (3 bytes)` | Keep - this is data, not instructions |

**Note**: References to "3 bytes" for word/data size are CORRECT. Only instruction length claims need fixing.

---

## Search Patterns for Verification

After fixes, run these searches to verify no instances remain:

```bash
# Issue 1: Should find no 64KB references except emulation comments
grep -rn "64.KB\|64KB\|65536" --include="*.rs" --include="*.md" --include="*.txt"

# Issue 2: Should find no "8 general-purpose" or "8 GP"
grep -rn "8 [Gg]eneral\|8 GP" --include="*.rs" --include="*.md" --include="*.txt"

# Issue 3: Should find no "zero/condition" or "zero.condition"
grep -rn "zero.condition\|zero/condition" --include="*.rs" --include="*.md"

# Issue 4: Should find no "1-4 bytes" for instructions
grep -rn "1-4 bytes\|1.4 bytes" --include="*.rs" --include="*.md" --include="*.txt"
```

---

## Implementation Steps

### Phase 1: Documentation (README, docs/)
1. Update `README.md` with all 4 fixes
2. Update `docs/isa-reference.md`
3. Update `docs/issues.md`
4. Update `docs/future-rust-backend.md`

### Phase 2: Source Code Comments (src/)
1. Update `src/lib.rs` doc comments
2. Update `src/cpu/mod.rs` doc comments
3. Update `src/cpu/state.rs` comments (keep 65536 for emulation, update comment)
4. Update `src/assembler.rs` comment
5. Update `src/app.rs` UI strings

### Phase 3: Work/Planning Documents
1. Update `work/description.txt`
2. Update `work/slides/99-resources.svg` (if editable)

### Phase 4: Rust-to-COR24 Subproject
1. Update `rust-to-cor24/src/pipeline.rs` comments
2. Update `rust-to-cor24/src/run.rs` comments
3. Update `rust-to-cor24/docs/planned-simplest.md`
4. Update `rust-to-cor24/docs/possible-futures.md`

### Phase 5: Rebuild and Verify
1. Run `cargo build` to ensure no code breakage
2. Run `trunk build --release` to rebuild WebUI
3. Run verification grep commands
4. Commit changes with descriptive message

---

## Completion Criteria

The fixes are complete when:

1. **All grep searches return no incorrect instances** (except intentional emulation limits)
2. **`cargo build` succeeds** with no errors
3. **`cargo test` passes** all tests
4. **WebUI displays correct information** in:
   - Welcome/help dialog
   - ISA reference panel
   - Register display labels
5. **README.md accurately reflects** the COR24 architecture
6. **No documentation claims** 64KB, 8 GP registers, zero/condition, or 1-4 byte instructions

---

## Notes

- The emulator uses 65536 bytes (64KB) for practical emulation purposes - this is intentional and should be documented as "emulation subset of full 16MB address space"
- The condition flag C is separate from the z register - these should not be conflated
- Word size (3 bytes/24 bits) is correct; only instruction encoding lengths need fixing
