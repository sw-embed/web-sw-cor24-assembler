; Memory Access: Store and load from non-adjacent regions
; Writes to 256 and 512 (256 bytes apart)
; Demonstrates memory viewer zero-row collapsing

        lc      r0,42       ; First value
        la      r1,256      ; First address block

        ; Store to first block at 256
        sb      r0,0(r1)    ; mem[256] = 42
        sb      r0,1(r1)    ; mem[257] = 42

        lcu     r0,200      ; Second value
        la      r1,512      ; Second address block (256 bytes away)

        ; Store to second block at 512
        sb      r0,0(r1)    ; mem[512] = 200
        sw      r0,4(r1)    ; mem[516..518] = 200

        ; Load them back to verify
        la      r1,256
        lb      r2,0(r1)    ; r2 = 42
        la      r1,512
        lw      r2,4(r1)    ; r2 = 200

halt:
        bra     halt
