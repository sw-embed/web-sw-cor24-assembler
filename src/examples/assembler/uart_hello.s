; UART Hello: Send "Hello\n" via UART
; UART data at 0xFF0100, status at 0xFF0101
; Poll TX busy (bit 7) before each byte

        lc      r0,72           ; 'H'
        la      r2,next1
        push    r2
        bra     putc
next1:  lc      r0,101          ; 'e'
        la      r2,next2
        push    r2
        bra     putc
next2:  lc      r0,108          ; 'l'
        la      r2,next3
        push    r2
        bra     putc
next3:  lc      r0,108          ; 'l'
        la      r2,next4
        push    r2
        bra     putc
next4:  lc      r0,111          ; 'o'
        la      r2,next5
        push    r2
        bra     putc
next5:  lc      r0,10           ; '\n'
        la      r2,halt
        push    r2
        bra     putc

halt:   bra     halt

; putc: send byte in r0, polling TX busy first
putc:   push    r0              ; save char
        la      r1,0xFF0100     ; UART base
.wait:  lb      r2,1(r1)        ; read status byte
        lcu     r0,128
        and     r2,r0           ; isolate bit 7
        ceq     r2,z
        brf     .wait           ; spin while TX busy
        pop     r0              ; restore char
        sb      r0,0(r1)        ; transmit byte
        pop     r2              ; return address
        jmp     (r2)
