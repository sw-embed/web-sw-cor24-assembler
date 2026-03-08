; led_on.s - Turn on LED D2
; LED I/O at 0xFF0000, bit 0 controls LED D2
; Expected: LED register = 0x01

_main:
	la	r0,-65536	; 0xFF0000 = LED I/O address
	lc	r1,1		; bit 0 = LED on
	sb	r1,0(r0)	; write to LED register
_halt:
	bra	_halt		; spin forever (test runner stops on cycle limit)
