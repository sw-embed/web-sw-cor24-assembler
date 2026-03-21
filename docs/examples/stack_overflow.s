; Stack Overflow: Infinite recursion filling the EBR/Stack region

        lc   r0, 0         ; recursion depth = 0
        bra  recurse

recurse:
        push r0             ; save depth on stack
        push r0             ; simulate a local variable
        lc   r2, 1
        add  r0, r2         ; depth++

        ; check if stack still in EBR region
        mov  r1, sp
        la   r2, -73728     ; bottom of EBR (0xFEE000 = -73728)
        clu  r2, r1         ; bottom < SP? (still room)
        brt  recurse        ; yes -> recurse deeper

halt:
        bra  halt           ; stack overflow - halt!
