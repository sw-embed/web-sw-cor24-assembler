# VHS tape: UART Hello — Rust → MSP430 → COR24 pipeline
# Shows each stage of the pipeline with pauses for narration
# Usage: vhs docs/vhs/uart-hello.tape

Output docs/vhs/uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

# --- Title ---
Type "# Rust → COR24 Pipeline: UART Hello"
Enter
Type "# uart_putc polls TX busy before each write"
Enter
Sleep 3s

# --- Show the Rust source ---
Type "echo '=== Rust Source ===' && head -30 rust-to-cor24/demos/demo_uart_hello/src/lib.rs"
Enter
Sleep 6s

# --- Run the pipeline, capture output ---
Type "cd rust-to-cor24/demos && bash demo_uart_hello/run.sh > /tmp/uart-demo.txt 2>&1 && cd ../.."
Enter
Sleep 25s

# --- Show MSP430 assembly (key functions) ---
Type "clear && echo '=== MSP430 Assembly (from rustc) ===' && grep -A 8 'uart_putc:' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Show COR24 assembly (translated) ---
Type "clear && echo '=== COR24 Assembly (translated) ===' && grep -A 20 'function: uart_putc' /tmp/uart-demo.txt"
Enter
Sleep 6s

# --- Show the UART output result ---
Type "clear && echo '=== Emulator Result ===' && grep -A 5 'UART:' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A 8 '=== Registers ===' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Done! UART output: Hello ===' && echo 'TX busy polling: cls r2,z / brt .wait'"
Enter
Sleep 4s
