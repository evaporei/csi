section .text
global index
index:
	; rdi: matrix
	; rsi: rows
	; rdx: cols
	; rcx: rindex
	; r8: cindex
	imul rdx, rcx
	add rdx, r8	
	mov rax, [rdi + 4 * rdx]
	ret
