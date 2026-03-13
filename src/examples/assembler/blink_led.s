; Blink LED: Toggle LED D2 on and off
; LED D2 at 0xFF0000 (write bit 0)
; Click Run to watch the LED blink!

        la      r1,0xFF0000

loop:
        lc      r0,1
        sb      r0,0(r1)

        push    r1
        lc      r1,10
delay1: lc      r2,0
wait1:  lc      r0,1
        add     r2,r0
        lc      r0,127
        clu     r2,r0
        brt     wait1
        lc      r0,1
        sub     r1,r0
        ceq     r1,z
        brf     delay1
        pop     r1

        lc      r0,0
        sb      r0,0(r1)

        push    r1
        lc      r1,10
delay2: lc      r2,0
wait2:  lc      r0,1
        add     r2,r0
        lc      r0,127
        clu     r2,r0
        brt     wait2
        lc      r0,1
        sub     r1,r0
        ceq     r1,z
        brf     delay2
        pop     r1

        bra     loop

halt:   bra     halt
