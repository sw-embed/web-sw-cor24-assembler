; Blink LED: Toggle LED D2 at ~1Hz
; Hover over D2 to see duty cycle %
; LED D2 at address -65536 (write bit 0)
;
; At default Run Speed 100/s, the delay
; loop of 16 iterations (48 instructions)
; gives ~1 blink per second.
; Edit the delay value to change the rate.

        la      r1,-65536

loop:
        lc      r0,1
        sb      r0,0(r1)    ; LED on

        ; On-time delay (~0.5s at 100/s)
        lc      r2,16
on_wait:
        add     r2,-1
        ceq     r2,z
        brf     on_wait

        lc      r0,0
        sb      r0,0(r1)    ; LED off

        ; Off-time delay (~0.5s at 100/s)
        lc      r2,16
off_wait:
        add     r2,-1
        ceq     r2,z
        brf     off_wait

        bra     loop
