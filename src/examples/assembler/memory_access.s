; Memory Access: Store and load from non-adjacent regions
; Writes to 0x0100 and 0x0200 (256 bytes apart)
; Demonstrates memory viewer zero-row collapsing

        lc      r0,42       ; First value
        la      r1,0x0100   ; First address block

        ; Store to first block at 0x0100
        sb      r0,0(r1)    ; mem[0x0100] = 42
        sb      r0,1(r1)    ; mem[0x0101] = 42

        lcu     r0,200      ; Second value
        la      r1,0x0200   ; Second address block (256 bytes away)

        ; Store to second block at 0x0200
        sb      r0,0(r1)    ; mem[0x0200] = 200
        sw      r0,4(r1)    ; mem[0x0204..06] = 200

        ; Load them back to verify
        la      r1,0x0100
        lb      r2,0(r1)    ; r2 = 42
        la      r1,0x0200
        lw      r2,4(r1)    ; r2 = 200

halt:   bra     halt
