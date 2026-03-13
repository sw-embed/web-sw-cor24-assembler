; Countdown: Display 10 down to 0 on LED
; Writes count to LED register, delays, decrements

        la      r1,0xFF0000 ; LED address
        lc      r0,10       ; Start at 10

loop:   sb      r0,0(r1)    ; Write count to LED

        ; Delay loop
        push    r0
        lc      r2,0
wait:   add     r2,1
        lc      r0,127
        clu     r2,r0
        brt     wait
        pop     r0

        lc      r2,1
        sub     r0,r2       ; count--
        ceq     r0,z        ; count == 0?
        brf     loop        ; Continue if not zero

        ; Clear LED and halt
        lc      r0,0
        sb      r0,0(r1)
halt:   bra     halt
