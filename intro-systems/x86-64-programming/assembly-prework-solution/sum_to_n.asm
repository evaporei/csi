section .text
global sum_to_n
sum_to_n:
	xor eax, eax
.loop:
	add eax, edi
	sub edi, 1
	jg .loop	
	ret
