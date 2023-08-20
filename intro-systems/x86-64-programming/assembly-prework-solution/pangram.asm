section .text
global pangram
pangram:
	; rdi: source string
	xor edx, edx
.loop:
	movzx ecx, byte [rdi]	; read next char
	cmp ecx, 0
	je .end
	or ecx, 32				; force lower case (in ascii, 'a' + 32 -> 'A')
	sub ecx, 'a'
	bts edx, ecx			; set the corresponding bit in our bit set
	inc rdi
	jmp .loop
.end:
	xor eax, eax
	and edx, 0x03ffffff
	cmp edx, 0x03ffffff		; it's a pangram if low order 26 bits are set
	sete al
	ret
