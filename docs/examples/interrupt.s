; COR24 Interrupt Example
; Main loop counts 0-9, ISR prints counter as ASCII digit via UART

; --- Setup interrupt vector ---
        la      r0, isr
        mov     iv, r0

; --- Enable UART RX interrupt ---
        la      r1, -65520      ; interrupt enable register
        lc      r0, 1
        sb      r0, 0(r1)

; --- Main loop: count 0-9 forever ---
        la      r1, 256         ; counter memory location
        lc      r0, 0
loop:
        sb      r0, 0(r1)       ; store counter
        lc      r2, 1
        add     r0, r2          ; counter++
        lc      r2, 10
        ceq     r0, r2          ; counter == 10?
        brf     loop
        lc      r0, 0           ; wrap to 0
        bra     loop

; --- Interrupt Service Routine ---
isr:
        push    r0
        push    r1
        push    r2
        mov     r2, c
        push    r2

        ; Read UART RX to acknowledge interrupt
        la      r1, -65280
        lb      r2, 0(r1)

        ; Read counter from memory
        la      r1, 256
        lb      r2, 0(r1)

        ; Convert to ASCII digit
        lc      r0, 48
        add     r0, r2

        ; Write to UART TX
        la      r1, -65280
        sb      r0, 0(r1)

        ; Restore registers
        pop     r2
        clu     z, r2
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)
