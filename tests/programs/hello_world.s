; hello_world.s - Print "Hello, World!\n" to UART using a loop
; UART data register at 0xFF0100, status at 0xFF0101 (bit 7 = TX busy)
; Polls TX busy before each write to avoid dropped characters.
; Expected UART output: "Hello, World!\n"

_main:
	push	fp
	mov	fp,sp
	la	r2,-65280	; r2 = 0xFF0100 UART data register
	la	r1,_string	; r1 = pointer to string

_loop:
	lb	r0,0(r1)	; load byte at *r1
	ceq	r0,z		; is it NUL terminator?
	brt	_done		; if zero, we're done
_tx_wait:
	push	r0		; save character
	lb	r0,1(r2)	; read UART status
	cls	r0,z		; bit 7 set = TX busy
	pop	r0		; restore character
	brt	_tx_wait	; spin until not busy
	sb	r0,0(r2)	; write byte to UART
	add	r1,1		; r1++ advance pointer
	bra	_loop		; next character

_done:
	mov	sp,fp
	pop	fp
_halt:
	bra	_halt		; spin forever

_string:
	.byte	72,101,108,108,111,44,32,87,111,114,108,100,33,10,0
	; H  e   l   l   o   ,  sp  W   o   r   l   d   !  \n NUL
