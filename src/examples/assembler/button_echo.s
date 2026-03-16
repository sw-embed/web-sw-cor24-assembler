; Button Echo: LED follows button state
; LED D2 lights when button S2 is pressed
; Click S2 button in I/O panel while running
;
; S2 is active-low (normally 1, pressed = 0)
; We invert with XOR so LED on = button pressed

        la      r1,-65536   ; I/O address (LEDSWDAT)
        lc      r2,1        ; Bit mask for XOR

loop:
        lb      r0,0(r1)    ; Read button S2 (bit 0: 1=released, 0=pressed)
        xor     r0,r2       ; Invert: pressed(0)->1(LED on), released(1)->0(LED off)
        sb      r0,0(r1)    ; Write to LED D2 (bit 0)

        bra     loop        ; Keep polling

halt:
        bra     halt        ; Never reached
