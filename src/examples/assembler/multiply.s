; Multiply: 6 × 7 = 42 via repeated addition
; Prints "42\n" to UART

        lc      r0,0            ; sum = 0
        lc      r1,7            ; counter = 7
loop:   add     r0,6            ; sum += 6
        push    r0              ; save sum
        lc      r0,1
        sub     r1,r0           ; counter--
        pop     r0              ; restore sum
        ceq     r1,z
        brf     loop            ; loop while counter != 0

        ; r0 = 42, divide by 10 (repeated subtraction)
        lc      r1,0            ; tens = 0
div10:  lc      r2,10
        clu     r0,r2           ; r0 < 10?
        brt     done            ; yes: r0=ones, r1=tens
        sub     r0,r2           ; r0 -= 10
        add     r1,1            ; tens++
        bra     div10
done:
        ; Print tens digit
        push    r0              ; save ones
        lc      r0,48           ; '0'
        add     r0,r1           ; r0 = '0' + tens
        la      r2,ret1
        push    r2
        bra     putc
ret1:
        ; Print ones digit
        pop     r0              ; restore ones
        lc      r1,48           ; '0'
        add     r0,r1           ; r0 = '0' + ones
        la      r2,ret2
        push    r2
        bra     putc
ret2:
        ; Print newline
        lc      r0,10           ; '\n'
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
