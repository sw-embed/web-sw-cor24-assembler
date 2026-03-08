; hello_uart.s - Print "Hi\n" to UART
; UART data at 0xFF0100 = -65280 signed
; Expected output: "Hi\n"

_main:
	push	fp
	mov	fp,sp
	la	r2,-65280	; 0xFF0100 UART data

	; Send 'H'
	lc	r0,72		; 'H' = 0x48
	sb	r0,0(r2)	; write to UART data

	; Send 'i'
	lc	r0,105		; 'i' = 0x69
	sb	r0,0(r2)

	; Send newline
	lc	r0,10		; '\n' = 0x0A
	sb	r0,0(r2)

	mov	sp,fp
	pop	fp
_halt:
	bra	_halt		; spin forever
