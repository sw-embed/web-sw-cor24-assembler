; count_down.s - Count down from 5, print digits to UART
; Tests: loops, branches, arithmetic, UART output
; Expected output: "54321"

_main:
	push	fp
	mov	fp,sp
	la	r2,-65280	; 0xFF0100 UART data
	lc	r0,5		; counter = 5

_loop:
	; Convert counter to ASCII digit: r1 = r0 + '0'
	mov	r1,r0
	add	r1,48		; + '0' (0x30)
	sb	r1,0(r2)	; send to UART

	; Decrement counter
	add	r0,-1		; r0 -= 1

	; If r0 != 0, loop
	ceq	r0,z
	brf	_loop

	mov	sp,fp
	pop	fp
_halt:
	bra	_halt		; spin forever
