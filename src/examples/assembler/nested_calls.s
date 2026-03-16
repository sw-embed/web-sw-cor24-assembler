; Nested Calls: 3-level function call chain
; main -> level_a -> level_b, showing stack frames
; Result: r0 = ((5 + 10) * 2) + 3 = 33

        ; --- main ---
        lc      r0,5            ; arg = 5
        la      r1,ret_a        ; return address
        la      r2,level_a
        jal     r1,(r2)         ; call level_a(5)
ret_a:
halt:
        bra     halt            ; r0 = 33

        ; --- level_a(x): returns level_b(x + 10) ---
level_a:
        push    fp
        push    r1              ; save return addr
        mov     fp,sp
        add     r0,10           ; x + 10 = 15
        la      r1,ret_b
        la      r2,level_b
        jal     r1,(r2)         ; call level_b(15)
ret_b:
        mov     sp,fp
        pop     r1              ; restore return addr
        pop     fp
        jmp     (r1)            ; return

        ; --- level_b(x): returns x * 2 + 3 ---
level_b:
        push    fp
        push    r1              ; save return addr
        mov     fp,sp
        add     r0,r0           ; x * 2 = 30
        add     r0,3            ; + 3 = 33
        mov     sp,fp
        pop     r1
        pop     fp
        jmp     (r1)            ; return
