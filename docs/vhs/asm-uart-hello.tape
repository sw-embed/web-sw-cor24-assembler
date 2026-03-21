# VHS tape: Assembler UART Hello — direct COR24 assembly
# Shows source, assembly listing, and emulator output
# Usage: vhs docs/vhs/asm-uart-hello.tape

Output docs/vhs/asm-uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

# --- Title ---
Type "# COR24 Assembler: UART Hello"
Enter
Type "# Polls TX busy (cls/brt) before each character"
Enter
Sleep 3s

# --- Show the assembly source ---
Type "clear && echo '=== COR24 Assembly Source ===' && cat src/examples/assembler/uart_hello.s"
Enter
Sleep 8s

# --- Assemble and run, capture output ---
Type "clear && echo '=== Assembling and Running ===' && rust-to-cor24/target/release/cor24-run --run src/examples/assembler/uart_hello.s --dump --trace 20 > /tmp/asm-demo.txt 2>&1 && echo 'Done.'"
Enter
Sleep 10s

# --- Show UART output ---
Type "clear && echo '=== UART Output ===' && grep -A 3 'UART:' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Show instruction trace (last 20) ---
Type "clear && echo '=== Instruction Trace (last 20) ===' && head -25 /tmp/asm-demo.txt"
Enter
Sleep 6s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A 8 '=== Registers ===' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Done! ===' && echo 'Source: uart_hello.s → rust-to-cor24/target/release/cor24-run → Hello on UART'"
Enter
Sleep 4s
