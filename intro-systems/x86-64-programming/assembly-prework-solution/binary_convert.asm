section .text
global binary_convert
binary_convert:
	xor eax, eax
.loop:
	movzx ecx, byte [rdi]	; retrieve next char
	shl eax, 1				; shift accumulator left to increase place value
	and ecx, 1				; only consider low order bit of ASCII char	
	add eax, ecx 			; add ascii value
	add rdi, 1 				; increment pointer
	cmp byte [rdi], 0
	jne .loop
	ret
