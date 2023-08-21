default rel

section .text
global volume
volume:
	mulss xmm0, xmm0	; V = r * r
	mulss xmm0, xmm1	; V *= h
	mulss xmm0, [pi3]  	; V *= pi / 3
 	ret

section .rodata
pi3: dd 1.0471975512	; precalculate pi / 3 to save an instruction
