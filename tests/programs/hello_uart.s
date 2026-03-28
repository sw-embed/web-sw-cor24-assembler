; hello_uart.s - Print "Hi\n" to UART
; UART data at 0xFF0100, status at 0xFF0101 (bit 7 = TX busy)
; Polls TX busy before each write to avoid dropped characters.
; Expected output: "Hi\n"

_main:
	push	fp
	mov	fp,sp
	la	r2,-65280	; 0xFF0100 UART data

	; Send 'H'
	lc	r0,72		; 'H' = 0x48
_tx1:
	lb	r1,1(r2)	; read UART status
	cls	r1,z		; bit 7 set = TX busy
	brt	_tx1		; spin until not busy
	sb	r0,0(r2)	; write to UART data

	; Send 'i'
	lc	r0,105		; 'i' = 0x69
_tx2:
	lb	r1,1(r2)
	cls	r1,z
	brt	_tx2
	sb	r0,0(r2)

	; Send newline
	lc	r0,10		; '\n' = 0x0A
_tx3:
	lb	r1,1(r2)
	cls	r1,z
	brt	_tx3
	sb	r0,0(r2)

	mov	sp,fp
	pop	fp
_halt:
	bra	_halt		; spin forever
