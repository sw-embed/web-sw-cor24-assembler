; OOM: Fill SRAM with incrementing counter (256-byte stride)
; Writes one byte every 256 addresses from 256 to top of SRAM

        la   r1, 256        ; start address (past program code)
        lc   r0, 1          ; counter starts at 1

loop:
        sb   r0, 0(r1)      ; store counter byte
        lc   r2, 1
        add  r0, r2          ; counter++
        lcu  r2, 128
        add  r1, r2
        add  r1, r2          ; address += 256
        la   r2, 1048576     ; top of SRAM (1 MB)
        clu  r1, r2          ; address < top?
        brt  loop

halt:
        bra  halt
