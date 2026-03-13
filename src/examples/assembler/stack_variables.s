; Stack Variables: Local vars on the stack
; Demonstrates register spilling via push/pop
;
; Computes: a=seed+1, b=a+seed, c=b+a, result=a^b^c
; with seed=7: a=8, b=15, c=23, result=8^15^23=16

        lc      r0,7            ; seed = 7
        la      r1,ret_main
        la      r2,compute
        jal     r1,(r2)         ; call compute(7)
ret_main:
        ; r0 = result (16 = 0x10)
        la      r1,0xFF0000
        sb      r0,0(r1)        ; Display on LED
halt:   bra     halt

        ; --- compute(seed in r0) ---
        ; Uses r0-r2 for values, spills to stack when
        ; we run out of registers
compute:
        push    r1              ; spill return addr

        ; a = seed + 1
        mov     r1,r0           ; r1 = seed (keep copy)
        add     r0,1            ; r0 = a = 8

        ; b = a + seed
        mov     r2,r0           ; r2 = a (save)
        add     r0,r1           ; r0 = b = a + seed = 15

        ; c = b + a  (need a, but r2 has it)
        push    r0              ; spill b — out of regs
        add     r0,r2           ; r0 = c = b + a = 23

        ; result = a ^ b ^ c
        xor     r2,r0           ; r2 = a ^ c
        pop     r0              ; restore b
        xor     r2,r0           ; r2 = a ^ c ^ b = 16
        mov     r0,r2           ; r0 = result

        pop     r1              ; restore return addr
        jmp     (r1)
