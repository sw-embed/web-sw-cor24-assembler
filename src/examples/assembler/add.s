; Add: Compute 100 + 200 + 42 = 342
; Store result to memory at 256

        lc      r0,100      ; r0 = 100
        lcu     r1,200      ; r1 = 200 (unsigned, >127)
        add     r0,r1       ; r0 = 300
        lc      r1,42       ; r1 = 42
        add     r0,r1       ; r0 = 342

        la      r1,256      ; result address
        sw      r0,0(r1)    ; store 342 to memory

halt:
        bra     halt
